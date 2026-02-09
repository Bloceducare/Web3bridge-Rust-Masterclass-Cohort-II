use std::collections::HashMap;

use crate::models::{Expense, ExpenseTracker, TransactionType};

impl ExpenseTracker {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, name: String, amount: f64, tx_type: TransactionType) -> Expense {
        let current_id = self.next_id;
        let new_expense = Expense {
            id: current_id,
            name,
            amount,
            tx_type,
        };
        self.values.insert(current_id, new_expense.clone());
        self.next_id += 1;
        new_expense
    }

    pub fn view_all(&self) -> Vec<&Expense> {
        self.values.values().collect()
    }

    pub fn view_all_owned(&self) -> Vec<Expense> {
        self.values.values().cloned().collect()
    }

    pub fn update(&mut self, id: u8, amount: f64, tx_type: TransactionType) -> bool {
        match self.values.get_mut(&id) {
            Some(exp) => {
                exp.amount = amount;
                exp.tx_type = tx_type;
                true
            }
            None => false,
        }
    }

    pub fn delete(&mut self, id: u8) -> bool {
        self.values.remove(&id).is_some()
    }
}
