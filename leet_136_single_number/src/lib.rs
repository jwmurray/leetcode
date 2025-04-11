struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut hashset: HashSet<i32> = HashSet::with_capacity(nums.len());

        for num in nums {
            if !hashset.insert(num) {
                // element is already in hashset, so remove it
                hashset.remove(&num); // this one has gone in and out
            }
        }
        return hashset.drain().next().unwrap(); // per the instructions, there is exactly one left over
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![2, 2, 1];
        let expected = 1;
        assert_eq!(Solution::single_number(nums), expected);

        let nums = vec![4, 1, 2, 1, 2];
        let expected = 4;
        assert_eq!(Solution::single_number(nums), expected);
    }
}
