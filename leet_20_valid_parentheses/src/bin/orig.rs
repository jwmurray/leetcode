use std::collections::HashMap;

fn main() {
    let input = "()".to_string();
    assert_eq!(is_valid(input), true);
}

pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let mut brackets_map: HashMap<char, char> = HashMap::new();

    // closing brackets are saved as values, closing brackets are saved as keys
    // We will only need to use closing brackets as keys, so we only need a one-way map.
    // However, we will use the values to verify whether an item is an opening bracket.
    brackets_map.insert(')', '(');
    brackets_map.insert('}', '{');
    brackets_map.insert(']', '[');

    for ch in s.chars() {
        if brackets_map.values().any(|&val| val == ch) {
            // Check whether ch is an opening bracket
            // If it's an opening bracket, push to stack
            stack.push(ch);
        } else if brackets_map.contains_key(&ch) {
            //
            // If it's a closing bracket
            // Check if stack is empty or if the top doesn't match expected opening bracket
            if stack.is_empty() || stack.pop().unwrap() != *brackets_map.get(&ch).unwrap() {
                return false;
            }
        }
        // If it's not a bracket, we ignore the character as our function is only verifying brackets
    }

    // Valid only if all brackets are matched (stack is empty)
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let input = "()".to_string();
        assert_eq!(is_valid(input), true);
    }

    #[test]
    fn test_is_valid2() {
        let input = "()[]{}".to_string();
        assert_eq!(is_valid(input), true);
    }

    #[test]
    fn test_is_valid3() {
        let input = "(}".to_string();
        assert!(is_valid(input) == false);
    }

    #[test]
    fn test_is_valid4() {
        let input = "([])".to_string();
        assert_eq!(is_valid(input), true);
    }
}
