fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Shadowing:");

    let x = 7;
    println!("The value of x is: {x}");
    let x = x + 1;

    {
        let x = x + 1;
        println!("The value of x is: {x}");
    }

    println!("The value of x is: {x}");

    let x;

    x = 0; // If you omit this line, it won't compile

    println!("{x}")
}
