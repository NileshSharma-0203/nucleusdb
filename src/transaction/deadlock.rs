use std::collections::{HashMap, HashSet};

pub struct DeadlockDetector {
    waits_for: HashMap<u64, Vec<u64>>,
}

impl DeadlockDetector {
    pub fn new() -> Self {
        Self {
            waits_for: HashMap::new(),
        }
    }

    pub fn add_wait(&mut self, from_tx: u64, to_tx: u64) {
        self.waits_for
            .entry(from_tx)
            .or_insert(Vec::new())
            .push(to_tx);
    }

    pub fn detect_deadlock(&self) -> bool {
        let mut visited = HashSet::new();

        let mut stack = HashSet::new();

        for &tx in self.waits_for.keys() {
            if self.has_cycle(tx, &mut visited, &mut stack) {
                return true;
            }
        }

        false
    }

    fn has_cycle(
        &self,
        tx: u64,
        visited: &mut HashSet<u64>,
        stack: &mut HashSet<u64>,
    ) -> bool {
        if stack.contains(&tx) {
            return true;
        }

        if visited.contains(&tx) {
            return false;
        }

        visited.insert(tx);
        stack.insert(tx);

        if let Some(neighbors) = self.waits_for.get(&tx) {
            for &neighbor in neighbors {
                if self.has_cycle(neighbor, visited, stack) {
                    return true;
                }
            }
        }

        stack.remove(&tx);

        false
    }

    pub fn show_graph(&self) {
        println!("Wait-For Graph:");

        for (tx, waits) in &self.waits_for {
            println!("T{} -> {:?}", tx, waits);
        }
    }
}