use std::collections::HashMap;

#[derive(Debug)]
pub struct Expense {
    id: u16,
    pub items: HashMap<String, f64>,
    pub total: f64,
}

#[derive(Debug)]
pub struct ExpenseTracker {
    expenses: Vec<Expense>,
    next_id: u16,
}

impl Expense {
    pub fn create(assigned_id: u16, items: HashMap<String, f64>) -> Self {
        let total = items.values().sum();
        Expense {
            id: assigned_id,
            items,
            total,
        }
    }

    fn sum_total(&mut self) {
        self.total = self.items.values().sum();
    }

    pub fn edit_items(&mut self, items: HashMap<String, f64>) {
        for (key, new_value) in items {
            self.items.insert(key, new_value);
        }
        self.sum_total();
    }

    pub fn remove_item(&mut self, item_name: &str) {
        self.items.remove(item_name);
        self.sum_total();
    }
}

impl ExpenseTracker {
    pub fn start() -> Self {
        ExpenseTracker {
            expenses: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_expense(&mut self, items: HashMap<String, f64>) {
        let expense = Expense::create(self.next_id, items);
        self.expenses.push(expense);
        self.next_id += 1;
    }

    pub fn edit_expense(&mut self, id: u16, items: HashMap<String, f64>) -> bool {
        if let Some(expense) = self.expenses.iter_mut().find(|e| e.id == id) {
            expense.edit_items(items);
            true
        } else {
            false
        }
    }

    pub fn remove_expense(&mut self, id: u16) -> bool {
        if let Some(pos) = self.expenses.iter().position(|e| e.id == id) {
            self.expenses.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_expenses(&self) -> &Vec<Expense> {
        &self.expenses
    }

    pub fn get_expense_by_id(&self, id: u16) -> Option<&Expense> {
        self.expenses.iter().find(|e| e.id == id)
    }

    pub fn get_total_expenses(&self) -> f64 {
        self.expenses.iter().map(|e| e.total).sum()
    }
}
