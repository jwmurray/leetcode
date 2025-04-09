pub fn str_str(haystack: &str, needle: &str) -> i32 {
    haystack.find(&needle).map(|i| i as i32).unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_Example_1() {
        let haystack = "sadbutsad".to_string();
        let needle = "sad".to_string();
        let output = 0;
        let result = str_str(&haystack, &needle);
        assert_eq!(result, output);
    }

    #[test]
    fn test_Example_2() {
        let haystack = "leetcode".to_string();
        let needle = "leeto".to_string();
        let output = -1;
        let result = str_str(&haystack, &needle);
        assert_eq!(result, output);
    }
}
