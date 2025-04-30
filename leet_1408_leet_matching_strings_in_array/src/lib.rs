struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut result_hash: HashSet<String> = HashSet::with_capacity(words.len());
        let mut words = words.clone();
        words.sort_by(|a, b| b.len().cmp(&a.len()));

        for (i, word) in words.iter().enumerate() {
            for sub_word in words[(i + 1)..].iter() {
                if word.len() <= sub_word.len() {
                    continue;
                }
                if word.contains(sub_word) {
                    result_hash.insert(sub_word.to_owned());
                }
            }
        }

        let mut result: Vec<String> = result_hash.into_iter().collect::<Vec<String>>();
        result.sort();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_substring() {
        let word = "superhero".to_string();
        let sub_word = "hero";
        assert_eq!(word.contains(sub_word), true);
    }
    #[test]
    fn it_works() {
        let words: Vec<String> = vec!["mass", "as", "hero", "superhero"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut expected: Vec<String> = vec!["as", "hero"].iter().map(|s| s.to_string()).collect();
        expected.sort();
        let mut result = Solution::string_matching(words);
        result.sort();
        assert_eq!(result, expected);

        let words: Vec<String> = vec!["leetcode", "et", "code"]
            .iter() // placeholder
            .map(|s| s.to_string())
            .collect();
        let mut expected: Vec<String> = vec!["et", "code"]
            .iter() // placeholder
            .map(|s| s.to_string())
            .collect();
        expected.sort();
        let mut result = Solution::string_matching(words);
        result.sort();
        assert_eq!(result, expected);

        let words: Vec<String> = vec!["blue", "green", "bu"]
            .iter() // placeholder
            .map(|s| s.to_string())
            .collect();
        let mut expected: Vec<String> = vec![]
            .iter() // placeholder
            .map(|s: &&str| s.to_string())
            .collect();
        expected.sort();
        let mut result = Solution::string_matching(words);
        result.sort();
        assert_eq!(result, expected);
    }
}
