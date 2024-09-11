use crate::arch::paging_sv39::Entry;

pub struct Table {
    pub entries: [Entry; 512],
}

impl Table {
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}