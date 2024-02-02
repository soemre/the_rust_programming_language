use std::{
    cmp::Ordering,
    io::{self, Write},
};

use rand::Rng;

fn main() {
    // Initial
    let mut round = 0;
    let num = rand::thread_rng().gen_range(1..=100);

    loop {
        // Prompt to get guess input
        print!("Guess a number between 1 and 100: ");
        io::stdout()
            .flush()
            .expect("Couldn't print the prompt to stdout.");

        // Get guess input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Couldn't get the user input.");

        // Validate
        let guess = match guess.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Given input is not a number.");
                continue;
            }
        };

        if guess > 100 || guess < 1 {
            println!("Given input must be between 1 and 100.");
            continue;
        }

        // Finalize
        round += 1;
        match num.cmp(&guess) {
            Ordering::Less => println!("Too high!"),
            Ordering::Greater => println!("Too low!"),
            Ordering::Equal => {
                println!("You won in {round} rounds!");
                break;
            }
        };
    }
}
