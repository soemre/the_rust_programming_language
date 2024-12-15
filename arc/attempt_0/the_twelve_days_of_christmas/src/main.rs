fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let lyrics = [
        "Twelve drummers drumming,",
        "Eleven pipers piping,",
        "Ten lords a-leaping,",
        "Nine ladies dancing,",
        "Eight maids a-milking,",
        "Seven swans a-swimming,",
        "Six geese a-laying,",
        "Five golden rings,",
        "Four calling birds,",
        "Three French hens,",
        "Two turtle doves,",
        "And a partridge in a pear tree!",
    ];

    for i in 0..days.len() {
        println!(
            "On the {} day of Christmas,\nmy true love gave to me",
            days[i]
        );

        if i == 0 {
            println!("A partridge in a pear tree!");
        } else {
            for line in &lyrics[(lyrics.len() - 1 - i)..] {
                println!("{line}");
            }
        };

        println!();
    }
}
