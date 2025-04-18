pub struct Solution;

use std::{collections::HashMap};

impl Solution {
    pub fn majority_element_with_hash(nums: Vec<i32>) -> i32 {
        let limit = nums.len()/2;
        let mut hash: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        let mut max = nums[0];
        for num in nums {
            *hash.entry(num).or_insert(0) += 1;

            // the following 5 lines of code is equivalent to the prior line of code.
            // if let Some(value) = hash.get(&num) {
            //     hash.insert(num, value + 1);
            // } else {
            //     hash.insert(num, 1);
            // }

            max = std::cmp::max(max, *hash.get(&num).unwrap());
            if *hash.get(&num).unwrap() > limit as i32 {
                return num;
            }
        }
        max
    }
    
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let limit = nums.len() / 2;
        let mut counts = HashMap::with_capacity(nums.len());
        let first_num = nums[0];  // Store first element before loop
        
        for num in nums {
            let count = counts.entry(num).or_insert(0);
            *count += 1;
            
            if *count > limit {
                return num;
            }
        }
        
        first_num  // Return stored first element
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![3,2,3];
        let expected =  3;
        assert_eq!(Solution::majority_element(nums), expected);
        
        let nums = vec![2,2,1,1,1,2,2];
        let expected = 2;
        assert_eq!(Solution::majority_element(nums), expected);
    }
    #[test]
    fn test_2() {
        
        let nums = vec![6,5,5];
        let expected = 5;
        assert_eq!(Solution::majority_element(nums), expected);
    }
}
