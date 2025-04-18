struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut check_vector = vec![0;nums.len() + 1];

        for num in nums {
            check_vector[num as usize] = num;
        }

        for (i, num) in check_vector.iter().enumerate() {
            if *num != i as i32 {
                return i as i32;
            }
        }
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_missing() {
        let input= vec![1,2];
        let expected: i32 = 0;
        assert_eq!(Solution::missing_number(input), expected);
        
        let input = vec![3,0,1];
        let expected = 2;
        assert_eq!(Solution::missing_number(input), expected);
    }
}
