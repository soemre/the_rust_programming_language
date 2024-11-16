mod company;
mod syntax;

use std::io::{self, Write};

fn main() {
    let mut cmp = company::Company::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("couldn't flush stdout");

        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("couldn't read the line");

        match syntax::interpret_line(&buf) {
            Ok(r) => cmp.handle(r),
            Err(err) => {
                println!("! {err}");
                continue;
            }
        };
    }
}
