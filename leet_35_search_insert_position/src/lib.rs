pub fn search_insert2(nums: Vec<i32>, target: i32) -> i32 {
    for (i, num) in nums.iter().enumerate() {
        if *num >= target {
            return i as i32;
        }
    }
    nums.len() as i32
}

pub fn search_insert3(nums: Vec<i32>, target: i32) -> i32 {
    for i in 0..nums.len() {
        if nums[i] >= target {
            return i as i32;
        }
    }
    nums.len() as i32
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    (0..nums.len())
        .find(|&i| nums[i] >= target)
        .unwrap_or(nums.len()) as i32
}

pub fn search_insert_binary(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn example_1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let output = 2;
        assert_eq!(search_insert(nums, target), output);
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let output = 1;
        assert_eq!(search_insert(nums, target), output);
    }

    #[test]
    fn example_3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let output = 4;
        assert_eq!(search_insert(nums, target), output);
    }

    #[test]
    fn example_binary_1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let output = 2;
        assert_eq!(search_insert_binary(nums, target), output);
    }

    #[test]
    fn example_binary_2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let output = 1;
        assert_eq!(search_insert_binary(nums, target), output);
    }

    #[test]
    fn example_binary_3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let output = 4;
        assert_eq!(search_insert_binary(nums, target), output);
    }

    #[test]
    fn performance_comparison() {
        // Create a sorted array with 1 million elements
        let nums: Vec<i32> = (0..1_000_000).collect();

        // Test case 1: Target at the very end (worst case for linear search)
        let target = 999_999;

        // Linear search
        let start = Instant::now();
        let result_linear = search_insert(nums.clone(), target);
        let duration_linear = start.elapsed();

        // Binary search
        let start = Instant::now();
        let result_binary = search_insert_binary(nums.clone(), target);
        let duration_binary = start.elapsed();

        println!(
            "Target at end ({}): Linear: {:?}, Binary: {:?}",
            target, duration_linear, duration_binary
        );
        assert_eq!(result_linear, result_binary);

        // Test case 2: Target not in array (somewhere in the middle)
        let target = 500_000 + 1; // Looking for a value that doesn't exist

        // Linear search
        let start = Instant::now();
        let result_linear = search_insert(nums.clone(), target);
        let duration_linear = start.elapsed();

        // Binary search
        let start = Instant::now();
        let result_binary = search_insert_binary(nums, target);
        let duration_binary = start.elapsed();

        println!(
            "Target in middle ({}): Linear: {:?}, Binary: {:?}",
            target, duration_linear, duration_binary
        );
        assert_eq!(result_linear, result_binary);
    }
}
