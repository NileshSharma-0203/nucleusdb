pub struct RecoveryManager;

impl RecoveryManager {
    pub fn recover(entries: &[String]) -> Vec<Vec<String>> {
        let mut committed_transactions = Vec::new();

        let mut current_transaction = Vec::new();

        let mut active = false;

        for entry in entries {
            match entry.as_str() {
                "BEGIN" => {
                    active = true;
                    current_transaction.clear();
                }

                "COMMIT" => {
                    if active {
                        committed_transactions.push(current_transaction.clone());
                    }

                    active = false;
                }

                "ROLLBACK" => {
                    active = false;
                    current_transaction.clear();
                }

                other => {
                    if active {
                        current_transaction.push(other.to_string());
                    }
                }
            }
        }

        committed_transactions
    }
}