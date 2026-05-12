use crate::execution::plan::PlanNode;

pub struct Optimizer;

impl Optimizer {
    pub fn choose_plan(
        table: &str,
        total_rows: usize,
        has_index: bool,
        column: &str,
        value: i64,
    ) -> PlanNode {
        let seq_scan_cost = total_rows as f64;

        let index_scan_cost = if has_index {
            (total_rows as f64).log2()
        } else {
            f64::INFINITY
        };

        println!("SeqScan cost: {}", seq_scan_cost);
        println!("IndexScan cost: {}", index_scan_cost);

        if index_scan_cost < seq_scan_cost {
            PlanNode::IndexScan {
                table: table.to_string(),
                column: column.to_string(),
                value,
            }
        } else {
            PlanNode::SeqScan {
                table: table.to_string(),
            }
        }
    }
}