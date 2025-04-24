struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn naive_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (pos1, num1) in nums.iter().enumerate() {
            for (pos2, num2) in nums[pos1 + 1..nums.len()].iter().enumerate() {
                if num1 + num2 == target {
                    // pos2 actually starts at pos1 + 1 relative to nums, so the real_pos2 needs to add 1 to the sum of the two pos indices.
                    let real_pos2 = pos1 + 1 + pos2;
                    return vec![pos1 as i32, real_pos2 as i32];
                }
            }
        }
        vec![]
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (pos, num) in nums.iter().enumerate() {
            let solution = target - num;
            if let Some(addend_index) = hash.get(&solution) {
                return vec![pos as i32, *addend_index as i32];
            } else {
                hash.insert(*num, pos);
            }
        }
        panic!("Solution not found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let input = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![1, 0];
        assert_eq!(Solution::two_sum(input, target), expected);

        let input = vec![3, 2, 4];
        let target = 6;
        let expected = vec![2, 1];
        assert_eq!(Solution::two_sum(input, target), expected);

        let input = vec![3, 3];
        let target = 6;
        let expected = vec![1, 0];
        assert_eq!(Solution::two_sum(input, target), expected);
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        let mut value: u8 = match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        };

        if let Coin::Nickel = coin {
            value = 5;
        }
        match coin {
            Coin::Penny => println!("We have a penny!"),
            _ => (),
        }

        value
    }

    #[test]
    fn test_match() {
        assert_eq!(value_in_cents(Coin::Penny), 1);
        assert_eq!(value_in_cents(Coin::Nickel), 5);
        assert_eq!(value_in_cents(Coin::Dime), 10);
        assert_eq!(value_in_cents(Coin::Quarter), 25);
    }
}
