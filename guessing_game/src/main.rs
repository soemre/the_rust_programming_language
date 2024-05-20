use std::{
    cmp::Ordering,
    io::{stdin, stdout, Write},
};

use rand::{self, Rng};

const RESULT_PREFIX: &str = "==> ";
const RESULT_SUFFIX: &str = " Try again.";
fn main() {
    // Initialize the game by generating a random number and prompting a message
    println!("Welcome! You need to guess a number between 1 and 100.");
    let rn = rand::thread_rng().gen_range(1..=100);

    let mut round = 0;

    // Keep asking until they find the number
    loop {
        // Increase the round number
        round += 1;

        // Ask for a guess and retrive it
        println!("\n##### Round {round} #####");
        print!("Enter your guess: ");
        stdout().flush().expect("Something went really wrong!");

        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("Something went really wrong!");

        // Convert the guess string to an integer and validate that it's an intager
        let guess = match guess.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                println!("{RESULT_PREFIX}Your guess needs to be an integer. {RESULT_SUFFIX}");
                continue;
            }
        };
        if guess > 100 || guess < 1 {
            println!("{RESULT_PREFIX}Your guess needs to be between 1 and 100. {RESULT_SUFFIX}");
            continue;
        }

        // Compare the guess with the pre-generated number and prompt the current result
        match rn.cmp(&guess) {
            Ordering::Less => println!("{RESULT_PREFIX}Too high! {RESULT_SUFFIX}"),
            Ordering::Greater => println!("{RESULT_PREFIX}Too low! {RESULT_SUFFIX}"),
            Ordering::Equal => {
                println!("\n##### RESULT #####");
                println!("You've found it!");
                println!("It's only been {round} rounds.");
                println!("Well done.");
                break;
            }
        }
    }
}
