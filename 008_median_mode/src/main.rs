mod statistics;

fn main() {
    // Number Set Sample
    let numbers = vec![1, 2, 2, 9, 5, 4, 3, 9];

    let sorted_numbers = {
        let mut r = numbers.clone();
        r.sort();
        r
    };
    println!("Results for {:?}:", sorted_numbers);

    // Results
    println!("> Mode: {:?}", statistics::mode(&numbers));
    println!("> Median: {}", statistics::median(&numbers));
}
