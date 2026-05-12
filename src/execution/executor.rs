use std::collections::HashMap;
use std::fs;

use serde::{Deserialize, Serialize};

use crate::catalog::schema::Schema;
use crate::sql::ast::Statement;
use crate::storage::table::Table;

const DATABASE_FILE: &str = "nucleus.db";

#[derive(Debug, Serialize, Deserialize)]
pub struct Executor {
    tables: HashMap<String, Table>,
}

impl Executor {
    pub fn new() -> Self {
        match fs::read_to_string(DATABASE_FILE) {
            Ok(contents) => {
                if contents.trim().is_empty() {
                    return Self {
                        tables: HashMap::new(),
                    };
                }

                match serde_json::from_str::<Executor>(&contents) {
                    Ok(executor) => executor,
                    Err(_) => Self {
                        tables: HashMap::new(),
                    },
                }
            }
            Err(_) => Self {
                tables: HashMap::new(),
            },
        }
    }

    pub fn execute(&mut self, statement: Statement) -> Result<String, String> {
        match statement {
            Statement::Begin | Statement::Commit | Statement::Rollback => {
                Ok("Transaction command handled by TransactionManager.".to_string())
            }
            Statement::Explain { .. } => {
                Ok("EXPLAIN is handled by the planner.".to_string())
            }
            Statement::CreateTable { table, columns } => {
                if self.tables.contains_key(&table) {
                    return Err(format!("Table '{}' already exists", table));
                }

                let schema = Schema::new(columns);
                let table_obj = Table::new(table.clone(), schema);

                self.tables.insert(table.clone(), table_obj);
                self.save()?;

                Ok(format!("Table '{}' created.", table))
            }

            Statement::Insert { table, values } => {
                let table_obj = self
                    .tables
                    .get_mut(&table)
                    .ok_or_else(|| format!("Table '{}' does not exist", table))?;

                table_obj.insert(values)?;
                self.save()?;

                Ok("1 row inserted.".to_string())
            }

            Statement::Select { table, filter } => {
                let table_obj = self
                    .tables
                    .get(&table)
                    .ok_or_else(|| format!("Table '{}' does not exist", table))?;

                table_obj.select(filter.as_ref())
            }
        }
    }

    fn save(&self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|error| format!("Failed to serialize database: {}", error))?;

        fs::write(DATABASE_FILE, json)
            .map_err(|error| format!("Failed to write database file: {}", error))?;

        Ok(())
    }
}