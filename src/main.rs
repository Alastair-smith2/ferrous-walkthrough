mod fizz_buzz;

use fizz_buzz::fizzbuzz;
fn main() {
    for i in 1..=100 {
        // counts including 100
        let result = fizzbuzz(i);
        println!("The result {:?}", result);
    }
}
