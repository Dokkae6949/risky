#[repr(i64)]
#[derive(Debug, Clone, Copy)]
pub enum EntryBits {
    None = 0,
    Valid = 1 << 0,
    Read = 1 << 1,
    Write = 1 << 2,
    Execute = 1 << 3,
    User = 1 << 4,
    Global = 1 << 5,
    Accessed = 1 << 6,
    Dirty = 1 << 7,

    // Common combinations
    ReadWrite = 1 << 1 | 1 << 2,
    ReadExecute = 1 << 1 | 1 << 3,
    ReadWriteExecute = 1 << 1 | 1 << 2 | 1 << 3,

    UserReadWrite = 1 << 1 | 1 << 2 | 1 << 4,
    UserReadExecute = 1 << 1 | 1 << 3 | 1 << 4,
    UserReadWriteExecute = 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4,
}

impl EntryBits {
    pub(crate) fn bits(self) -> i64 {
        self as i64
    }
}

pub struct Entry(i64);

impl Entry {
    pub fn get(&self) -> i64 {
        self.0
    }

    pub fn set(&mut self, entry: i64) {
        self.0 = entry;
    }

    /// Check if the first bit (V bit) of the entry is set to 1.
    pub fn is_valid(&self) -> bool {
        self.get() & EntryBits::Valid.bits() != 0
    }

    /// Check if the first bit (V bit) of the entry is set to 0.
    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    /// Check if the entry is a leaf entry.
    /// A leaf entry is an entry that has
    /// any of it's R, W, or X bits set to 1.
    pub fn is_leaf(&self) -> bool {
        self.get() & EntryBits::ReadWriteExecute.bits() != 0
    }

    /// Check if the entry is a branch entry.
    /// A branch entry is an entry that has it's
    /// R, W, and X bits set to 0.
    pub fn is_branch(&self) -> bool {
        !self.is_leaf()
    }
}