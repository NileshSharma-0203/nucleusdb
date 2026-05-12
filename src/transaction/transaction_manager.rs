use crate::sql::ast::Statement;

pub struct TransactionManager {
    active: bool,
    log: Vec<Statement>,
}

impl TransactionManager {
    pub fn new() -> Self {
        Self {
            active: false,
            log: Vec::new(),
        }
    }

    pub fn begin(&mut self) -> Result<String, String> {
        if self.active {
            return Err("Transaction already active".to_string());
        }

        self.active = true;
        self.log.clear();

        Ok("Transaction started".to_string())
    }

    pub fn record(&mut self, statement: Statement) {
        if self.active {
            self.log.push(statement);
        }
    }

    pub fn commit(&mut self) -> Result<Vec<Statement>, String> {
        if !self.active {
            return Err("No active transaction".to_string());
        }

        self.active = false;

        let statements = self.log.clone();

        self.log.clear();

        Ok(statements)
    }

    pub fn rollback(&mut self) -> Result<String, String> {
        if !self.active {
            return Err("No active transaction".to_string());
        }

        self.active = false;
        self.log.clear();

        Ok("Transaction rolled back".to_string())
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}