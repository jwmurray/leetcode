// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

// Example 1:
// Input: strs = ["flower","flow","flight"]
// Output: "fl"

// Example 2:
// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.

// Constraints:

// 1 <= strs.length <= 200
// 0 <= strs[i].length <= 200
// strs[i] consists of only lowercase English letters if it is non-empty.

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        // Find the shortest string length
        let min_len = strs.iter().map(|s| s.len()).min().unwrap();

        // Get the first string as reference
        let first = strs[0].as_str();

        // Find the first position where any string differs
        let prefix_len = (0..min_len)
            .take_while(|&i| strs.iter().all(|s| s.as_bytes()[i] == first.as_bytes()[i]))
            .count();

        first[..prefix_len].to_string()
    }

    pub fn longest_common_prefix2(strs: Vec<String>) -> String {
        struct StringArray<'a> {
            string_slices: Vec<(&'a str, usize)>,
        }

        impl<'a> StringArray<'a> {
            fn new(strings: Vec<&'a str>) -> Self {
                Self {
                    string_slices: strings.into_iter().map(|s| (s, 0)).collect(),
                }
            }
        }

        impl<'a> Iterator for StringArray<'a> {
            type Item = char;

            fn next(&mut self) -> Option<Self::Item> {
                let mut candidate_char = None;
                for (s, pos) in self.string_slices.iter_mut() {
                    let next_char = s[*pos..].chars().next();
                    if next_char.is_some() {
                        if candidate_char.is_none() {
                            candidate_char = next_char;
                        } else if candidate_char != next_char {
                            return None;
                        }
                        *pos += 1;
                    } else {
                        return None;
                    }
                }
                candidate_char
            }
        }

        let string_array = StringArray::new(strs.iter().map(|s| s.as_str()).collect());
        let mut return_string: String = String::new();
        for c in string_array {
            return_string.push(c);
        }
        return_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        // Example 1:
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let output = "fl".to_string();
        assert_eq!(Solution::longest_common_prefix(strs.clone()), output);
        assert_eq!(Solution::longest_common_prefix2(strs), output);

        //  Example 2:
        let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let output = "".to_string();
        assert_eq!(Solution::longest_common_prefix(strs.clone()), output);
        assert_eq!(Solution::longest_common_prefix2(strs), output);
    }

    #[test]
    fn test_iter_mut() {
        let x = &mut [1, 2, 4];
        for elem in x.iter_mut() {
            *elem += 2;
        }
        assert_eq!(x, &[3, 4, 6]);
    }
}
