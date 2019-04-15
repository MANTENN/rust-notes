fn main() {
    println!("First Word!");

    let s = String::from("Hello World!");

    let word = first_word(&s);

    println!("First word: {}", word);

}

// Prefered approach over:
//  fn first_word(s: &str) -> &str
// because it allows both String and &str values to be passed in as arguments
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }

    }

    &s
}