pub mod lock;
pub use lock::*;

#[cfg(feature = "allocator_bump")]
pub mod bump;
#[cfg(feature = "allocator_bump")]
pub use bump::*;
#[cfg(feature = "allocator_bump")]
#[cfg(not(feature = "allocator_fixed_size_block"))]
#[global_allocator]
pub static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

#[cfg(feature = "allocator_fixed_size_block")]
pub mod fixed_sized_block;
#[cfg(feature = "allocator_fixed_size_block")]
pub use fixed_sized_block::*;
#[cfg(feature = "allocator_fixed_size_block")]
#[global_allocator]
pub static KERNEL_HEAP_ALLOCATOR: Locked<FixedSizedBlockAllocator> = Locked::new(FixedSizedBlockAllocator::new());

/// The kernel heap.
/// This is a fixed size heap of 128 KiB.
pub static KERNEL_HEAP: [u8; 0x0001_0000] = [0; 0x0001_0000];

/// Initialize the kernel heap allocator.
pub unsafe fn init() {
    #[cfg(feature = "allocator_bump")]
    #[cfg(feature = "allocator_fixed_size_block")]
    {
        let heap_start = KERNEL_HEAP.as_ptr() as usize;
        let heap_size = KERNEL_HEAP.len();
        KERNEL_HEAP_ALLOCATOR.lock().init(heap_start, heap_size);
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
    assert!(align.is_power_of_two());
    (addr + align - 1) & !(align - 1)
}