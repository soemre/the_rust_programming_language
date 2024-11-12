const NUMBERS_TH: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: [&str; 12] = [
    "And a partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

fn main() {
    println!("The Twelve Days of Christmas by The Wiggles");

    for i in 0..12 {
        println!(
            "\nOn the {} day of Christmas,\nmy true love gave to me",
            NUMBERS_TH[i]
        );
        if i == 0 {
            println!("A partridge in a pear tree");
            continue;
        }

        for j in (0..i + 1).rev() {
            println!("{}", GIFTS[j]);
        }
    }
}
