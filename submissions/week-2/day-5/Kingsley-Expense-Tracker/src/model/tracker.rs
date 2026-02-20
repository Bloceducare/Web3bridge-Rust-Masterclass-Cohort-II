use super::expense::{Expense, TransactionType};
use std::collections::HashMap;

pub enum TrackerActions {
    ViewAll,
    Add,
    Edit,
    ViewSingle,
    Delete,
    Quit,
}

impl TrackerActions {
    pub fn get_power_action(action: &str) -> Option<TrackerActions> {
        match action.trim().to_lowercase().as_str() {
            "1" => Some(Self::ViewAll),
            "2" => Some(Self::Add),
            "3" => Some(Self::Edit),
            "4" => Some(Self::ViewSingle),
            "5" => Some(Self::Delete),
            "q" => Some(Self::Quit),
            _ => None,
        }
    }
}
pub struct ExpenseTracker {
    pub values: HashMap<u8, Expense>,
    pub next_key: u8,
}

impl ExpenseTracker {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            next_key: 1,
        }
    }

    pub fn add(&mut self, expense: &Expense) -> Result<Expense, String> {
        let new_expense = Expense {
            id: self.next_key,
            title: expense.title.clone(),
            amount: expense.amount,
            tx_type: expense.tx_type.clone(),
        };
        self.values.insert(self.next_key, new_expense.clone());
        self.next_key += 1;

        Ok(new_expense)
    }

    pub fn get_all_expenses(&self) -> Vec<&Expense> {
        self.values.values().collect()
    }

    pub fn update_item(
        &mut self,
        id: u8,
        amount: u32,
        tx_type: TransactionType,
    ) -> Result<&Expense, String> {
        // Read more on the relationship between match. and use of ok, ok_or, ok_else
        let item = self
            .values
            .get_mut(&id)
            .ok_or_else(|| format!("Expense not found"))?;

        item.amount = amount;
        item.tx_type = tx_type;
        Ok(item)
    }

    pub fn get_single_item(&self, id: u8) -> Option<&Expense> {
        self.values.get(&id)
    }

    pub fn delete_expense(&mut self, id: u8) -> Result<Expense, &str> {
        if let Some(item) = self.values.remove(&id) {
            Ok(item)
        } else {
            Err("An error occured deleting item")
        }
    }
}
