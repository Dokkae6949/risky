#[repr(C)]
#[derive(Clone, Copy)]
pub struct TrapFrame {
    pub regs:       [usize; 32], // 0 - 255
    pub fregs:      [usize; 32], // 256 - 511
    pub satp:       usize,       // 512 - 519
    pub trap_stack: *mut u8,     // 520
    pub hartid:     usize,       // 528
}

impl TrapFrame {
    pub fn new() -> Self {
        Self {
            regs:       [0; 32],
            fregs:      [0; 32],
            satp:       0,
            trap_stack: core::ptr::null_mut(),
            hartid:     0,
        }
    }
}