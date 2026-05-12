use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::storage::page::{Page, PageId, PAGE_SIZE};

pub struct DiskManager {
    file: std::fs::File,
    next_page_id: PageId,
}

impl DiskManager {
    pub fn new(path: &str) -> Result<Self, String> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .map_err(|error| format!("Failed to open database file: {}", error))?;

        let file_size = file
            .metadata()
            .map_err(|error| format!("Failed to read file metadata: {}", error))?
            .len();

        let next_page_id = file_size / PAGE_SIZE as u64;

        Ok(Self { file, next_page_id })
    }

    pub fn allocate_page(&mut self) -> Page {
        let page_id = self.next_page_id;
        self.next_page_id += 1;

        Page::new(page_id)
    }

    pub fn write_page(&mut self, page: &Page) -> Result<(), String> {
        let offset = page.id * PAGE_SIZE as u64;

        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(|error| format!("Failed to seek page: {}", error))?;

        self.file
            .write_all(&page.data)
            .map_err(|error| format!("Failed to write page: {}", error))?;

        self.file
            .flush()
            .map_err(|error| format!("Failed to flush file: {}", error))?;

        Ok(())
    }

    pub fn read_page(&mut self, page_id: PageId) -> Result<Page, String> {
        let offset = page_id * PAGE_SIZE as u64;

        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(|error| format!("Failed to seek page: {}", error))?;

        let mut page = Page::new(page_id);

        self.file
            .read_exact(&mut page.data)
            .map_err(|error| format!("Failed to read page: {}", error))?;

        Ok(page)
    }
}