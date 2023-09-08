pub fn backwards_nth_fib(n: i64) -> i64
{
    if n == 0 || n == 1 {
        return n;
    }
    backwards_nth_fib(n - 1) + backwards_nth_fib(n - 2)
}

pub fn forwards_nth_fib(n: i64) -> i64 {
    if n == 0 || n == 1 {
        return n;
    }
    forwards_nth_fib_computation(n - 1, 0, 1)
}
pub fn forwards_nth_fib_computation(cnt: i64, last1: i64, last2: i64) -> i64
{
    if cnt == 0 {
        return last2;
    }
    forwards_nth_fib_computation(cnt - 1, last2, last1 + last2)
}