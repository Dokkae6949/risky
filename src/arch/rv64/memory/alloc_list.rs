#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum AllocListFlags {
    Taken = 1 << 63,
}

impl AllocListFlags {
    pub fn bits(self) -> usize {
        self as usize
    }
}

/// A structure representing a node in the allocation list.
/// The allocation list knows where the next node is located
/// and whether the current node is taken or free because
/// it stores the taken bit in the most significant bit of the size field
/// and the size of the allocation in the remaining bits.
///
/// Meaning the next node is located at the address of the current node
/// plus the size of the current node.
#[derive(Debug, Clone, Copy)]
pub struct AllocList {
    flags_size: usize,
}

impl AllocList {
    pub fn is_taken(&self) -> bool {
        self.flags_size & AllocListFlags::Taken.bits() != 0
    }

    pub fn is_free(&self) -> bool {
        !self.is_taken()
    }

    pub fn set_taken(&mut self) {
        self.flags_size |= AllocListFlags::Taken.bits();
    }

    pub fn set_free(&mut self) {
        self.flags_size &= !AllocListFlags::Taken.bits();
    }

    pub fn size(&self) -> usize {
        self.flags_size & !AllocListFlags::Taken.bits()
    }

    pub fn set_size(&mut self, size: usize) {
        let k = self.is_taken();
        self.flags_size = size & !AllocListFlags::Taken.bits();
        if k {
            self.flags_size |= AllocListFlags::Taken.bits();
        }
    }
}