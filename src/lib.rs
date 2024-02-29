use rand::Rng;



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b 
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

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
    #[test]
    fn randfun() {
        let random_number= rand::thread_rng().gen_range(1..=100);
        match random_number.cmp(&1){
            Ordering::Greater => print!("Rust is doch ganz nice bisher \n"),
            Ordering::Equal => print!("(bin auf seite 3)\n"),
            Ordering::Less => print!("Rust is kacke\n")
        }
    }
}
