use std::{env, process::exit};

fn main() {
    let n = {
        let arg = match env::args_os().nth(1) {
            Some(s) => match s.into_string() {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("given string includes unknown characters");
                    exit(1);
                }
            },
            None => {
                eprintln!("you must specify a positive integer in order to run this command");
                exit(1);
            }
        };

        match arg.parse::<u32>() {
            Ok(v) => v,
            Err(_) => {
                eprintln!("given argument should be an positive integer.");
                exit(1);
            }
        }
    };

    print!("1");
    for current in 2..=n {
        print!(", {}", fibonacci(current));
    }
    print!("\n");

    println!("{n}th number of fibonacci: {}", fibonacci(n));
}

fn fibonacci(n: u32) -> u32 {
    if n == 1 || n == 0 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}
