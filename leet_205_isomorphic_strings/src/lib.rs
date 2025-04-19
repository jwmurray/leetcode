struct Solution;

use std::{any, collections::HashMap};

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut hash:HashMap<char,char> = HashMap::with_capacity(s.len());
        for (ch_s, ch_t) in s.chars().zip(t.chars()) {
            if hash.contains_key(&ch_s) {
                if  hash.get(&ch_s) != Some(&ch_t) {
                    return false;
                }
            }  else {
                if hash.values().any(|&v| v == ch_t) {
                    return false;
                }
            }
            hash.insert(ch_s, ch_t);
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works2() {
        let s = "badc".to_string();
        let t = "baba".to_string();
        let expected = false;
        assert_eq!(Solution::is_isomorphic(s,t),expected);
    }

    #[test]
    fn it_works() {
        let s = "egg".to_string();
        let t = "add".to_string();
        let expected = true;
        assert_eq!(Solution::is_isomorphic(s,t),expected);

        let s = "foo".to_string();
        let t = "bar".to_string();
        let expected = false;
        assert_eq!(Solution::is_isomorphic(s,t),expected);
        
        let s = "paper".to_string();
        let t = "title".to_string();
        let expected = true;
        assert_eq!(Solution::is_isomorphic(s,t),expected);
    }
}