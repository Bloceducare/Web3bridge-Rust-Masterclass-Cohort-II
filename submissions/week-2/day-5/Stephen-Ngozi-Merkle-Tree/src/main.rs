use std::{collections::HashMap, io};

// Expense tracker
// Add the expenses
// Remove
// Update
// View

// Hashmaps
// structs
// enums
// Hashmaps

#[derive(Debug, Clone)]
enum TransactionType {
    Credit,
    Debit,
}
#[derive(Debug, Clone)]
struct Expense {
    id: u8,
    name: String,
    amount: f64,
    tx_type: TransactionType,
}

struct ExpenseTracker {
    values: HashMap<u8, Expense>,
    next_id: u8,
}

impl ExpenseTracker {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, name: String, amount: f64, tx_type: TransactionType) -> Expense {
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

    fn view_all(&self) -> Vec<&Expense> {
        self.values.values().collect()
    }

    fn update(&mut self, id: u8, amount: f64, tx_type: TransactionType) -> bool {
        match self.values.get_mut(&id) {
            Some(exp) => {
                exp.amount = amount;
                exp.tx_type = tx_type;
                true
            }
            None => false,
        }
        // let updated_expense = Expense {
        //     id,
        //     amount,
        //     tx_type,
        // };
        // self.values.put(id)
    }

    fn delete(&mut self, id: u8) -> bool {
        self.values.remove(&id).is_some()
    }
}
fn main() {
    println!("Hello, world!");
}
