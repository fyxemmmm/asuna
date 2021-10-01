
fn main() {
    let mut s = String::from("Hello World!");
    let word_index = first_word(&s);

    s.clear();
    println!("{}", word_index);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
