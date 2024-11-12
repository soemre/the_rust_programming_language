use std::io::{self, Write};

fn main() {
    // Retrive a number
    let (number, result) = 'in_l: loop {
        // Prompt for a number
        print!("Enter a number: ");
        io::stdout().flush().expect("Failed to flush stdout");

        // Get a number
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        // Validate the number
        let number: u32 = match number.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Please enter a valid positive number");
                continue;
            }
        };

        // Calculate the n-th number of Fibonacci
        let mut result = 0u32;
        let mut tail = 1;
        for i in 0..number {
            result = match result.checked_add(tail) {
                Some(n) => n,
                None => {
                    println!("Current program doesn't allows numbers bigger than {}", i);
                    continue 'in_l;
                }
            };
            tail = result - tail;
        }
        break (number, result);
    };

    // Prompt the result
    println!("The {number}th number of fibonacci is {result}.");
}
