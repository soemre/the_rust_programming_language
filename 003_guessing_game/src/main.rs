use std::{
    cmp::Ordering,
    io::{self, Write},
    ops::RangeInclusive,
};

use rand::Rng;

const INTERVAL: RangeInclusive<i32> = 1..=100;

fn main() {
    // Initialize the game
    let secret = rand::thread_rng().gen_range(INTERVAL);
    let mut round_count = 1;

    println!("Welcome to the guessing game!");

    loop {
        // Prompt the user for a guess
        print!("[{round_count}] What do you think it is: ");
        io::stdout()
            .flush()
            .expect(prompt_err("flushing STDOUT").as_str());

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect(&prompt_err("reading STDIN").as_str());

        // Validate the guess
        let guess = match guess.trim().parse() {
            Ok(n) => {
                if !INTERVAL.contains(&n) {
                    prompt_interval_err();
                    continue;
                }
                n
            }
            Err(_) => {
                prompt_interval_err();
                continue;
            }
        };

        // Check the guess
        round_count += 1;

        match guess.cmp(&secret) {
            Ordering::Less => println!("> Too Low!\n"),
            Ordering::Greater => println!("> Too High!\n"),
            Ordering::Equal => {
                println!("> Congratulations! You guessed it right!");
                println!("> It was your {}. round. Well done!", round_count - 1);
                break;
            }
        }
    }
}

// Helper functions
fn prompt_interval_err() {
    println!(
        "> Opps! You need to enter a number between {} and {}. Try again!\n",
        INTERVAL.start(),
        INTERVAL.end()
    );
}

fn prompt_err(msg: &str) -> String {
    format!("While {} something unexpected happend!", msg)
}
