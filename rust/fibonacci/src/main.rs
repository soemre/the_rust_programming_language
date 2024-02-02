use std::io::{self, Write};

fn main() {
    // Get Input
    let num: i32 = loop {
        // Ask for input
        print!("Enter a number: ");
        io::stdout().flush().expect("Couldn't prompt to stdout.");

        // Get Input
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Couldn't read stdin.");

        // Parse Input
        let buf = match buf.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Given input must be a number!");
                continue;
            }
        };

        // Validate Range
        if buf > 2 {
            break buf;
        } else {
            println!("Given input must be higher than 2!");
        };
    };

    // Generate the fibonacci number
    let mut prev = 1;
    let mut result = 1;

    for _ in 0..(num - 2) {
        let temp = prev;
        prev = result;
        result += temp;
    }

    // Prompt the result
    println!("The {num}th number of fibonacci is {result}.");
}
