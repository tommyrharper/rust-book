use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {query:?} in file {filename:?}");

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text: \n{}", contents);

    if contents.contains(query) {
        println!("------ {} found! -------", query);
    } else {
        println!("------ {} not found! -------", query);
    }
}
