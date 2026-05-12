use crate::sql::ast::Value;
use crate::storage::disk_manager::DiskManager;
use crate::storage::record::RecordSerializer;
use crate::storage::record_id::RecordId;

pub struct HeapFile {
    disk: DiskManager,
}

impl HeapFile {
    pub fn new(path: &str) -> Result<Self, String> {
        Ok(Self {
            disk: DiskManager::new(path)?,
        })
    }

    pub fn insert_record(&mut self, values: &[Value]) -> Result<RecordId, String> {
        let serialized = RecordSerializer::serialize(values)?;

        let mut page = self.disk.allocate_page();

        let record_count: u32 = 1;

        page.write_bytes(0, &record_count.to_le_bytes())?;

        let record_size = serialized.len() as u32;

        page.write_bytes(4, &record_size.to_le_bytes())?;

        page.write_bytes(8, &serialized)?;

        let page_id = page.id;

        self.disk.write_page(&page)?;

        Ok(RecordId::new(page_id, 0))
    }

    pub fn read_record(&mut self, page_id: u64) -> Result<Vec<Value>, String> {
        let page = self.disk.read_page(page_id)?;

        let size_bytes = page.read_bytes(4, 4)?;

        let mut size_array = [0u8; 4];

        size_array.copy_from_slice(size_bytes);

        let record_size = u32::from_le_bytes(size_array) as usize;

        let record_bytes = page.read_bytes(8, record_size)?;

        RecordSerializer::deserialize(record_bytes)
    }
}