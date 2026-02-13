# Expense Tracker

A simple CLI expense tracker built in Rust. It supports adding, viewing, updating, and removing expenses, and persists data to a JSON file between runs.

## Assignment Requirements Checklist

- Prints out operations a user can carry out.
- Program keeps running after reading, adding, updating, or deleting.
- User can quit the program with 'q' and is prompted y/n.
- After updating or adding, the user can see the values they just added.
- All interactions are written to a file (saved to JSON).
- Abstracted into modules.

## How It Works

- Startup loads existing data from `expenses.json`.
- Each expense has an `id`, `name`, `amount`, and `tx_type` (Credit/Debit).
- Data is saved back to `expenses.json` after add/update/delete and on quit.

## Run

```bash
cargo run
```

## Project Structure

- `src/main.rs`: application entry point and menu loop.
- `src/menu.rs`: menu options and selection logic.
- `src/modules.rs`: feature modules (add/view/update/delete/quit).
- `src/tracker.rs`: core tracker data model and persistence.
- `expenses.json`: persisted data file.

## Usage Tips

- Enter the menu number (1-4) or name (`add`, `view`, `update`, `remove`), or `q` to quit.
- On quit, confirm with `y` or `n`.
