use rand::{self, Rng};

use std::{
    cmp::Ordering,
    io::{self, Write},
    ops::RangeInclusive,
};

const GUESS_RANGE: RangeInclusive<i32> = 1..=100;

fn main() {
    let mut round = 0;

    // Create a random intager between 1 and 100
    let rand_num = rand::thread_rng().gen_range(GUESS_RANGE);

    loop {
        // increase the current round count
        round += 1;

        // guess input
        let guess = get_guess(round, GUESS_RANGE);

        // indicate "low" or "high"
        match guess.cmp(&rand_num) {
            Ordering::Equal => break,
            Ordering::Greater => println!("> Too high! Try agian.\n"),
            Ordering::Less => println!("> Too low! Try agian.\n"),
        }
    }

    // terminate
    println!("> Well done! You guessed it right. That was your {round} round.");
}

fn get_guess(round: i32, range: RangeInclusive<i32>) -> i32 {
    loop {
        // Prompt
        print!("{round}: Guess a number: ");
        io::stdout().flush().expect("stdout, system error");

        // Input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("couldn't read the guess");

        // String Parsing
        match guess.trim().parse() {
            Result::Ok(guess) => {
                // Validate the range
                // if 100 < guess || 1 > guess {
                if !range.contains(&guess) {
                    println!(
                        "> Given input must be between {} and {}.\n",
                        range.start(),
                        range.end()
                    );
                    continue;
                }

                break guess;
            }
            Result::Err(_) => {
                println!("> Given input is not a number.\n");
                continue;
            }
        }
    }
}
