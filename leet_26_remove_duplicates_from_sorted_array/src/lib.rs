pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // Index of the current number that is being checked.
    // As the array is sorted ascending, we simply want to find the next element that does not equal the current element.
    let mut check_index: usize = 0;

    for current in 1..nums.len() {
        // start with the second item, and compare it to the first
        if nums[check_index] != nums[current] {
            check_index += 1; // current and check are different.  This means the current is unique and should be copied forward and then the check advances.
            nums[check_index] = nums[current]; // Copy current to be the next check value
        }
    }

    return (check_index + 1) as i32;
}

pub fn remove_duplicates_mem_optimized(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut check_index = 0;
    let mut current = 1;
    let limit = nums.len();

    while current < limit {
        let candidate_value = nums[current];
        let check_value = nums[check_index];
        if check_value != candidate_value {
            check_index += 1;
            nums[check_index] = candidate_value;
        }
        current += 1;
    }

    (check_index + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_remove_duplicates1() {
        let mut nums = vec![1, 1, 2];
        let expected_output_start = [1, 2];
        let unique_count = remove_duplicates(&mut nums) as usize;
        assert_eq!(unique_count, expected_output_start.len());
        for i in 0..unique_count {
            assert_eq!(nums[i], expected_output_start[i]);
        }
    }

    #[test]
    fn it_remove_duplicates2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let expected_output_start = [0, 1, 2, 3, 4];

        let unique_count = remove_duplicates(&mut nums) as usize;
        assert_eq!(unique_count, expected_output_start.len());
        for i in 0..unique_count {
            assert_eq!(nums[i], expected_output_start[i]);
        }
    }

    #[test]
    fn it_remove_duplicates2_1() {
        let mut nums = vec![1, 1, 2];
        let k = remove_duplicates2(&mut nums);
        assert_eq!(k, 2);
        assert_eq!(&nums[..k as usize], &[1, 2]);
    }

    #[test]
    fn it_remove_duplicates2_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let k = remove_duplicates2(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(&nums[..k as usize], &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn it_remove_duplicates2_empty() {
        let mut nums = vec![];
        let k = remove_duplicates2(&mut nums);
        assert_eq!(k, 0);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn it_remove_duplicates_mem_optimized_1() {
        let mut nums = vec![1, 1, 2];
        let k = remove_duplicates_mem_optimized(&mut nums);
        assert_eq!(k, 2);
        assert_eq!(&nums[..k as usize], &[1, 2]);
    }

    #[test]
    fn it_remove_duplicates_mem_optimized_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let k = remove_duplicates_mem_optimized(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(&nums[..k as usize], &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn it_remove_duplicates_mem_optimized_empty() {
        let mut nums = vec![];
        let k = remove_duplicates_mem_optimized(&mut nums);
        assert_eq!(k, 0);
        assert_eq!(nums, vec![]);
    }
}
