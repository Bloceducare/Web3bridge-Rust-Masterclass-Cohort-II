use std::collections::HashMap;
fn add(a: u8, b: u8) -> u8 {
    a + b
}
enum Result<T, Z> {
    Ok(T),
    Err(Z),
}

#[derive(Debug, Clone)]
struct Point1<T, U> {
    x: T,
    y: U,
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// X1        Y1
//       X
// X2        Y2

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl<T: Copy, U: Copy> Point1<T, U> {
    fn init(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn main() {
    let point = Point1 { x: 1, y: 2.0 };

    let val = point.init();

    println!("The value is : {val:?}");

    println!("The result is : {point:?}");

    let our_vec = vec![3.0, 22.0, 99.0, 12.0];

    let our_char = vec!['a', 'y', 'i', 'z', 'o'];

    let result = largest(&our_vec);

    let char_result = largest(&our_char);

    println!("The result is : {result:.1}");

    println!("The result is : {char_result}");

    let p1 = Point { x: 12, y: 16.5 };

    let p2 = Point {
        x: "Hello Abel",
        y: 'z',
    };

    let p3 = p1.mixup(p2);

    // let p3 = p2.mixup(p1);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
