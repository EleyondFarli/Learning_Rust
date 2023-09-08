use crate::christmas::sing_twelve_days;
use crate::fibonacci::{backwards_nth_fib, forwards_nth_fib};
pub mod temperature;
pub mod fibonacci;
pub mod christmas;

fn main() {
    // temperature::test_temperature(-40.0);
    // println!("{}", backwards_nth_fib(7));
    // println!("{}", forwards_nth_fib(50));
    sing_twelve_days();
}
