use std::io::stdin;

fn main() {
    let mut val = String::from("default value");

    get_user_input(&mut val);

    let val = val;
    println!("immutable val variable: \"{val}\"");
}

fn get_user_input(dest: &mut String) {
    let mut buf = String::new();

    stdin()
        .read_line(&mut buf)
        .expect("couldn't read user input");

    buf = String::from(buf.trim());

    if buf != "" {
        *dest = buf;
    }
}
