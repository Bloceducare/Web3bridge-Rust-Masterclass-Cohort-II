fn main() {
    let mut s1 = String::from("hello");
    s1 = String::from("adam");

    // 0xf6a20020
    // 0x105779aa0

    println!("==== cap, len, ptr S1=========");
    println!("S1 {} {} {:?}", s1.len(), s1.capacity(), s1.as_ptr());
    let s2 = &s1;

    println!("{s1} {}", "world!");

    println!("==== cap, len, ptr S2=========");
    println!("S2 {} {} {:?}", s2.len(), s2.capacity(), s2.as_ptr());

    println!("====int=========");

    let a = ["hello", "hi"];
    let b = a;

    println!("A {} {:?}", a.len(), s1.as_ptr());

    println!("value of a is {a:?}");

    let name = String::from("Hello");

    take(&name);

    take_and_give_back(name);
}

fn take(take: &String) {
    println!("Rust is taken: {take}");
}

fn take_and_give_back(take: String) -> String {
    println!("Rust is taken: {take}");
    take
}
