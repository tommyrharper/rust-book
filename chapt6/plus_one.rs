fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    print_option(six);
    print_option(none);
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn print_option(option: Option<i32>) {
    match option {
        Some(i) => println!("{}", i),
        // None => println!("nothing to print"),
        _ => ()
    }
}
