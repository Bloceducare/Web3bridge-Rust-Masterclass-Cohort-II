#[derive(Debug)]
struct BorrowedValue<'a, 'b> {
    part1: &'a mut u8,
    part2: &'b str,
    id: u8,
}

impl<'a, 'b> BorrowedValue<'a, 'b> {
    fn run_away() {
        println!("Run");
    }

    // inside implementation we don't need to specify the <'a,'b>
    // because the struct impl already has the lifetime specifier
    // and this would shadow the existing specifier contained in the
    // struct implementation
    fn tup(x: &'a str, y: &'b str) -> (&'a str, &'b str) {
        (x, y)
    }
}

// outside of an implementation we need to specify the <'a,'b>
fn tup<'a, 'b>(x: &'a str, y: &'b str) -> (&'a str, &'b str) {
    (x, y)
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // this won't work
    // let r;
    //
    // {
    //     let x = 5;
    //     r = &x;
    // }

    // this works
    let r = 1; // r is 'a

    {
        let x = &r; // x is 'b
        // 'b = & 'a
        // r = &x;
    }
    println!("r: {r}");

    let x = "hello";
    let y = "sam";
    let return_value = longest(x, y);
    println!("The longest values is: {return_value}");
    println!("directly: {}", longest(x, y));

    let mut one = 1_u8;
    let bv = BorrowedValue {
        part1: &mut one,
        part2: "another borrow",
        id: 1,
    };

    println!("BorrowedValue: {bv:?}");
}
