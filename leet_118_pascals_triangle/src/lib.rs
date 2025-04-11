struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        for i in 0..num_rows as usize {
            let mut row = vec![1; i + 1];
            if i > 1 {
                for j in 1..=((i + 1) / 2) {
                    let value = result[i - 1][j - 1] + result[i - 1][j];
                    (row[j], row[i - j]) = (value, value);
                }
            }
            result.push(row);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expected: Vec<Vec<i32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1]];

        assert_eq!(Solution::generate(3), expected);

        let expected: Vec<Vec<i32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(Solution::generate(5), expected);
    }
    #[test]
    fn test_1() {
        let expected: Vec<Vec<i32>> = vec![vec![1]];
        assert_eq!(Solution::generate(1), expected);
    }
}
