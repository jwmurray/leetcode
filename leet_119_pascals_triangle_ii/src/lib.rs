struct Solution;

use std::cmp::min;

impl Solution {
    fn combinations(n: u64, r: u64) -> u64 {
        // Use the symmetry C(n, r) = C(n, n-r) to minimize calculations
        let k = min(r, n - r);

        // C(n, k) = product_{i=1 to k} (n - i + 1) / i
        if k == 0 {
            return 1;
        }

        // Use u128 for intermediate calculations to prevent overflow
        let mut res: u128 = 1;
        for i in 0..k {
            // The division is guaranteed to be exact here
            res = res * (n as u128 - i as u128) / (i as u128 + 1);
        }

        // Final result fits in u64 (and even i32 for this problem's constraints)
        res as u64
    }

    pub fn get_row(row_index: i32) -> Vec<i32> {
        // Row(n,i) = combinations(n,i)
        let n = row_index as u64; // Use u64 for n
        (0..=n) // Iterate up to n (inclusive)
            .map(|r| Solution::combinations(n, r) as i32) // Calculate combinations for each r
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact() {
        assert_eq!(Solution::combinations(4, 2), 6);
        // let fact_0 = Solution::fact(0);
        // println!("fact:0: {}", fact_0);
        // assert_eq!(Solution::fact(0), fact_0);
        // assert_eq!(Solution::fact(3), 6);
        // assert_eq!(Solution::fact(5), 120);
        assert_eq!(Solution::combinations(4, 0), 1);
        assert_eq!(Solution::combinations(4, 1), 4);
        assert_eq!(Solution::combinations(4, 2), 6);
        assert_eq!(Solution::combinations(4, 3), 4);
        assert_eq!(Solution::combinations(4, 4), 1);
        // assert_eq!(combinations(4, 1), 6);
    }
    #[test]
    fn it_works() {
        let row_index = 0;
        let output = vec![1];
        assert_eq!(Solution::get_row(row_index), output);
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
        assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1]);
        assert_eq!(Solution::get_row(5), vec![1, 5, 10, 10, 5, 1]);
        assert_eq!(Solution::get_row(6), vec![1, 6, 15, 20, 15, 6, 1]);
        assert_eq!(Solution::get_row(7), vec![1, 7, 21, 35, 35, 21, 7, 1]);
        assert_eq!(Solution::get_row(8), vec![1, 8, 28, 56, 70, 56, 28, 8, 1]);
        assert_eq!(
            Solution::get_row(9),
            vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1]
        );
        assert_eq!(
            Solution::get_row(10),
            vec![1, 10, 45, 120, 210, 252, 210, 120, 45, 10, 1]
        );
        assert_eq!(
            Solution::get_row(11),
            vec![1, 11, 55, 165, 330, 462, 462, 330, 165, 55, 11, 1]
        );
        assert_eq!(
            Solution::get_row(12),
            vec![1, 12, 66, 220, 495, 792, 924, 792, 495, 220, 66, 12, 1]
        );
        assert_eq!(
            Solution::get_row(13),
            vec![
                1, 13, 78, 286, 715, 1287, 1716, 1716, 1287, 715, 286, 78, 13, 1
            ]
        );
        assert_eq!(
            Solution::get_row(14),
            vec![
                1, 14, 91, 364, 1001, 2002, 3003, 3432, 3003, 2002, 1001, 364, 91, 14, 1
            ]
        );
        assert_eq!(
            Solution::get_row(15),
            vec![
                1, 15, 105, 455, 1365, 3003, 5005, 6435, 6435, 5005, 3003, 1365, 455, 105, 15, 1
            ]
        );
        assert_eq!(
            Solution::get_row(16),
            vec![
                1, 16, 120, 560, 1820, 4368, 8008, 11440, 12870, 11440, 8008, 4368, 1820, 560, 120,
                16, 1
            ]
        );
        assert_eq!(
            Solution::get_row(17),
            vec![
                1, 17, 136, 680, 2380, 6188, 12376, 19448, 24310, 24310, 19448, 12376, 6188, 2380,
                680, 136, 17, 1
            ]
        );
        assert_eq!(
            Solution::get_row(18),
            vec![
                1, 18, 153, 816, 3060, 8568, 18564, 31824, 43758, 48620, 43758, 31824, 18564, 8568,
                3060, 816, 153, 18, 1
            ]
        );
    }
}
