use std::cmp::Eq;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T, Key, Value>
where
    T: Fn(Key) -> Value,
    Key: Eq + std::hash::Hash + Copy,
    Value: Copy,
{
    calculation: T,
    value: HashMap<Key, Value>,
}

impl<T, Key, Value> Cacher<T, Key, Value>
where
    T: Fn(Key) -> Value,
    Key: Eq + std::hash::Hash + Copy,
    Value: Copy,
{
    fn new(calculation: T) -> Cacher<T, Key, Value> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: Key) -> Value {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a: u32| a);

    let v1 = c.value(1);
    assert_eq!(v1, 1);

    let v2 = c.value(2);
    assert_eq!(v2, 2);
}
