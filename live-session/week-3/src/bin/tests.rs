fn add(a: u8, b: u8) -> u8 {
    a + b
}

fn main() {
    println!()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn setup() -> u8 {
        add(3, 7)
    }

    #[test]
    fn test_add() {
        let result = setup();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_not_equal() {
        let result = setup();
        assert_ne!(result, 4);
    }
}
