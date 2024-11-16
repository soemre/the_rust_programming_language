pub fn to_piglatin(text: &str) -> String {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let first = match text.chars().next() {
        Some(l) => l,
        None => return String::new(),
    };

    if VOWELS.contains(&first) {
        return String::from(text) + "-hay";
    }

    return format!("{}-{}ay", String::from(&text[1..]), first);
}
