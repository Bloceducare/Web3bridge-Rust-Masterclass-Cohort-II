use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;

const DATA_FILE: &str = "expenses.json";

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum TransactionType {
    Credit,
    Debit,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::Credit => write!(f, "Credit"),
            TransactionType::Debit => write!(f, "Debit"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Expense {
    pub id: u32,
    pub name: String,
    pub amount: f64,
    pub tx_type: TransactionType,
}

#[derive(Serialize, Deserialize)]
pub struct ExpenseTracker {
    records: HashMap<u32, Expense>,
    next_id: u32,
}

impl ExpenseTracker {
    pub fn load() -> Self {
        if let Ok(data) = fs::read_to_string(DATA_FILE) {
            if let Ok(tracker) = serde_json::from_str(&data) {
                return tracker;
            }
        }
        Self {
            records: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn save(&self) {
        let json = serde_json::to_string_pretty(self).expect("Failed to serialize data");
        fs::write(DATA_FILE, json).expect("Unable to write file");
    }

    pub fn add(&mut self, name: String, amount: f64, tx_type: TransactionType) -> Expense {
        let expense = Expense {
            id: self.next_id,
            name,
            amount,
            tx_type,
        };
        self.records.insert(expense.id, expense.clone());
        self.next_id += 1;
        self.save();
        expense
    }

    pub fn delete(&mut self, id: u32) -> Option<Expense> {
        let removed = self.records.remove(&id);
        if removed.is_some() {
            self.save();
        }
        removed
    }

    pub fn update(
        &mut self,
        id: u32,
        name: Option<String>,
        amount: Option<f64>,
        tx_type: Option<TransactionType>,
    ) -> Option<Expense> {
        if let Some(exp) = self.records.get_mut(&id) {
            if let Some(n) = name {
                exp.name = n;
            }
            if let Some(a) = amount {
                exp.amount = a;
            }
            if let Some(t) = tx_type {
                exp.tx_type = t;
            }
            let updated = exp.clone();
            self.save();
            return Some(updated);
        }
        None
    }

    pub fn get_expense(&self, id: u32) -> Option<&Expense> {
        self.records.get(&id)
    }

    pub fn get_all_expenses(&self) -> Vec<&Expense> {
        let mut expenses: Vec<&Expense> = self.records.values().collect();
        expenses.sort_by_key(|e| e.id);
        expenses
    }
}
