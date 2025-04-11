#[allow(dead_code)]
#[allow(unused_imports)]

struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut seen = Vec::new();
        let mut used = 0;
        let mut top3: Vec<i32> = nums
            .iter()
            .cloned()
            .filter(|&x| {
                used += 1;
                if !seen.contains(&x) {
                    seen.push(x);
                    true
                } else {
                    false
                }
            })
            .take(3)
            .collect();

        top3.sort_by(|a, b| b.cmp(a));

        for &num in nums.iter().skip(used) {
            if num <= top3[2] || top3.contains(&num) {
                // we know that there are 3 numbers in the vector.  Skip for any less than the top 3.
                continue;
            } else {
                top3[2] = num;
                top3.sort_by(|a, b| b.cmp(a));
            }
        }

        if top3.len() > 2 { top3[2] } else { top3[0] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_third_max() {
        assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
        assert_eq!(Solution::third_max(vec![1, 2]), 2);
        assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
        assert_eq!(Solution::third_max(vec![1, 2, 2, 5, 3, 5]), 2);
    }
}
