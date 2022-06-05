fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else if number == 5 {
        println!("number was 5 and condition was false");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let conditional_number = if condition { 5 } else { 6 };
    println!("value: {}", conditional_number);
}
