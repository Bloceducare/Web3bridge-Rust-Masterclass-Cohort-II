use std::collections::HashMap;
// common collections
//  string, vec, hashmaps

fn main() {
    let names = vec![String::from("Victory the silent speaker")];
    println!("Value of our vec is {names:?}");

    let mut new_map: HashMap<String, i32> = HashMap::new();
    let abel = String::from("abel");
    new_map.insert(abel.clone(), 20);
    let value = new_map.get(&abel);

    // account_number -> balance
    //  key               value

    println!("value: {:?}", value.unwrap());

    let new_sting = String::from("Hello");
    let o = new_sting.chars().nth(4);

    // this will fail
    // let string_fail = String::from("hello");
    // string_fail[0];
    // this will fail why?
    // You cannot index string like arrays so
    // you basically have to use the chars().nth()

    match o {
        Some(o) => println!("value of nth character: {o}"),
        None => println!("value not found"),
    }

    println!("value {o:?}");
}
