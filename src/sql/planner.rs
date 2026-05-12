use crate::execution::plan::PlanNode;
use crate::sql::ast::{Expression, Statement, Value};

pub struct Planner;

impl Planner {
    pub fn build(statement: &Statement) -> PlanNode {
        match statement {
            Statement::Explain { statement } => {
                Self::build(statement)
            }
            Statement::Select { table, filter } => {
                match filter {
                    Some(Expression::Equals { column, value }) => {
                        if column == "id" {
                            if let Value::Int(n) = value {
                                return PlanNode::IndexScan {
                                    table: table.clone(),
                                    column: column.clone(),
                                    value: *n,
                                };
                            }
                        }

                        PlanNode::SeqScan {
                            table: table.clone(),
                        }
                    }

                    None => PlanNode::SeqScan {
                        table: table.clone(),
                    },
                }
            }

            _ => PlanNode::SeqScan {
                table: "unknown".to_string(),
            },
        }
    }
}