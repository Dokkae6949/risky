use crate::allocator::align_up;
use crate::arch::consts::get_page_align;
use crate::arch::paging_sv39::{Entry, EntryBits, Table};
use crate::arch::rv64::memory::page_allocator::{dealloc, zalloc};

/// Map a virtual address to a physical address.
/// # Safety
/// This function will fail if the provided level is greater than 2.
///
/// ## Flags
/// The provided flags are what will be set in the entry.
/// The flags can be a combination of the following:
/// - [`EntryBits::Read`]
/// - [`EntryBits::Write`]
/// - [`EntryBits::Execute`]
/// - [`EntryBits::User`]
/// - [`EntryBits::Global`]
///
/// The flags must have at least one of the R, W, or X bits set.
/// The Valid bit is set automatically.
pub fn map(root: &mut Table, vaddr: usize, paddr: usize, flags: i64, level: usize) {
    // Make sure that either the R, W, or X bit is set.
    assert_ne!(flags & EntryBits::ReadWriteExecute.bits(), 0);
    assert!(level < 3);

    // Extract the VPN (Virtual Page Number) from the virtual address.
    // Each VPN is 9 bits long. Thus, we use 0x1ff (0b1_1111_1111) to mask.
    let vpn = [
        // VPN[0] = vaddr[20:12]
        (vaddr >> 12) & 0x1ff,
        // VPN[1] = vaddr[29:21]
        (vaddr >> 21) & 0x1ff,
        // VPN[2] = vaddr[38:30]
        (vaddr >> 30) & 0x1ff,
    ];

    // Extract the PPN (Physical Page Number) from the physical address.
    // Each PPN except for the last one is 9 bits long. Thus, we use 0x1ff (0b1_1111_1111) to mask.
    // The last PPN is 26 bits long. Thus, we use 0x3ff_ffff (0b11_1111_1111_1111_1111_1111_1111) to mask.
    let ppn = [
        // PPN[0] = paddr[20:12]
        (paddr >> 12) & 0x1ff,
        // PPN[1] = paddr[29:21]
        (paddr >> 21) & 0x1ff,
        // PPN[2] = paddr[55:30]
        (paddr >> 30) & 0x3ff_ffff,
    ];

    let mut v = &mut root.entries[vpn[2]];

    // Now, we're going to traverse the page table and set the bits
    // properly. We expect the root to be valid, however we're required to
    // create anything beyond the root.
    // In Rust, we create a range iterator using the .. operator.
    // The .rev() will reverse the iteration since we need to start with
    // VPN[2] The .. operator is inclusive on start but exclusive on end.
    // So, (0..2) will iterate 0 and 1.
    for i in (level..2).rev() {
        if v.is_invalid() {
            let page = zalloc(1).expect("out of memory");
            // The page is already aligned by 4,096, so store it
            // directly The page is stored in the entry shifted
            // right by 2 places.
            v.set((page as i64 >> 2) | EntryBits::Valid.bits());
        }

        let entry = ((v.get() & !0x3ff) << 2) as *mut Entry;
        v = unsafe { entry.add(vpn[i]).as_mut().expect("entry is null") };
    }

    let entry =
        (ppn[2] << 28) as i64 | // PPN[2] = [53:28]
            (ppn[1] << 19) as i64 | // PPN[1] = [27:19]
            (ppn[0] << 10) as i64 | // PPN[0] = [18:10]
            flags |                 // Specified flags such as R, W, X, U, G
            EntryBits::Valid.bits();// Valid bit
    v.set(entry);
}

/// Unmaps a table and deallocates the associated memory.
/// Note that root itself is not deallocated.
pub fn unmap(root: &mut Table) {
    // Start at level 2
    for lv2 in 0..root.len() {
        let ref entry_lv2 = root.entries[lv2];
        if entry_lv2.is_valid() && entry_lv2.is_branch() {
            let memaddr_lv1 = (entry_lv2.get() & !0x3ff) << 2;
            let table_lv1 = unsafe {
                (memaddr_lv1 as *mut Table).as_mut().expect("table_lv1 is null")
            };

            for lv1 in 0..root.len() {
                let ref entry_lv1 = table_lv1.entries[lv1];
                if entry_lv1.is_valid() && entry_lv1.is_branch() {
                    let memaddr_lv0 = (entry_lv1.get() & !0x3ff) << 2;
                    // The next level is level 0, which
                    // cannot have branches, therefore,
                    // we free here.
                    dealloc(memaddr_lv0 as *mut u8);
                }
            }

            dealloc(memaddr_lv1 as *mut u8);
        }
    }
}

/// Translate a virtual address to a physical address.
/// Walk the page table to convert a virtual address to a
/// physical address.
/// If a page fault would occur, this returns None
/// Otherwise, it returns Some with the physical address.
pub fn virt_to_phys(root: &Table, vaddr: usize) -> Option<usize> {
    // Extract the VPN (Virtual Page Number) from the virtual address.
    // Each VPN is 9 bits long. Thus, we use 0x1ff (0b1_1111_1111) to mask.
    let vpn = [
        // VPN[0] = vaddr[20:12]
        (vaddr >> 12) & 0x1ff,
        // VPN[1] = vaddr[29:21]
        (vaddr >> 21) & 0x1ff,
        // VPN[2] = vaddr[38:30]
        (vaddr >> 30) & 0x1ff,
    ];

    let mut v = &root.entries[vpn[2]];

    for i in (0..=2).rev() {
        if v.is_invalid() {
            break;
        }
        else if v.is_leaf() {
            let off_mask = (1 << (12 + 9 * i)) - 1;
            let vaddr_pgoff = vaddr & off_mask;
            let addr = ((v.get() << 2) as usize) & !off_mask;
            return Some(addr | vaddr_pgoff);
        }

        // Set v to the next entry which is pointed to by
        // this entry. However, we the address is shifted right
        // by 2 bits when stored in the page table entry. So
        // we need to shift left by 2 bits to get the usable address.
        let entry = ((v.get() & !0x3ff) << 2) as *mut Entry;
        v = unsafe { entry.add(vpn[i - 1]).as_ref().expect("entry is null") };
    }

    None
}


/// Identity map range
/// Takes a contiguous allocation of memory and maps it using PAGE_SIZE
/// This assumes that start <= end
pub fn id_map_range(
    root: &mut Table,
    start: usize,
    end: usize,
    bits: i64
) {
    assert!(start <= end, "start must be less than or equal to end");

    let mut memaddr = start & !(get_page_align() - 1);
    let num_kb_pages = (align_up(end, get_page_align())
        - memaddr)
        / get_page_align();

    // I named this num_kb_pages for future expansion when
    // I decide to allow for GiB (2^30) and 2MiB (2^21) page
    // sizes. However, the overlapping memory regions are causing
    // nightmares.
    for _ in 0..num_kb_pages {
        map(root, memaddr, memaddr, bits, 0);
        memaddr += 1 << 12;
    }
}