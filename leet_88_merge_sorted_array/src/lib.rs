struct Solution;

type Index = i32; // Create an Index alias.  Negative values are a sentinel that the buffer is exhausted.
// comparing the assembly, the trait solution uses slightly more code,but it makes it a
// little clearer how we are using the -1 from Index to show exhaustion of the buffers.
// This need comes up because the code has to walk the buffer in reverse order
// AND with the overlapping buffers in this contrived example, there is no good way
// to separate the buffers using iterators without using unsafe code, as two iterators
// would have writable references to the same buffer.
trait ToUsize {
    fn to_usize(&self) -> usize;
}

impl ToUsize for Index {
    fn to_usize(&self) -> usize {
        *self as usize
    }
}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // we have 3 buffers:
        // m, n, and tail
        // tail starts pointing at the last element of the overflow portion

        // Our first problem is that tail and m will overlap
        // Our second problem is that usize is used to index the buffers, but we want to use -1 as the indicator that we are done.

        // To handle this, we can cast the index to usize when it is used as an index.  Alternatively, we could use an Option<usize>

        let mut n_i: Index = n - 1; // n_i points to the last valid element of the n buffer.  -1 indicates that the buffer is used up.
        let mut t_i: Index = m + n - 1; // the t index is the last valid element of the tail buffer.  -1 indicates the buffer is used up.
        // This could be found by taking nums1.len() - 1, but m+n-1 saves a dereference.
        let mut m_i: Index = m - 1; // m index is the last valid element of the m buffer.  -1 indicates the buffer is used up.

        while m_i >= 0 && n_i >= 0 {
            // We are taking from both lists
            // Compare the next ending elements of the m and n buffers.
            // Place the greater of the two into the next slot of the tail.
            // Then, decrement the indices of the tail and the appropriate cmp buffer.
            if nums2[n_i.to_usize()] > nums1[m_i.to_usize()] {
                nums1[t_i.to_usize()] = nums2[n_i.to_usize()];
                n_i -= 1;
            } else {
                nums1[t_i.to_usize()] = nums1[m_i.to_usize()];
                m_i -= 1;
            }
            t_i -= 1;
        }
        // Now, the merge portion is complete.  We just finish copying from n.
        // If we have not finished the m buffer, that is OK as it is already
        // in the correct location
        while n_i >= 0 {
            nums1[t_i.to_usize()] = nums2[n_i.to_usize()];
            t_i -= 1;
            n_i -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_0() {
        // example_1

        let mut nums1 = vec![2, 3, 0];
        let m = 2;
        let mut nums2 = vec![1];
        let n = 1;
        let output = vec![1, 2, 3];
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, output);
    }
    #[test]
    fn test_example_01() {
        // example_1

        let mut nums1 = vec![1, 3, 0];
        let m = 2;
        let mut nums2 = vec![2];
        let n = 1;
        let output = vec![1, 2, 3];
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, output);
    }

    #[test]
    fn test_example_1() {
        // example_1

        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        let output = vec![1, 2, 2, 3, 5, 6];
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, output);
        // Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
        // The result of the merge is [1,2,2,3,5,6] with the underlined elements coming from nums1.
    }
    #[test]
    fn example_2() {
        // example_2

        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;
        let output = vec![1];
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, output);
        // Explanation: The arrays we are merging are [1] and [].
        // The result of the merge is [1].
    }
    #[test]
    fn example_3() {
        // example_3

        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;
        let output = vec![1];
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, output);
        // Explanation: The arrays we are merging are [] and [1].
        // The result of the merge is [1].
        // Note that because m = 0, there are no elements in nums1. The 0 is only there to ensure the merge result can fit in nums1.
    }
}
