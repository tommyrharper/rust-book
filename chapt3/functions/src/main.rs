fn main() {
    println!("Hello, world!");

    another_function(5);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    println!("this is five + one: {}", plus_one(five()));
}

fn another_function(x: i32) {
    println!("another function, {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
