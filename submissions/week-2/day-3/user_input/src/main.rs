use std::io;

//list of allowed input
#[derive(Debug)]
enum PowerOptions {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

//data to print when the correct power option is inputted
fn power_output(power_option: PowerOptions) {
    match power_option {
        PowerOptions::Off => println!("Off"),
        PowerOptions::Sleep => println!("Sleeping"),
        PowerOptions::Reboot => println!("Rebooting"),
        PowerOptions::Shutdown => println!("Shutting down"),
        PowerOptions::Hibernate => println!("Hibernating"),
    }
}

//accepts user input and convert it to lowercase
fn user_input() -> String {

    println!("Please input power option.");

    let mut new_user_input = String::new();

    io::stdin()
        .read_line(&mut new_user_input)
        .expect("Failed to read line");

    new_user_input.trim().to_lowercase()

}



//loop through user input and display the correct power option
//when a wrong option is inputted, invalid input should be displayed
fn main() {

    loop {

        let lowercase_user_input = user_input();

        let new_user_input = match lowercase_user_input.as_str() {
            "off" => PowerOptions::Off,
            "sleep" => PowerOptions::Sleep,
            "reboot" => PowerOptions::Reboot,
            "shutdown" => PowerOptions::Shutdown,
            "hibernate" => PowerOptions::Hibernate,
            _ => {
                println!("Invalid input");
                continue;
            }
        };

        power_output(new_user_input);
    }

}
