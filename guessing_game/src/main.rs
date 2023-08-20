use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    // This is how you create a variable
    // let apples = 5;
    // println!("On a completely unrelated note, there are {apples} in the tree");

    println!("Let's play a game :)");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {secret_number}");

    println!("You're gonna guess a number between 1 and 100. Ready???");
    loop {
        println!("Input your guess:");

        let mut guess = String::new();
        let in_channel = io::stdin();
        in_channel
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue, //_ is a catchall value
        };
        println!("Your guess was: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Try a bigger one"),
            Ordering::Greater => println!("Try a smaller one"),
            Ordering::Equal => {
                println!("Good job, here's a cookie!");
                break;
            },
        }
    }

    // This is how you print a variable
    // let x = 5;
    // let y = 10;
    // println!("x = {x} and y + 2 = {}", y + 2);
}
