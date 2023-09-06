use std::io;

//Constants can be global
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Mutable variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("There are {THREE_HOURS_IN_SECONDS} seconds in 3 hours");

    //Shadowing a variable
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Compound Types
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x is {x}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    // Array type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // Initialise with the same value
    let a = [3; 5];
    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index:");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index : usize = index
        .trim()
        .parse()
        .expect("Intex entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
