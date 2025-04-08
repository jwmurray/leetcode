pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let output = nums
        .iter()
        .filter(|&x| *x != val)
        .cloned()
        .collect::<Vec<i32>>();

    // Copy elements from the output vector to the original vector
    // without causing length changes to the original vector.
    for i in 0..output.len() {
        nums[i] = output[i];
    }

    // Remove elements from the end of the vector to prevent copying
    for i in (output.len()..nums.len()).rev() {
        nums.remove(i);
    }

    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![3, 2, 2, 3];
        let expected_nums = vec![2, 2];
        let length = remove_element(&mut nums, 3);
        assert_eq!(length, expected_nums.len() as i32);
        assert_eq!(nums, expected_nums);
    }

    #[test]
    fn it_works2() {
        let mut nums = vec![1];
        let expected_nums = vec![];
        let length = remove_element(&mut nums, 1);
        assert_eq!(length, expected_nums.len() as i32);
        assert_eq!(nums, expected_nums);
    }
}
