fn main() {
    let mut count = 0;
    let val = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up true;
            }
            remaining -= 1;
        }

        count += 1;
    };
    println!("End count = {count} & Loop return val = {val}");

    forin();
}

fn forin() {
    for number in 1..4 {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
