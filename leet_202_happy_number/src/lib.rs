pub struct Solution;

use std::collections::HashSet;

struct Digits {
    // The number we are currently iterating over.
    // It will be reduced digit by digit.
    number: u32,
}

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.number == 0 {
            None
        } else {
            let digit = self.number % 10;
            self.number /= 10;
            Some(digit)
        }
    }
}

impl IntoIterator for Integer {
    type Item = u32;
    type IntoIter = Digits;

    fn into_iter(self) -> Self::IntoIter {
        Digits { number: self.0 }
    }
}
struct Integer(u32);

impl Integer {
    fn digits(&self) -> Digits {
        Digits { number: self.0 }
    }

    fn get_happy_sum(&self) -> u32 {
        self.digits().map(|d| d * d).sum()
    }
}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut current_num = n as u32;
        let mut seen = HashSet::new();


        loop {
            if current_num == 1 {
                break true;
            }

            // If insert returns false, the number was already seen -> cycle
            if !seen.insert(current_num) {
                break false;
            }

            // Calculate the next number
            current_num = Integer(current_num).get_happy_sum();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        let integer = Integer(5);

        assert_eq!(Integer(0).get_happy_sum(), 0);
        assert_eq!(integer.0, 5);
        assert_eq!(integer.get_happy_sum(), 25);
        assert_eq!(Integer(2).get_happy_sum(), 4);
        assert_eq!(Integer(91).get_happy_sum(), 82);
        assert_eq!(Integer(100).get_happy_sum(), 1);
    }

    #[test]
    fn test_happy_sum() {
        assert_eq!(Solution::is_happy(2), false);
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(100), true);
    }
}
