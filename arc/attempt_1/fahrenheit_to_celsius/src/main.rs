use core::fmt;
use std::{env, process};

fn main() {
    let mode = match env::args_os().nth(1) {
        Some(mode) => mode.into_string().expect("given argument must use unicode"),
        None => {
            eprintln!("you must specify a mode (f|c)");
            process::exit(1);
        }
    };

    let val = match env::args_os().nth(2) {
        Some(val) => {
            let val = val
                .into_string()
                .expect("given argument must be an valid integer value");

            match val.parse() {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("you must provide an integer value");
                    process::exit(1);
                }
            }
        }
        None => {
            eprintln!("you must provide an integer value");
            process::exit(1);
        }
    };

    let temp = match mode.as_str() {
        "f" => Temperature::from_fahrenheit(val),
        "c" => Temperature::from_celsius(val),
        _ => {
            eprintln!("invalid mode (f|c)");
            process::exit(1);
        }
    };

    println!("{}", temp)
}

struct Temperature {
    fahrenheit: i32,
    celsius: i32,
}

impl Temperature {
    fn from_celsius(celsius: i32) -> Temperature {
        let fahrenheit = (celsius * 9 / 5) + 32;

        Temperature {
            celsius,
            fahrenheit,
        }
    }

    fn from_fahrenheit(fahrenheit: i32) -> Temperature {
        let celsius = (fahrenheit - 32) * 5 / 9;

        Temperature {
            celsius,
            fahrenheit,
        }
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Fahrenheit: {}, Celsius: {}",
            self.fahrenheit, self.celsius
        )
    }
}
