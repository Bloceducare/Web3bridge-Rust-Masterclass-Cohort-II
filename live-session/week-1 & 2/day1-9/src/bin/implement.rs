pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

pub struct RectTup(pub u32, pub u32);

pub fn tuple_area(dimension: &RectTup) -> u32 {
    dimension.0 * dimension.1
}

pub fn area(dimension: &Rectangle) -> u32 {
    dimension.width * dimension.height
}

impl Rectangle {
    pub fn impl_area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = RectTup(34, 36);

    let tuple_result = tuple_area(&rect1);

    println!("Value of tuple rect is {:?}", tuple_result);

    let rect = Rectangle {
        width: 20,
        height: 40,
    };

    let res = rect.impl_area();

    println!("Value of rectangle implementation is {:?}", res);

    let result = area(&rect);

    println!("Value of result is {:?}", result);
}
