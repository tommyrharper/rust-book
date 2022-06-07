fn main() {
    let mut x = String::from("beans");

    let y = &x;
    let z = &mut x;

    println!("{} {}", y, z);
}
