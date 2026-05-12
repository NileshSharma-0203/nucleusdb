use std::collections::{HashMap, VecDeque};

use crate::storage::disk_manager::DiskManager;
use crate::storage::page::{Page, PageId};

pub struct BufferPool {
    disk: DiskManager,
    cache: HashMap<PageId, Page>,
    lru: VecDeque<PageId>,
    capacity: usize,
}

impl BufferPool {
    pub fn new(path: &str, capacity: usize) -> Result<Self, String> {
        Ok(Self {
            disk: DiskManager::new(path)?,
            cache: HashMap::new(),
            lru: VecDeque::new(),
            capacity,
        })
    }

    pub fn fetch_page(&mut self, page_id: PageId) -> Result<&Page, String> {
        if !self.cache.contains_key(&page_id) {
            if self.cache.len() >= self.capacity {
                self.evict_page()?;
            }

            let page = self.disk.read_page(page_id)?;
            self.cache.insert(page_id, page);
        }

        self.touch_page(page_id);

        Ok(self.cache.get(&page_id).unwrap())
    }

    pub fn new_page(&mut self) -> Result<&mut Page, String> {
        if self.cache.len() >= self.capacity {
            self.evict_page()?;
        }

        let page = self.disk.allocate_page();
        let page_id = page.id;

        self.cache.insert(page_id, page);
        self.touch_page(page_id);

        Ok(self.cache.get_mut(&page_id).unwrap())
    }

    pub fn flush_page(&mut self, page_id: PageId) -> Result<(), String> {
        let page = self
            .cache
            .get(&page_id)
            .ok_or_else(|| format!("Page {} not in buffer pool", page_id))?;

        self.disk.write_page(page)
    }

    fn touch_page(&mut self, page_id: PageId) {
        self.lru.retain(|&id| id != page_id);
        self.lru.push_back(page_id);
    }

    fn evict_page(&mut self) -> Result<(), String> {
        let victim = self
            .lru
            .pop_front()
            .ok_or_else(|| "No page available for eviction".to_string())?;

        let page = self
            .cache
            .remove(&victim)
            .ok_or_else(|| "Victim page missing from cache".to_string())?;

        if page.dirty {
            self.disk.write_page(&page)?;
        }

        Ok(())
    }
}