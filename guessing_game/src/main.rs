use std::io;

fn main() {
    // This is how you create a variable
    // let apples = 5;
    // println!("On a completely unrelated note, there are {apples} in the tree");

    println!("Let's play a game :)");
    println!("You're gonna guess a number. Ready???");
    println!("Input your guess:");

    let mut guess = String::new();
    let in_channel = io::stdin();
    in_channel
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess was: {guess}");

    // This is how you print a variable
    // let x = 5;
    // let y = 10;
    // println!("x = {x} and y + 2 = {}", y + 2);
}
