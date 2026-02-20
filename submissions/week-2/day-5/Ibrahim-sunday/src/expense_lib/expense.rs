use chrono::{DateTime, Local};
use std::fs::File;
use std::io::Write;
use std::{collections::HashMap, io};

use super::commands_disp::display_commands;

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Credit,
    Debit,
}
#[derive(Debug, Clone)]
pub struct Expense {
    pub id: u8,
    pub name: String,
    pub amount: f64,
    pub tx_type: TransactionType,
    pub date: DateTime<Local>,
}

pub struct ExpenseTracker {
    pub values: HashMap<u8, Expense>,
    next_id: u8,
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
            date: Local::now(),
        };
        self.values.insert(current_id, new_expense.clone());
        self.next_id += 1;
        new_expense
    }

    pub fn view_one(&self, id: u8) -> Option<&Expense> {
        self.values.get(&id)
    }

    pub fn view_all(&self) -> Vec<&Expense> {
        self.values.values().collect()
    }

    pub fn view_all_debits(&self) -> Vec<&Expense> {
        self.values
            .values()
            .filter(|expense| expense.tx_type == TransactionType::Debit)
            .collect()
    }

    pub fn view_all_credit(&self) -> Vec<&Expense> {
        self.values
            .values()
            .filter(|expense| expense.tx_type == TransactionType::Credit)
            .collect()
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

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;

        writeln!(file, "===== EXPENSE TRACKER DATA =====")?;
        writeln!(file, "Total Expenses: {}\n", self.values.len())?;

        for expense in self.values.values() {
            writeln!(file, "ID: {}", expense.id)?;
            writeln!(file, "Name: {}", expense.name)?;
            writeln!(file, "Amount: ${:.2}", expense.amount)?;
            writeln!(file, "Type: {:?}", expense.tx_type)?;
            writeln!(file, "Date: {:?}", expense.date)?;
            writeln!(file, "---")?;
        }

        Ok(())
    }
}

pub fn expense_loop() {
    let mut expenses = ExpenseTracker::new();
    loop {
        display_commands();
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Command is needed");
        let parsed_cmd = command.trim();

        if parsed_cmd == "add" {
            let (new_expense_name, new_amount, tx_type) = get_expense_input();
            let new_expense = expenses.add(new_expense_name, new_amount, tx_type);
            println!("\n========================");
            println!("Added: {:?}", new_expense);
            println!("========================\n");
        } else if parsed_cmd == "view_expense" {
            let id = get_expense_id();
            let expense = expenses.view_one(id);

            match expense {
                Some(expense) => {
                    println!("\n====================");
                    println!("ID: {}", expense.id);
                    println!("Name: {}", expense.name);
                    println!("Amount: ${:.2}", expense.amount);
                    println!("Type: {:?}", expense.tx_type);
                    println!("Date: {:?}", expense.date);
                    println!("===========================\n");
                }
                None => {
                    println!("❌ Expense with ID {} not found.", id);
                }
            }
        } else if parsed_cmd == "view_all" {
            println!("\n========================");
            let all_expenses = expenses.view_all();
            if all_expenses.is_empty() {
                println!("No debit transactions found.");
            } else {
                for expense in &all_expenses {
                    println!(
                        "ID: {}, Name: {}, Amount: ${:.2}, Date: {:?}",
                        expense.id, expense.name, expense.amount, expense.date
                    );
                }
            }
            println!("==================================\n");
        } else if parsed_cmd == "view_all_credit" {
            println!("\n========================");
            let credits = expenses.view_all_credit();
            if credits.is_empty() {
                println!("No debit transactions found.");
            } else {
                for expense in &credits {
                    println!(
                        "ID: {}, Name: {}, Amount: ${:.2}, Date: {:?}",
                        expense.id, expense.name, expense.amount, expense.date
                    );
                }
                let total_debits: f64 = credits.iter().map(|e| e.amount).sum();
                println!("\nTotal credits: ${:.2}", total_debits);
            }
            println!("==================================\n");
        } else if parsed_cmd == "view_all_debit" {
            println!("\n========================");
            let debits = expenses.view_all_debits();
            if debits.is_empty() {
                println!("No debit transactions found.");
            } else {
                for expense in &debits {
                    println!(
                        "ID: {}, Name: {}, Amount: ${:.2}, Date: {:?}",
                        expense.id, expense.name, expense.amount, expense.date
                    );
                }
                let total_debits: f64 = debits.iter().map(|e| e.amount).sum();
                println!("\nTotal Debits: ${:.2}", total_debits);
            }
            println!("==================================\n");
        } else if parsed_cmd == "update" {
            println!("About to update an expense, kindly enter the details");

            let id = get_expense_id();

            let (new_amount, tx_type) = get_expense_update_input();

            let is_success = expenses.update(id, new_amount, tx_type);

            if is_success {
                println!("Succesfully updated expense with Id {}", id);
            } else {
                println!("An error occured when trying to update the id");
            }
        } else if parsed_cmd == "del" {
            println!("Delete an expense: Pass in the ID of the expense");

            //get id

            let id = get_expense_id();

            let is_success = expenses.delete(id);

            if is_success {
                println!("Successfully Deleted Expense with ID {}", id);
            } else {
                println!("Unknown error occurred when deleting")
            }
        } else if parsed_cmd == "q" {
            println!("\n========================");
            println!("Are you sure you want to quit this program? reply with y/n");
            let mut quit_prog_approval = String::new();
            io::stdin()
                .read_line(&mut quit_prog_approval)
                .expect("Expected y/n");

            if quit_prog_approval.trim() == "y" {
                println!("Program ended! File saved!");
                match expenses.save_to_file("expenses.txt") {
                    Ok(_) => println!("File saved: expenses.txt"),
                    Err(e) => println!(" Error saving file: {}", e),
                }
                println!("Program ended...");
                println!("========================\n");

                break;
            }
        }

        command.clear();
    }
}

fn get_expense_id() -> u8 {
    let mut id_input = String::new();
    io::stdin()
        .read_line(&mut id_input)
        .expect("Error taking expense Id");
    let id: u8 = id_input.trim().parse().expect("error parsing Id");
    id
}

fn get_expense_input() -> (String, f64, TransactionType) {
    // input 1
    println!("Enter transaction title:");
    let mut new_expense_name_input = String::new();
    io::stdin()
        .read_line(&mut new_expense_name_input)
        .expect("Failed to read type");
    let new_expense_name = new_expense_name_input.trim().to_string();

    // input 2
    let mut new_amount_input = String::new();
    println!("Enter amount transacted");
    io::stdin()
        .read_line(&mut new_amount_input)
        .expect("Failed to read type");
    let new_amount: f64 = new_amount_input
        .trim()
        .parse()
        .expect("An error occured parsing amount");

    // input 3
    let mut tx_type_input = String::new();
    println!("Enter type (debit/credit):");
    io::stdin()
        .read_line(&mut tx_type_input)
        .expect("Failed to read type");
    let tx_type = match tx_type_input.trim() {
        "debit" => TransactionType::Debit,
        "credit" => TransactionType::Credit,
        _ => {
            println!("Invalid type! Using Debit.");
            TransactionType::Debit
        }
    };

    (new_expense_name, new_amount, tx_type)
}

fn get_expense_update_input() -> (f64, TransactionType) {
    // input 1

    let mut new_amount_input = String::new();
    println!("Enter amount transacted");
    io::stdin()
        .read_line(&mut new_amount_input)
        .expect("Failed to read type");
    let new_amount: f64 = new_amount_input
        .trim()
        .parse()
        .expect("An error occured parsing amount");

    // input 2
    let mut tx_type_input = String::new();
    println!("Enter type (debit/credit):");
    io::stdin()
        .read_line(&mut tx_type_input)
        .expect("Failed to read type");
    let tx_type = match tx_type_input.trim() {
        "debit" => TransactionType::Debit,
        "credit" => TransactionType::Credit,
        _ => {
            println!("Invalid type! Using Debit.");
            TransactionType::Debit
        }
    };

    (new_amount, tx_type)
}
