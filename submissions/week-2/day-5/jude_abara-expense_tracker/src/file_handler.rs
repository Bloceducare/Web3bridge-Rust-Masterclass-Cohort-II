use crate::expense::{/* Expense, */ ExpenseTracker, TransactionType};
use std::fs::OpenOptions;
use std::io::{self, Write};

pub struct FileHandler {
    filename: String,
}

impl FileHandler {
    /// Creates a new FileHandler with the specified filename
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
        }
    }

    /// Saves all expenses to the file
    pub fn save_expenses(&self, tracker: &ExpenseTracker) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.filename)?;

        writeln!(
            file,
            "═══════════════════════════════════════════════════════════"
        )?;
        writeln!(
            file,
            "                    EXPENSE TRACKER REPORT                  "
        )?;
        writeln!(
            file,
            "═══════════════════════════════════════════════════════════"
        )?;
        writeln!(file)?;

        let expenses = tracker.view_all();

        if expenses.is_empty() {
            writeln!(file, "No transactions recorded.")?;
        } else {
            let mut total_credit = 0.0;
            let mut total_debit = 0.0;

            for expense in &expenses {
                writeln!(
                    file,
                    "───────────────────────────────────────────────────────────"
                )?;
                writeln!(file, "ID:          {}", expense.id)?;
                writeln!(file, "Description: {}", expense.description)?;
                writeln!(file, "Amount:      ${:.2}", expense.amount)?;
                writeln!(file, "Type:        {:?}", expense.tx_type)?;

                match expense.tx_type {
                    TransactionType::Credit => total_credit += expense.amount,
                    TransactionType::Debit => total_debit += expense.amount,
                }
            }

            writeln!(
                file,
                "───────────────────────────────────────────────────────────"
            )?;
            writeln!(file)?;
            writeln!(file, "SUMMARY:")?;
            writeln!(file, "  Total Transactions: {}", expenses.len())?;
            writeln!(file, "  Total Credit:       ${:.2}", total_credit)?;
            writeln!(file, "  Total Debit:        ${:.2}", total_debit)?;
            writeln!(
                file,
                "  Net Balance:        ${:.2}",
                total_credit - total_debit
            )?;
        }

        writeln!(file)?;
        writeln!(
            file,
            "═══════════════════════════════════════════════════════════"
        )?;
        writeln!(
            file,
            "Report generated: {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        )?;

        Ok(())
    }

    // /// Appends a single expense to the file (for logging individual operations)
    // pub fn log_operation(&self, operation: &str, expense: &Expense) -> io::Result<()> {
    //     let mut file = OpenOptions::new()
    //         .create(true)
    //         .append(true)
    //         .open(format!("{}.log", self.filename))?;

    //     writeln!(
    //         file,
    //         "[{}] {} - ID: {}, Description: {}, Amount: ${:.2}, Type: {:?}",
    //         chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
    //         operation,
    //         expense.id,
    //         expense.description,
    //         expense.amount,
    //         expense.tx_type
    //     )?;

    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_handler_creation() {
        let handler = FileHandler::new("test_expenses.txt");
        assert_eq!(handler.filename, "test_expenses.txt");
    }
}
