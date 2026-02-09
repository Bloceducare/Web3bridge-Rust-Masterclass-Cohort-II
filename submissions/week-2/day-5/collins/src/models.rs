use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Expense {
    pub id: u8,
    pub name: String,
    pub amount: f64,
    pub tx_type: TransactionType,
}

impl fmt::Display for Expense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {} ({}, {})",
            self.id, self.name, self.amount, self.tx_type
        )
    }
}

#[derive(Debug, Clone)]
pub struct ExpenseTracker {
    pub values: HashMap<u8, Expense>,
    pub next_id: u8,
}
