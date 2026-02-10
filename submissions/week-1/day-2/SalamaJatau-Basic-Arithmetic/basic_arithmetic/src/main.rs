fn main() {
    println!("Addition: {}", addition(10,20));
    println!("Subtraction: {}", subtraction(20,10));
    println!("Multiplication: {}", multiplication(10,20));
    println!("Division: {}", division(20,2));
}

fn addition(first_number: i32, second_number: i32) -> i32{
    first_number + second_number
}

fn subtraction(first_number: i32, second_number: i32) -> i32{
    first_number - second_number
}

fn multiplication(first_number: i32, second_number: i32) -> i32{
    first_number * second_number
}

fn division(first_number: i32, second_number: i32) -> i32{
    first_number / second_number
}
