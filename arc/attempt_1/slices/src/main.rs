fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    println!("{}", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    let slice2 = &[2, 3];

    assert_eq!(slice, slice2);

    let s = &String::from("value");
    println!("{}", s);

    // let b = Box::new(a[..]);

    let mut s = String::from("abcdefg");
    let s_slice = &mut s[2..5];

    s_slice.make_ascii_uppercase();

    println!("{} {}", s_slice, s_slice.len());
}

// When you create an instance like that it will be only created for the current frame stack
// fn return_string() -> &String {
//     &String::from("value")
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
