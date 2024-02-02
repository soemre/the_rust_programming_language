use std::io::{self, Write};

fn main() {
    // Get Input
    let fah: f64 = loop {
        // Ask for input
        print!("Enter a number: ");
        io::stdout().flush().expect("Couldn't prompt to stdout.");

        // Get Input
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Couldn't read stdin.");

        // Parse Input
        break match buf.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Given input must be a number!");
                continue;
            }
        };
    };

    // Convert to Celsius
    let cel = (fah - 32f64) / 1.8;

    // Convert to Result
    println!("{fah} F is {cel} C");
}
