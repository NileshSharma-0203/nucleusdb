use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum LockType {
    Shared,
    Exclusive,
}

#[derive(Debug)]
pub struct LockManager {
    locks: HashMap<String, Vec<LockType>>,
}

impl LockManager {
    pub fn new() -> Self {
        Self {
            locks: HashMap::new(),
        }
    }

    pub fn acquire_lock(
        &mut self,
        resource: &str,
        lock_type: LockType,
    ) -> Result<(), String> {
        let existing = self
            .locks
            .entry(resource.to_string())
            .or_insert(Vec::new());

        match lock_type {
            LockType::Shared => {
                if existing.contains(&LockType::Exclusive) {
                    return Err(format!(
                        "Cannot acquire shared lock on {}",
                        resource
                    ));
                }

                existing.push(LockType::Shared);
            }

            LockType::Exclusive => {
                if !existing.is_empty() {
                    return Err(format!(
                        "Cannot acquire exclusive lock on {}",
                        resource
                    ));
                }

                existing.push(LockType::Exclusive);
            }
        }

        Ok(())
    }

    pub fn release_locks(&mut self, resource: &str) {
        self.locks.remove(resource);
    }

    pub fn show_locks(&self) {
        println!("Current Locks:");

        for (resource, locks) in &self.locks {
            println!("{} -> {:?}", resource, locks);
        }
    }
}