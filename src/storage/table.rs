use serde::{Deserialize, Serialize};

use crate::catalog::schema::Schema;
use crate::sql::ast::{Expression, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    pub values: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub schema: Schema,
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new(name: String, schema: Schema) -> Self {
        Self {
            name,
            schema,
            rows: Vec::new(),
        }
    }

    pub fn insert(&mut self, values: Vec<Value>) -> Result<(), String> {
        self.schema.validate_values(&values)?;

        self.rows.push(Row { values });

        Ok(())
    }

    pub fn select(&self, filter: Option<&Expression>) -> Result<String, String> {
        let mut output = String::new();

        output.push_str(&format!("Table: {}\n", self.name));

        let column_names = self.schema.column_names();

        output.push_str(&column_names.join(" | "));
        output.push('\n');

        output.push_str(
            &column_names
                .iter()
                .map(|name| "-".repeat(name.len()))
                .collect::<Vec<_>>()
                .join("-+-"),
        );

        output.push('\n');

        for row in &self.rows {
            if self.row_matches_filter(row, filter)? {
                let values = row.values.iter().map(format_value).collect::<Vec<_>>();

                output.push_str(&values.join(" | "));
                output.push('\n');
            }
        }

        Ok(output)
    }

    fn row_matches_filter(
        &self,
        row: &Row,
        filter: Option<&Expression>,
    ) -> Result<bool, String> {
        match filter {
            None => Ok(true),

            Some(Expression::Equals { column, value }) => {
                let column_index = self
                    .schema
                    .columns
                    .iter()
                    .position(|col| col.name == *column)
                    .ok_or_else(|| format!("Column '{}' does not exist", column))?;

                Ok(row.values[column_index] == *value)
            }
        }
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::Int(n) => n.to_string(),
        Value::Text(s) => s.clone(),
    }
}