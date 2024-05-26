#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_must_use)]
use std::io;
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    const TILES_PER_PAGE: i32 = 15;
    const TILES_PER_SQUARE: i32 = 4;

    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t: i32 = scan.next();

    for _ in 0..t {
        let mut x = scan.next::<i32>();
        let y = scan.next::<i32>();
        let pages_w_2_squares = y / 2;
        let pages_w_1_square = y % 2;
        x -= (TILES_PER_PAGE - TILES_PER_SQUARE * 2) * pages_w_2_squares;
        x -= (TILES_PER_PAGE - TILES_PER_SQUARE) * pages_w_1_square;
        if x < 0 {
            write!(out, "{}\n", pages_w_2_squares + pages_w_1_square).ok();
        } else {
            if x % TILES_PER_PAGE == 0 {
                write!(
                    out,
                    "{}\n",
                    pages_w_2_squares + pages_w_1_square + x / TILES_PER_PAGE
                )
                .ok();
            } else {
                write!(
                    out,
                    "{}\n",
                    pages_w_2_squares + pages_w_1_square + x / TILES_PER_PAGE + 1
                )
                .ok();
            }
        }
    }
}
