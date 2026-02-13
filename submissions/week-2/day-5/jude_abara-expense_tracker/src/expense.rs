use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Credit,
    Debit,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Credit => write!(f, "Credit"),
            TransactionType::Debit => write!(f, "Debit"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expense {
    pub id: u8,
    pub description: String,
    pub amount: f64,
    pub tx_type: TransactionType,
}

#[derive(Debug)]
pub struct ExpenseTracker {
    values: HashMap<u8, Expense>,
    next_id: u8,
}

impl ExpenseTracker {
    /// Creates a new empty ExpenseTracker
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            next_id: 1,
        }
    }

    /// Adds a new expense and returns the created expense
    pub fn add(&mut self, description: String, amount: f64, tx_type: TransactionType) -> Expense {
        let current_id = self.next_id;
        let new_expense = Expense {
            id: current_id,
            description,
            amount,
            tx_type,
        };

        self.values.insert(current_id, new_expense.clone());
        self.next_id += 1;

        new_expense
    }

    /// Returns all expenses as a vector of references
    pub fn view_all(&self) -> Vec<&Expense> {
        let mut expenses: Vec<&Expense> = self.values.values().collect();
        expenses.sort_by_key(|e| e.id);
        expenses
    }

    /// Updates an existing expense by ID
    pub fn update(&mut self, id: u8, expense: Expense) -> bool {
        match self.values.get_mut(&id) {
            Some(exp) => {
                exp.description = expense.description;
                exp.amount = expense.amount;
                exp.tx_type = expense.tx_type;
                true
            }
            None => false,
        }
    }

    /// Deletes an expense by ID and returns true if successful
    pub fn delete(&mut self, id: u8) -> bool {
        self.values.remove(&id).is_some()
    }

    /// Checks if an expense with the given ID exists
    pub fn exists(&self, id: u8) -> bool {
        self.values.contains_key(&id)
    }

    /// Gets a reference to an expense by ID
    pub fn get(&self, id: u8) -> Option<&Expense> {
        self.values.get(&id)
    }

    // /// Returns the number of expenses
    // pub fn count(&self) -> usize {
    //     self.values.len()
    // }
}

impl Default for ExpenseTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_expense() {
        let mut tracker = ExpenseTracker::new();
        let expense = tracker.add("Groceries".to_string(), 50.0, TransactionType::Debit);

        assert_eq!(expense.id, 1);
        assert_eq!(expense.description, "Groceries");
        assert_eq!(expense.amount, 50.0);
    }

    #[test]
    fn test_view_all() {
        let mut tracker = ExpenseTracker::new();
        tracker.add("Item 1".to_string(), 10.0, TransactionType::Credit);
        tracker.add("Item 2".to_string(), 20.0, TransactionType::Debit);

        let expenses = tracker.view_all();
        assert_eq!(expenses.len(), 2);
    }

    #[test]
    fn test_update_expense() {
        let mut tracker = ExpenseTracker::new();
        let expense = tracker.add("Original".to_string(), 100.0, TransactionType::Credit);

        let updated = Expense {
            id: expense.id,
            description: "Updated".to_string(),
            amount: 150.0,
            tx_type: TransactionType::Debit,
        };

        assert!(tracker.update(expense.id, updated));
        let result = tracker.get(expense.id).unwrap();
        assert_eq!(result.description, "Updated");
        assert_eq!(result.amount, 150.0);
    }

    #[test]
    fn test_delete_expense() {
        let mut tracker = ExpenseTracker::new();
        let expense = tracker.add("To Delete".to_string(), 75.0, TransactionType::Debit);

        assert!(tracker.delete(expense.id));
        assert!(!tracker.exists(expense.id));
    }
}
