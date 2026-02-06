#[derive(Debug)]
struct StudentDetails {
    id: u8,
    first_name: String,
    last_name: String,
    age: u8,
    is_registered: bool,
}

struct Color(i32, u32, f64);

fn main() {
    let mut abel = StudentDetails {
        id: 1,
        first_name: String::from("Abel"),
        last_name: String::from("Osaretin"),
        age: 2,
        is_registered: true,
    };

    abel.last_name = String::from("Bimbo");

    let mike = StudentDetails {
        first_name: String::from("Mike"),
        last_name: String::from("Anand"),
        is_registered: false,
        ..abel
    };

    println!("New student {:?}", mike);

    println!(
        "Student details {:?} {:?} {:?} {:?}",
        abel.first_name, abel.last_name, abel.age, abel.id
    );

    let s = dangle();
    let b = s.to_owned();
    println!("value of {b}");

    let _black = Color(0, 0, 0.0);
}

fn create_student(last_name: String, first_name: String, age: u8, id: u8) -> StudentDetails {
    StudentDetails {
        id,
        first_name,
        last_name,
        age,
        is_registered: true,
    }
}

fn create_student2(data: &StudentDetails) -> StudentDetails {
    StudentDetails {
        id: data.id,
        first_name: data.first_name.clone(),
        last_name: data.last_name.clone(),
        age: data.age,
        is_registered: data.is_registered,
    }
}

fn dangle() -> &'static str {
    let s = "hello";
    &s
}

fn create_student_(data: &mut StudentDetails) -> &StudentDetails {
    data
}
