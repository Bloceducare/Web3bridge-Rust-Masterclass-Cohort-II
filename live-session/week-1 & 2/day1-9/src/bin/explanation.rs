fn main() {
    // NOTE:
    // 1. Every  value has an owner.
    // 2. There can only be an owner to  a value at a time.
    // 3. Once the owner goes out of scope the value is dropped.

    let mut greeting = String::from("Hello Abel");

    println!("==== cap, len, ptr=========");
    println!(
        "Initials {} {} {:?}",
        greeting.len(),
        greeting.capacity(),
        greeting.as_ptr()
    );
    println!("Before pop print: {}", &greeting);

    greeting.push_str(", How are you?");

    println!("==== cap, len, ptr after push=========");
    println!(
        "Before pop: {} {} {:?}",
        greeting.len(),
        greeting.capacity(),
        greeting.as_ptr()
    );

    println!("Before pop: {greeting}");

    let r = greeting.pop();

    println!("==== cap, len, ptr after pop=========");
    println!(
        "Before pop: {} {} {:?}",
        greeting.len(),
        greeting.capacity(),
        greeting.as_ptr()
    );

    if let Some(value) = r {
        println!("Value: {value}")
    }

    println!("{:?}", r.unwrap());
    println!("After pop:{greeting}");

    println!("================================");

    let hello = String::from("Hello");
    println!("{hello}");

    // println!("{hello}"); // Diagnostics: 1. cannot find value `hello` in this scope [E0425]
}
