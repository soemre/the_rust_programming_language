const NTH: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const ITEMS: [&str; 12] = [
    "{} partridge in a pear tree.",
    "Two turtle doves,",
    "Three French hens,",
    "Four calling birds,",
    "Five golden rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,",
];

fn main() {
    println!("The Twelve Days of Christmas");
    println!();

    for n in 0..12 {
        if n != 0 {
            println!();
        }

        println!(
            "On the {} day of Christmas,\nmy true love gave to me",
            NTH[n]
        );

        for item in (0..=n).rev() {
            if item == 0 {
                println!(
                    "{} partridge in a pear tree.",
                    if n != 0 { "And a" } else { "A" }
                );
                continue;
            };

            println!("{}", ITEMS[item]);
        }
    }
}
