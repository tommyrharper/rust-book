fn main() {
    let res = last_char_of_first_line("asd\nhi");

    match res {
        Some(val) => println!("res: {val}"),
        None => println!("nope"),
    }
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}