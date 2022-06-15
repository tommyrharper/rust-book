use std::collections::HashMap;

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

fn main() {
    let input = vec![2, 4, 5, 6, 7, 7];
    // let input = vec![3, 6, 9, 8, 9];

    let mode = get_mode(&input);

    println!("mode: {}", mode);
}

fn get_mode(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for el in vec {
        let count = map.entry(el).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    let mut res = vec[0];

    for (key, value) in &map {
        if value > &max {
            max = *value;
            res = **key;
        }
    }

    res
}