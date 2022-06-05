const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Mutation
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("{}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    let x = x + 1;

    {
        let x = x * 2;
        // x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // Numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    println!(
        "sum: {}, difference: {}, product: {}, quotient: {}, floored: {}, remainder: {}",
        sum, difference, product, quotient, floored, remainder
    );
}
