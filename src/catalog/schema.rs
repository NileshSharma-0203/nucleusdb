use serde::{Deserialize, Serialize};

use crate::sql::ast::{ColumnDefinition, DataType, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub columns: Vec<ColumnDefinition>,
}

impl Schema {
    pub fn new(columns: Vec<ColumnDefinition>) -> Self {
        Self { columns }
    }

    pub fn column_names(&self) -> Vec<String> {
        self.columns
            .iter()
            .map(|column| column.name.clone())
            .collect()
    }

    pub fn validate_values(&self, values: &[Value]) -> Result<(), String> {
        if values.len() != self.columns.len() {
            return Err(format!(
                "Expected {} values, got {}",
                self.columns.len(),
                values.len()
            ));
        }

        for (value, column) in values.iter().zip(self.columns.iter()) {
            match (&column.data_type, value) {
                (DataType::Int, Value::Int(_)) => {}
                (DataType::Text, Value::Text(_)) => {}
                _ => {
                    return Err(format!("Type mismatch for column '{}'", column.name));
                }
            }
        }

        Ok(())
    }
}