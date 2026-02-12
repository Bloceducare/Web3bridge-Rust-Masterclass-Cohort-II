mod modules;
use std::io;

use crate::modules::{ExpenseTracker, UserOptions};

// Task : Create an expense tracker that can do the following: 

// - Add the expenses
// - Remove
// - Update
// - View Expenses

// Make use of the following data types: 

// - Hashmaps
// - structs
// - enums


fn main() {
    let mut tracker1 = ExpenseTracker::new();

    loop {
        let user_choice = ExpenseTracker::get_user_choice();
        tracker1.execute_user_choice(Option::expect(user_choice.as_ref(), "failed"));

        if user_choice == Some(UserOptions::Exit) {

            println!("Are you sure you want to exit?\nY/N");

            let mut exit_choice = String::new();
            io::stdin().read_line(&mut exit_choice);
            let exit_choice = exit_choice.trim();

            if exit_choice == 'y'.to_string() || exit_choice == 'Y'.to_string() {   
                // let view = tracker1.view_all();

                // tracker1.write_to_file();
                break;
            }
        }
    }
}
