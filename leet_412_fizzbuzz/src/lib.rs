struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        const FIZZ: &str = "Fizz";
        const BUZZ: &str = "Buzz";
        const FIZZBUZZ: &str = "FizzBuzz";
        let result: Vec<String> = (1..=n)
            .map(|i| {
                if i % 15 == 0 {
                    FIZZBUZZ.to_string()
                } else if i % 3 == 0 {
                    FIZZ.to_string()
                } else if i % 5 == 0 {
                    BUZZ.to_string()
                } else {
                    format!("{i}")
                }
            })
            .collect();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 5;
        let output = vec!["1", "2", "Fizz", "4", "Buzz"];
        assert_eq!(Solution::fizz_buzz(n), output);
    }
}
