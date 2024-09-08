mod table;
mod entry;

pub use table::*;
pub use entry::*;

use crate::arch::rv64::memory::page_allocator::{dealloc, zalloc};

/// Map a virtual address to a physical address.
/// # Safety
/// This function will fail if the provided level is greater than 4.
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
    assert!(level < 5);

    // Extract the VPN (Virtual Page Number) from the virtual address.
    // Each VPN is 9 bits long. Thus, we use 0x1ff (0b1_1111_1111) to mask.
    let vpn = [
        // VPN[0] = vaddr[20:12]
        (vaddr >> 12) & 0x1ff,
        // VPN[1] = vaddr[29:21]
        (vaddr >> 21) & 0x1ff,
        // VPN[2] = vaddr[38:30]
        (vaddr >> 30) & 0x1ff,
        // VPN[3] = vaddr[47:39]
        (vaddr >> 39) & 0x1ff,
        // VPN[4] = vaddr[56:48]
        (vaddr >> 48) & 0x1ff,
    ];

    // Extract the PPN (Physical Page Number) from the physical address.
    // Each PPN except for the last one is 9 bits long. Thus, we use 0x1ff (0b1_1111_1111) to mask.
    // The last PPN is 8 bits long. Thus, we use 0xff (0b1111_1111) to mask.
    let ppn = [
        // PPN[0] = paddr[20:12]
        (paddr >> 12) & 0x1ff,
        // PPN[1] = paddr[29:21]
        (paddr >> 21) & 0x1ff,
        // PPN[2] = paddr[38:30]
        (paddr >> 30) & 0x1ff,
        // PPN[3] = paddr[47:39]
        (paddr >> 39) & 0x1ff,
        // PPN[4] = paddr[55:48]
        (paddr >> 48) & 0xff,
    ];

    let mut v = &mut root.entries[vpn[4]];

    for i in (level..4).rev() {
        if v.is_invalid() {
            let page = zalloc(1).expect("out of memory");
            v.set((page as i64 >> 2) | EntryBits::Valid.bits());
        }

        let entry = ((v.get() & !0x3ff) << 2) as *mut Entry;
        v = unsafe { entry.add(vpn[i]).as_mut().expect("entry is null") };
    }

    let entry = (ppn[4] << 46) as i64 | // PPN[4] = [53:46]
                (ppn[3] << 37) as i64 | // PPN[3] = [45:37]
                (ppn[2] << 28) as i64 | // PPN[2] = [36:28]
                (ppn[1] << 19) as i64 | // PPN[1] = [27:19]
                (ppn[0] << 10) as i64 | // PPN[0] = [18:10]
                flags |                 // Specified flags such as R, W, X, U, G
                EntryBits::Valid.bits();// Valid bit
    v.set(entry);
}

/// Unmap a table.
pub fn unmap(root: &mut Table) {
    // Start at level 5
    for lv4 in 0..root.len() {
        let ref entry_lv4 = root.entries[lv4];
        if entry_lv4.is_valid() && entry_lv4.is_branch() {
            let memaddr_lv3 = (entry_lv4.get() & !0x3ff) << 2;
            let table_lv3 = unsafe {
                (memaddr_lv3 as *mut Table).as_mut().expect("table_lv4 is null")
            };

            for lv3 in 0..root.len() {
                let ref entry_lv3 = table_lv3.entries[lv4];
                if entry_lv3.is_valid() && entry_lv3.is_branch() {
                    let memaddr_lv2 = (entry_lv3.get() & !0x3ff) << 2;
                    let table_lv2 = unsafe {
                        (memaddr_lv2 as *mut Table).as_mut().expect("table_lv3 is null")
                    };

                    for lv2 in 0..root.len() {
                        let ref entry_lv2 = table_lv2.entries[lv2];
                        if entry_lv2.is_valid() && entry_lv2.is_branch() {
                            let memaddr_lv1 = (entry_lv2.get() & !0x3ff) << 2;
                            let table_lv1 = unsafe {
                                (memaddr_lv1 as *mut Table).as_mut().expect("table_lv2 is null")
                            };

                            for lv1 in 0..root.len() {
                                let ref entry_lv1 = table_lv1.entries[lv1];
                                if entry_lv1.is_valid() && entry_lv1.is_branch() {
                                    let memaddr_lv0 = (entry_lv1.get() & !0x3ff) << 2;
                                    let table_lv0 = unsafe {
                                        (memaddr_lv0 as *mut Table).as_mut().expect("table_lv1 is null")
                                    };

                                    for lv0 in 0..root.len() {
                                        let ref entry_lv0 = table_lv0.entries[lv0];
                                        if entry_lv0.is_valid() && entry_lv0.is_branch() {
                                            let memaddr_lv0 = (entry_lv0.get() & !0x3ff) << 2;
                                            // Since this is the last level, we can deallocate the page.
                                            dealloc(memaddr_lv0 as *mut u8);
                                        }
                                    }

                                    dealloc(memaddr_lv1 as *mut u8);
                                }
                            }

                            dealloc(memaddr_lv2 as *mut u8);
                        }
                    }

                    dealloc(memaddr_lv3 as *mut u8);
                }
            }
        }
    }
}

/// Translate a virtual address to a physical address.
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
        // VPN[3] = vaddr[47:39]
        (vaddr >> 39) & 0x1ff,
        // VPN[4] = vaddr[56:48]
        (vaddr >> 48) & 0x1ff,
    ];

    let mut v = &root.entries[vpn[4]];

    for i in (0..=4).rev() {
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