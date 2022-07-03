#[derive(PartialEq, Debug)]
struct Counter {
    value: u32,
}

impl Counter {
    pub fn new() -> Self {
        Counter { value: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value < 5 {
            self.value = &self.value + 1;
            Some(self.value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let mut counter = Counter::new();
        assert_eq!(counter.value, 0);
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            // (1, 2), (2, 3), (3, 4), (4, 5)
            .map(|(a, b)| a * b)
            //  2, 6, 12, 20
            .filter(|x| x % 3 == 0)
            // 6, 12
            .sum();
        assert_eq!(18, sum);
    }
}
