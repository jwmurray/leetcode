struct Solution;

struct RangeIterator<'a> {
    slice: &'a [i32], // static range over which the iteration takes place
    // pos maintains the position of the next subslice to be returned.
    // if pos = slice.len(), then next() will return None.  Hence, pos will be
    // incremented to the index following the last non-None next() call.
    pos: usize,
}

impl<'a> RangeIterator<'a> {
    fn new(slice: &'a [i32]) -> Self {
        RangeIterator { slice, pos: 0 }
    }
}

impl<'a> Iterator for RangeIterator<'a> {
    // The iterator will yield slices representing contiguous ranges
    type Item = &'a [i32];

    fn next(&mut self) -> Option<Self::Item> {
        // Check if we've reached the end of the slice
        if self.pos >= self.slice.len() {
            return None;
        }

        // Record the start position
        let start_pos = self.pos;
        let mut end_pos = self.pos;

        // Find the end of the contiguous range
        while end_pos + 1 < self.slice.len() && self.slice[end_pos + 1] == self.slice[end_pos] + 1 {
            end_pos += 1;
        }

        // Update the iterator's position for the next call
        self.pos = end_pos + 1;

        // Return the slice representing the range
        Some(&self.slice[start_pos..=end_pos])
    }
}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        // Create the range iterator from the input vector slice
        let range_iter = RangeIterator::new(&nums);

        // Iterate over the sub-slices yielded by the iterator
        for sub_slice in range_iter {
            // Get the start and end values from the slice
            // .unwrap() is safe here because the iterator logic ensures non-empty slices
            let start = sub_slice.first().unwrap();
            let end = sub_slice.last().unwrap();

            // Format the string based on whether it's a single number or a range
            if start == end {
                result.push(format!("{}", start));
            } else {
                result.push(format!("{}->{}", start, end));
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iteration() {
        let nums = vec![4, 5, 7];
        let mut range = RangeIterator::new(&nums);
        assert_eq!(range.pos, 0);
        // Compare with slices &[i32]
        assert_eq!(range.next(), Some(&[4, 5] as &[i32]));
        assert_eq!(range.next(), Some(&[7] as &[i32]));
        assert_eq!(range.next(), None);
    }

    #[test]
    fn it_works() {
        let nums = vec![0, 1, 2, 4, 5, 7];
        let expected = vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()];
        assert_eq!(Solution::summary_ranges(nums), expected);
    }

    #[test]
    fn empty_input() {
        let nums = vec![];
        let expected: Vec<String> = vec![];
        assert_eq!(Solution::summary_ranges(nums), expected);
    }

    #[test]
    fn single_element() {
        let nums = vec![5];
        let expected = vec!["5".to_string()];
        assert_eq!(Solution::summary_ranges(nums), expected);
    }

    #[test]
    fn negative_numbers() {
        let nums = vec![-2, -1, 0, 2, 3, 5, 7, 8];
        let expected = vec![
            "-2->0".to_string(),
            "2->3".to_string(),
            "5".to_string(),
            "7->8".to_string(),
        ];
        assert_eq!(Solution::summary_ranges(nums), expected);
    }

    #[test]
    fn non_contiguous() {
        let nums = vec![0, 2, 4, 6, 8];
        let expected = vec![
            "0".to_string(),
            "2".to_string(),
            "4".to_string(),
            "6".to_string(),
            "8".to_string(),
        ];
        assert_eq!(Solution::summary_ranges(nums), expected);
    }

    #[test]
    fn large_range() {
        let nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let expected = vec!["0->9".to_string()];
        assert_eq!(Solution::summary_ranges(nums), expected);
    }
}
