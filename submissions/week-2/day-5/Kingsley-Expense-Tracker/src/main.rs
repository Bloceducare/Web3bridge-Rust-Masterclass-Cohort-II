mod model;
mod logger_module;
mod ui;

use model::{ExpenseTracker, TrackerActions};
use logger_module as logger;
use std::io;
use ui::display_actions;
use ui::start;

fn main() {
    let mut tracker = ExpenseTracker::new();
    println!(
        r#"
  _____                                      
 | ____|_  ___ __   ___ _ __  ___  ___   
 |  _| \ \/ / '_ \ / _ \ '_ \/ __|/ _ \  
 | |___ >  <| |_) |  __/ | | \__ \  __/  
 |_____/_/\_\ .__/ \___|_| |_|___/\___|  
            |_|                          
       T R A C K E R
    "#
    );

    println!(
        r#"
Welcome to Expense Tracker v1.0!
---------------------------------
Manage your income and expenses efficiently.
Usage: Select an option from the menu below to get started.
All data is saved automatically in memmory.
"#
    );

    loop {
        display_actions();

        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Please enter a valid character");

        let action = TrackerActions::get_power_action(&action);

        if let Some(act) = action {
            start(&act, &mut tracker)
        } else {
            println!("Failed to process command. Please enter a valid input")
        }
    }
}
