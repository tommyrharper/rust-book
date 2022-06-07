#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: i32
}

fn main() {
    let user1 = User {
        username: String::from("tom"),
        email: String::from("tom@tom.com"),
        age: 25,
    };

    let user2 = User {
        email: String::from("tom2@tom.com"),
        username: user1.username.clone(),
        ..user1
    };

    println!("user2: {:?}, user1: {:?}", user2, user1);
}
