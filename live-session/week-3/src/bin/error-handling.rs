use std::{fs::File, io::ErrorKind};

fn main() {
    // let our_vector = vec![1, 2, 3, 4, 5];
    // our_vector[50];

    // let greeting_file_result =
    //     File::open("hello.txt").unwrap_or(File::create_new("hello.txt").unwrap());

    let greeting_file = File::open("hello.txt");
    let greeting = File::open("hello.txt");
    let greeting1 = File::open("hello.txt");

    match greeting_file {
        Ok(result) => result,
        Err(e) => return println!("Error couldn't open file: {}", e),
    };

    let f = match greeting {
        Ok(file) => file,
        Err(e) => panic!("Error couldn't open file: {}", e),
    };

    match greeting1 {
        Ok(result) => result,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Error creating file {e}"),
            },
            _ => panic!("An error occured"),
        },
    };
}
