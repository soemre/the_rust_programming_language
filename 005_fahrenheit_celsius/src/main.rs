use std::io::{self, Write};

const FORMAT_ERR: &str = "Your inputs must be numbers followed by 'F' or 'C'. (i.e. 100C)";

fn main() {
    loop {
        // Get User Input
        print!("Enter a F or C degree to convert: ");
        io::stdout().flush().expect("Couldn't flush STDOUT");
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Couldn't get the user input from STDIN");

        // Validate the Required Format
        buf = buf.trim().to_uppercase();
        let degree: Degree = match buf.find(|c| c == 'C' || c == 'F') {
            Some(i) => {
                if buf.len() > i + 1 {
                    println!("{}", FORMAT_ERR);
                    continue;
                }

                let degree = match &buf[..i].trim().parse::<f64>() {
                    Ok(d) => d,
                    Err(_) => {
                        println!("{}", FORMAT_ERR);
                        continue;
                    }
                }
                .clone();

                if buf.ends_with('C') {
                    Degree::Celsius(degree)
                } else {
                    Degree::Fahrenheit(degree)
                }
            }
            None => {
                println!("{}", FORMAT_ERR);
                continue;
            }
        };

        // Calculate and Prompt
        match degree {
            Degree::Fahrenheit(d) => println!("> {d} F = {:.2} C", (d - 32.) * 5. / 9.),
            Degree::Celsius(d) => println!("> {d} C = {:.2} F", d * 9. / 5. + 32.),
        }
        break;
    }
}

enum Degree {
    Fahrenheit(f64),
    Celsius(f64),
}
