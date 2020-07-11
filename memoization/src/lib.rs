#[macro_use]
extern crate log;

use std::collections::HashMap;

#[allow(unused)]
struct Factorial {
    cache: HashMap<i32, i32>,
}

impl Factorial {
    #[allow(unused)]
    fn new() -> Factorial {
        let cache: HashMap<i32, i32> = HashMap::new();
        Factorial { cache }
    }

    #[allow(unused)]
    fn factorial(&mut self, n: i32) -> i32 {
        debug!("Current value of n: {:?}", &n);

        if self.cache.contains_key(&n) {
            return *self.cache.get(&n).unwrap();
        }

        match n {
            0 => 1,
            _ => {
                let x = self.factorial(n - 1) * n;
                self.cache.insert(n, x);

                return x;
            }
        }
    }
}

#[allow(unused)]
fn factorial(n: i32) -> i32 {
    debug!("Current value of n: {:?}", &n);

    match n {
        0 => 1,
        _ => factorial(n - 1) * n,
    }
}

#[cfg(test)]
mod tests {
    use super::{factorial, Factorial};

    struct TestCase {
        input: i32,
        expected: i32,
    }

    impl TestCase {
        fn new(input: i32, expected: i32) -> TestCase {
            TestCase { input, expected }
        }
    }

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_factorial() {
        init();

        let cases = vec![TestCase::new(5, 120), TestCase::new(3, 6)];

        for case in cases {
            let result = factorial(case.input);
            assert_eq!(result, case.expected);
        }

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_memoized_factorial() {
        init();

        let mut mem_factorial = Factorial::new();

        let cases = vec![
            TestCase::new(5, 120),
            TestCase::new(3, 6),
            TestCase::new(7, 5040),
        ];

        for case in cases {
            let result = mem_factorial.factorial(case.input);
            assert_eq!(result, case.expected);
        }
    }
}
