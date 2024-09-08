#[repr(u8)]
pub enum PageBits {
    Empty = 0,
    Taken = 1 << 0,
    Last = 1 << 1,

    // Common combinations
    TakenLast = 1 << 0 | 1 << 1,
}

impl PageBits {
    pub fn bits(self) -> u8 {
        self as u8
    }
}

pub struct Page {
    flags: u8,
}

impl Page {
    /// Check if the first bit (Taken bit) of the page is set to 1.
    pub fn is_last(&self) -> bool {
        self.flags & PageBits::Last.bits() != 0
    }

    /// Check if the first bit (Taken bit) of the page is set to 1.
    pub fn is_taken(&self) -> bool {
        self.flags & PageBits::Taken.bits() != 0
    }

    /// Check if the first bit (Taken bit) of the page is set to 0.
    pub fn is_free(&self) -> bool {
        !self.is_taken()
    }

    /// Clears all flags of the page.
    pub fn clear(&mut self) {
        self.flags = PageBits::Empty.bits();
    }

    /// Sets the specified flags of the page.
    pub fn set_flags(&mut self, flags: PageBits) {
        self.flags |= flags.bits();
    }

    /// Clears the specified flags of the page.
    pub fn clear_flags(&mut self, flags: PageBits) {
        self.flags &= !flags.bits();
    }
}