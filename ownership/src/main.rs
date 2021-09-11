fn main() {
    let mut s = String::from("hello"); // String型はヒープにメモリを確保する
    s.push_str(" world"); // String型は可変化が可能
    let len = calculate_length(&s);

    println!("The length of {} is {}", s, len);

    let word = first_word(&s);
    println!("word is {}", word);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
