use std::collections::BTreeMap;

use crate::storage::record_id::RecordId;

pub struct BPlusTree {
    map: BTreeMap<i64, RecordId>,
}

impl BPlusTree {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: i64, rid: RecordId) {
        self.map.insert(key, rid);
    }

    pub fn search(&self, key: i64) -> Option<&RecordId> {
        self.map.get(&key)
    }

    pub fn range_search(
        &self,
        start: i64,
        end: i64,
    ) -> Vec<(i64, RecordId)> {
        self.map
            .range(start..=end)
            .map(|(k, v)| (*k, *v))
            .collect()
    }
}