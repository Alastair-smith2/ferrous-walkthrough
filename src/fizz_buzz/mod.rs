pub fn fizzbuzz(i: u32) -> String {
    match (i % 3 == 0, i % 5 == 0) {
        (true, true) => String::from("FizzBuzz"),
        (true, false) => String::from("Fizz"),
        (false, true) => String::from("Buzz"),
        _ => i.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fizzbuzz_divisible_by_3_works() {
        assert_eq!(fizzbuzz(6), String::from("Fizz"))
    }

    #[test]
    fn fizzbuzz_divisible_by_5_works() {
        assert_eq!(fizzbuzz(10), String::from("Buzz"))
    }

    #[test]
    fn fizzbuzz_divisible_by_15_works() {
        assert_eq!(fizzbuzz(15), String::from("FizzBuzz"))
    }

    #[test]
    fn fizzbuzz_non_divisible_works() {
        assert_eq!(fizzbuzz(8), String::from("8"))
    }
}
