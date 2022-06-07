fn main() {
    let example = String::from("tom goes to school");
    let result = first_word(&example);

    println!("Result: {}", result);

    assert_eq!(result, "tom");
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
