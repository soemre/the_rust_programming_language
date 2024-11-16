mod translate;

use std::io::{self, Write};
fn main() {
    // Ask for input
    let mut buf = String::new();

    print!("Translate to pig-latin: ");
    io::stdout().flush().expect("Couldn't flush the stdout");
    io::stdin()
        .read_line(&mut buf)
        .expect("Couldn't read the line");

    // Print the translation
    print!("> ");
    for word in buf.split_whitespace() {
        print!("{} ", translate::to_piglatin(word));
    }
    print!("\n");
}
