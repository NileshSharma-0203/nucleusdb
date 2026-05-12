use crate::sql::ast::Value;

#[derive(Debug, Clone)]
pub struct RowVersion {
    pub values: Vec<Value>,
    pub created_by: u64,
    pub deleted_by: Option<u64>,
}

impl RowVersion {
    pub fn visible_to(&self, tx_id: u64) -> bool {
        if self.created_by > tx_id {
            return false;
        }

        match self.deleted_by {
            Some(deleted_tx) => deleted_tx > tx_id,
            None => true,
        }
    }
}