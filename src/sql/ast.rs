use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Explain {
        statement: Box<Statement>,
    },
    Begin,
    Commit,
    Rollback,
    Select {
        table: String,
        filter: Option<Expression>,
    },

    CreateTable {
        table: String,
        columns: Vec<ColumnDefinition>,
    },

    Insert {
        table: String,
        values: Vec<Value>,
    },
    
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Equals {
        column: String,
        value: Value,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
    Int,
    Text,
    Bool,
    Float,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Int(i64),
    Text(String),
}