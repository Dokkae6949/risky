pub mod lock;
pub use lock::*;

pub mod bump;
pub use bump::*;

#[global_allocator]
pub static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

/// Initialize the kernel heap allocator.
pub unsafe fn init() {
    #[cfg(feature = "allocator_bump")]
    {
        let heap_start = crate::arch::consts::KERNEL_HEAP_OFFSET;
        let heap_size = crate::arch::consts::KERNEL_HEAP_SIZE;
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

/// Align the address `addr` upwards to alignment `align`.
/// # Safety
/// `align` must be a power of two.
/// # Logic
/// The following is a more readable version of the align_up function.
/// ```
/// let remainder = addr % align;
/// if remainder == 0 {
///     addr // addr already aligned
/// } else {
///     addr - remainder + align
/// }
/// ```
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

