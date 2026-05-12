pub const PAGE_SIZE: usize = 4096;

pub type PageId = u64;

#[derive(Debug, Clone)]
pub struct Page {
    pub id: PageId,
    pub data: [u8; PAGE_SIZE],
    pub dirty: bool,
}

impl Page {
    pub fn new(id: PageId) -> Self {
        Self {
            id,
            data: [0; PAGE_SIZE],
            dirty: false,
        }
    }

    pub fn write_bytes(&mut self, offset: usize, bytes: &[u8]) -> Result<(), String> {
        let end = offset + bytes.len();

        if end > PAGE_SIZE {
            return Err(format!(
                "Write out of bounds: offset={}, len={}, page_size={}",
                offset,
                bytes.len(),
                PAGE_SIZE
            ));
        }

        self.data[offset..end].copy_from_slice(bytes);
        self.dirty = true;

        Ok(())
    }

    pub fn read_bytes(&self, offset: usize, length: usize) -> Result<&[u8], String> {
        let end = offset + length;

        if end > PAGE_SIZE {
            return Err(format!(
                "Read out of bounds: offset={}, len={}, page_size={}",
                offset, length, PAGE_SIZE
            ));
        }

        Ok(&self.data[offset..end])
    }
}