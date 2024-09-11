use core::alloc::{GlobalAlloc, Layout};
use crate::allocator::Locked;
use crate::arch::rv64::memory::kernel_allocator::{kfree, kzmalloc};

pub mod page_allocator;
pub mod kernel_allocator;
pub mod page;
pub mod alloc_list;

struct KernelGlobalAlloc;

unsafe impl GlobalAlloc for Locked<KernelGlobalAlloc> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let _ = self.lock();

        kzmalloc(layout.size()).expect("out of memory")
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let _ = self.lock();

        kfree(ptr);
    }
}

#[global_allocator]
static KERNEL_GLOBAL_ALLOC: Locked<KernelGlobalAlloc> = Locked::new(KernelGlobalAlloc);

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}