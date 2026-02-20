use crate::models::TransactionType;
use std::io::{self, Write};

use crate::models::Expense;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table, presets::UTF8_FULL};

pub fn print(s: &str) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}

pub fn read_input(prompt: &str, is_optional: bool) -> Option<String> {
    print(prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let trimmed = input.trim().to_string();

    if is_optional && trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

pub fn read_f64(prompt: &str, is_optional: bool) -> Option<f64> {
    loop {
        let input = read_input(prompt, is_optional);
        match input {
            None => return None,
            Some(s) if s.is_empty() => println!("Please enter a valid amount."),
            Some(s) => match s.parse::<f64>() {
                Ok(n) => return Some(n),
                Err(_) => println!("Invalid number. Please enter a valid amount."),
            },
        }
    }
}

pub fn read_u32(prompt: &str) -> u32 {
    loop {
        let input = read_input(prompt, false).unwrap();
        match input.parse::<u32>() {
            Ok(n) => return n,
            Err(_) => println!("Invalid ID. Please enter a valid positive number."),
        }
    }
}

pub fn read_transaction_type(is_optional: bool) -> Option<TransactionType> {
    loop {
        let input = read_input("Type (c = Credit, d = Debit): ", is_optional);
        match input {
            None => return None,
            Some(s) => match s.as_str() {
                "c" | "C" | "credit" | "Credit" => return Some(TransactionType::Credit),
                "d" | "D" | "debit" | "Debit" => return Some(TransactionType::Debit),
                _ => println!("Invalid input. Please enter 'c' for Credit or 'd' for Debit."),
            },
        }
    }
}

pub fn print_expense(exp: &Expense) {
    let color = match exp.tx_type {
        TransactionType::Credit => Color::Green,
        TransactionType::Debit => Color::Red,
    };
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);
    table.add_row(vec![
        Cell::new(exp.id),
        Cell::new(&exp.name).add_attribute(Attribute::Bold),
        Cell::new(format!("${:.2}", exp.amount)),
        Cell::new(format!("{}", exp.tx_type)).fg(color),
    ]);
    println!("{table}");
}

pub fn print_expenses_table(expenses: &[&Expense]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["ID", "Name", "Amount", "Type"]);

    let mut total_credit: f64 = 0.0;
    let mut total_debit: f64 = 0.0;

    for exp in expenses {
        let color = match exp.tx_type {
            TransactionType::Credit => {
                total_credit += exp.amount;
                Color::Green
            }
            TransactionType::Debit => {
                total_debit += exp.amount;
                Color::Red
            }
        };

        table.add_row(vec![
            Cell::new(exp.id),
            Cell::new(&exp.name).add_attribute(Attribute::Bold),
            Cell::new(format!("${:.2}", exp.amount)),
            Cell::new(format!("{}", exp.tx_type)).fg(color),
        ]);
    }
    println!("{table}");

    let net = total_credit - total_debit;
    println!("Total Credit: +${:.2}", total_credit);
    println!("Total Debit:  -${:.2}", total_debit);
    println!("Net Balance:  {:+.2}", net);
}
