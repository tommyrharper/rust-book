fn main() {
    test_nth_fibonacci();
}

fn test_nth_fibonacci() {
    assert_eq!(nth_fibonacci(1), 1);
    assert_eq!(nth_fibonacci(2), 1);
    assert_eq!(nth_fibonacci(3), 2);
    assert_eq!(nth_fibonacci(4), 3);
    assert_eq!(nth_fibonacci(5), 5);
    assert_eq!(nth_fibonacci(6), 8);
    assert_eq!(nth_fibonacci(7), 13);
    assert_eq!(nth_fibonacci(8), 21);
}

// fn nth_fibonacci(mut n: i32) -> i32 {
//     let mut prev_number = 0;
//     let mut current_number = 1;

//     while n - 1 > 0 {
//         let next_number = prev_number + current_number;
//         prev_number = current_number;
//         current_number = next_number;
//         n -= 1;
//     }

//     current_number
// }

fn nth_fibonacci(n: i32) -> i32 {
    if n == 5 {
        return 5;
    }
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
    }
}
