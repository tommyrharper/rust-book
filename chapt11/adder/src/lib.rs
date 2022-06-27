pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_two(2);
        assert_eq!(result, 4);
        assert!(true);
        assert_ne!(3, 4);
        let error: Result<i32, String> = Err(String::from("error"));
        assert!(error.is_err())
    }

    #[test]
    fn with_custom_message() {
        let thing = String::from("beans");
        assert!(true, "this test has a custom message full of {}", thing);
    }

    #[test]
    #[should_panic(expected = "Make this test fail")]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    #[should_panic]
    fn another_panic() {
        panic!("Make this test fail");
    }

    #[test]
    fn it_works_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
