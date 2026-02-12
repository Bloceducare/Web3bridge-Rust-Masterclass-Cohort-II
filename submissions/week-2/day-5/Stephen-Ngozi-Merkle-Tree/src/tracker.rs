use std::{collections::HashMap,fs::File,io::Write};
use crate::expense::{Expense, TransactionType};


// Expense tracker
// Add the expenses
// Remove
// Update
// View

// Hashmaps
// structs
// enums
// Hashmaps




pub struct ExpenseTracker {
    pub values: HashMap<u8, Expense>,
    pub next_id: u8,
}

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
        let mut expenses: Vec<&Expense> = self.values.values().collect();

        expenses.sort_by(|a, b|{a.id.cmp(&b.id)});
        expenses
        
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
        // let updated_expense = Expense {
        //     id,
        //     amount,
        //     tx_type,
        // };
        // self.values.put(id)
    }

    pub fn delete(&mut self, id: u8) -> bool {
        self.values.remove(&id).is_some()
    }


    pub fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Could not create file");
        
        for expense in self.values.values() {
            let line = format!("ID: {}, Name: {}, Amount: {:.2}, Type: {:?}\n", 
                            expense.id, expense.name, expense.amount, expense.tx_type);
            file.write_all(line.as_bytes()).expect("Could not write to file");
        }
    }
}