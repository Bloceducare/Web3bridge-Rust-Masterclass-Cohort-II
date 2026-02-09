use crate::expenses::Expense;
use std::fs::File;
use std::io::{Result, Write};

const REPORT_FILE: &str = "report.txt";

pub fn write_to_report(expenses: &Vec<Expense>, total: f64) -> Result<()> {
    let mut file = File::create(REPORT_FILE)?;
    for (index, expense) in expenses.iter().enumerate() {
        let items_str = expense
            .items
            .iter()
            .map(|(item, price)| format!("{item}: ₦{price:.2}"))
            .collect::<Vec<String>>()
            .join(", ");
        writeln!(
            file,
            "Expense {}: Total: ₦{:.2} --- Items: {}",
            index + 1,
            expense.total,
            items_str
        )?;
    }
    writeln!(file, "\nTotal Expenses: ₦{:.2}", total)?;
    Ok(())
}
