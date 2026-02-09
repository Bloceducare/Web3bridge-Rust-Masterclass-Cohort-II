mod expense_lib;

use expense_lib::{
    commands_disp::{display_commands, start_command},
    expense::expense_loop,
};

fn main() {
    println!("Welcome to RUSTY expense tracker!");
    display_commands();

    start_command();

    expense_loop();
}
