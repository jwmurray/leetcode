// Given an array nums, you can perform the following operation any number of times:

//     Select the adjacent pair with the minimum sum in nums. If multiple such pairs exist, choose the leftmost one.
//     Replace the pair with their sum.

// Return the minimum number of operations needed to make the array non-decreasing.

// An array is said to be non-decreasing if each element is greater than or equal to its previous element (if it exists).

// Example 1:
// Input: nums = [5,2,3,1]
// Output: 2
// Explanation:
//     The pair (3,1) has the minimum sum of 4. After replacement, nums = [5,2,4].
//     The pair (2,4) has the minimum sum of 6. After replacement, nums = [5,6].
// The array nums became non-decreasing in two operations.

// Example 2:
// Input: nums = [1,2,2]
// Output: 0
// Explanation:
// The array nums is already sorted.

// Constraints:
//     1 <= nums.length <= 50
//     -1000 <= nums[i] <= 1000

pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
    // Return true if each adjacent pair in list pair.left <= pair.right
    // Return false if for any adjacent pair in the list, pair.left > pair.right, ie, if any pair is_decreasing()
    fn is_not_decreasing(vector: &Vec<i32>) -> bool {
        for i in 0..(vector.len() - 1) {
            if vector[i] > vector[i + 1] {
                return false;
            }
        }
        true
    }

    fn remove_left_most_minimum_pair(nums: &mut Vec<i32>) {
        let mut minimum_sum = nums[0] + nums[1]; // current min sum
        let mut minimum_sum_index = 0; // index of minimum sum

        for i in 1..(nums.len() - 1) {
            let candidate_min = nums[i] + nums[i + 1];
            if candidate_min < minimum_sum {
                // Only modify the
                minimum_sum = candidate_min;
                minimum_sum_index = i;
            }
        }
        nums[minimum_sum_index] = minimum_sum;
        nums.remove(minimum_sum_index + 1);
    }

    // Takes the index of the first element of the pair to join
    // Returns the Some(index) of the next pair to join
    // Returns None if no pair is found
    let mut nums = nums.clone(); // Clone the input and modify that

    let mut count = 0;
    while !is_not_decreasing(&nums) {
        // While we do not have a non-decreasing list, remove the left-most lowest sum pair
        remove_left_most_minimum_pair(&mut nums);
        count += 1;
    }
    count
}

pub fn is_not_decreasing(vector: &Vec<i32>) -> bool {
    if vector.len() < 2 {
        return true;
    }

    for i in 0..(vector.len() - 1) {
        if vector[i] > vector[i + 1] {
            return false;
        }
    }
    true
}

pub fn remove_left_most_minimum_pair(nums: &mut Vec<i32>) {
    let mut minimum_sum = nums[0] + nums[1]; // current min sum
    let mut minimum_sum_index = 0; // index of minimum sum
    let not_decreasing: bool = true;

    for i in 1..(nums.len() - 1) {
        let candidate_min = nums[i] + nums[i + 1];
        if candidate_min < minimum_sum {
            // Only modify the
            minimum_sum = candidate_min;
            minimum_sum_index = i;
        }
    }
    nums[minimum_sum_index] = minimum_sum;
    nums.remove(minimum_sum_index + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_pair_if_increasing_2() {
        let mut nums = vec![5, 2, 3, 1];
        let mut mvector = nums.clone();
        remove_left_most_minimum_pair(&mut mvector);
        assert_eq!(mvector, vec![5, 2, 4]);

        let mut mvector = nums.clone();
        remove_left_most_minimum_pair(&mut mvector);
        assert_eq!(mvector, vec![5, 2, 4]);
        remove_left_most_minimum_pair(&mut mvector);
        assert_eq!(mvector, vec![5, 6]);

        let mvector = nums.clone();
        let turns = minimum_pair_removal(nums);
        assert_eq!(turns, 2);

        let mvector = vec![2, 2, -1, 3, -2, 2, 1, 1, 1, 0, -1];
        let turns = minimum_pair_removal(mvector);
        assert_eq!(turns, 9);
    }

    #[test]
    fn test_minimum_pair_removal() {
        let vector = vec![2];
        let turns = minimum_pair_removal(vector);
        assert_eq!(turns, 0);

        let vector = vec![1, 2, 2];
        let turns = minimum_pair_removal(vector);
        assert_eq!(turns, 0);

        let vector = vec![3, 2];
        let turns = minimum_pair_removal(vector);
        assert_eq!(turns, 1);

        let vector = vec![5, 2, 3, 1];
        let turns = minimum_pair_removal(vector);
        assert_eq!(turns, 2);

        let vector = vec![2, 2, -1, 3, -2, 2, 1, 1, 1, 0, -1];
        let turns = minimum_pair_removal(vector);
        assert_eq!(turns, 9);
    }
    #[test]
    fn test_is_increasing() {
        assert_eq!(is_not_decreasing(&vec![1, 1]), true);
        assert_eq!(is_not_decreasing(&vec![1]), true);
        assert_eq!(is_not_decreasing(&vec![]), true);
        assert_eq!(is_not_decreasing(&vec![1, 0]), false);
        assert_eq!(is_not_decreasing(&vec![1, 2, 2]), true);
        assert_eq!(is_not_decreasing(&vec![5, 2, 3, 1]), false);
    }
}
