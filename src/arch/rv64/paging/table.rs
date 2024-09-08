use crate::arch::rv64::paging::entry::Entry;

pub struct Table {
    pub entries: [Entry; 512],
}

impl Table {
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}