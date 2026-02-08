use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Credit,
    Debit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: u8,
    pub name: String,
    pub amount: f64,
    pub tx_type: TransactionType,
}

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
        self.values.values().collect()
    }

    pub fn update(&mut self, id: u8, amount: f64, tx_type: TransactionType) -> Expense {
        let updated_data = self.values.get_mut(&id).expect("Failed to Update");
        updated_data.amount = amount;
        updated_data.tx_type = tx_type;
        updated_data.clone()
    }

    pub fn delete(&mut self, id: u8) -> bool {
        self.values.remove(&id).is_some()
    }

    pub fn save_to_file(&self, filename: &str) {
        let json = serde_json::to_string_pretty(&self.values).expect("Failed to serialize");
        let mut file = File::create(filename).expect("Failed to create file");
        file.write_all(json.as_bytes()).expect("Failed to write");

        println!("Data saved to {}", filename);
    }

    pub fn load_from_file(filename: &str) -> HashMap<u8, Expense> {
        if let Ok(mut file) = File::open(filename) {
            println!("\n-------------------------------\n");
            println!("Loading data from {}", filename);
            println!("\n-------------------------------\n");
            let mut content = String::new();
            file.read_to_string(&mut content).ok();
            println!("\n-------------------------------\n");
            println!("Data loaded from {}", filename);
            println!("\n-------------------------------\n");
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        }
    }
}
