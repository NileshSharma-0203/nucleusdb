#[derive(Debug)]
pub enum PlanNode {
    SeqScan {
        table: String,
    },

    IndexScan {
        table: String,
        column: String,
        value: i64,
    },
}