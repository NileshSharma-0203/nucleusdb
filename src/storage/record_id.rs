#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecordId {
    pub page_id: u64,
    pub slot_id: u16,
}

impl RecordId {
    pub fn new(page_id: u64, slot_id: u16) -> Self {
        Self { page_id, slot_id }
    }
}