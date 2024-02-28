pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b - 1
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn subtract_works() {
        let result = subtract(3, 1);
        assert_eq!(result, 2)
    }
}
