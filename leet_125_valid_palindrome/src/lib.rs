struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .chars()
            .filter(|ch| ch.is_alphanumeric())
            .map(|ch| ch.to_ascii_lowercase())
            .collect();

        let len = chars.len();
        let mut left_iter = chars[0..(len / 2)].iter();
        let mut right_iter = chars[((len + 1) / 2)..len].iter().rev();

        while let (Some(left), Some(right)) = (left_iter.next(), right_iter.next()) {
            if left != right {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_palindrome("aba".to_owned()), true);
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned()),
            true
        );
        assert_eq!(Solution::is_palindrome("race a car".to_owned()), false);
    }
}
