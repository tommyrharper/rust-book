use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let string_is_num = guess.trim().parse::<f64>().is_ok();

    if !string_is_num {
        println!("That is not a number!");
        return;
    }
    
    println!("You guessed: {}", guess);
}
