use std::fs::OpenOptions;
use std::io::Write;
use std::fs;
use crate::sql::ast::Statement;

pub struct Wal {
    path: String,
}

impl Wal {
    pub fn read_entries(&self) -> Result<Vec<String>, String> {
        match fs::read_to_string(&self.path) {
            Ok(contents) => {
                Ok(contents.lines().map(|s| s.to_string()).collect())
            }
    
            Err(_) => Ok(Vec::new()),
        }
    }
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn append(&self, entry: &str) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|error| format!("Failed to open WAL: {}", error))?;

        writeln!(file, "{}", entry)
            .map_err(|error| format!("Failed to write WAL: {}", error))?;

        file.flush()
            .map_err(|error| format!("Failed to flush WAL: {}", error))?;

        Ok(())
    }

    pub fn log_statement(&self, statement: &Statement) -> Result<(), String> {
        self.append(&format!("{:?}", statement))
    }
}