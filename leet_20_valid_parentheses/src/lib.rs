use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    let hashmap: HashMap<char, char> = HashMap::from([(')', '('), ('}', '{'), (']', '[')]);

    for ch in s.chars() {
        if hashmap.values().any(|&val| val == ch) {
            stack.push(ch);
        } else if hashmap.contains_key(&ch) {
            // if stack.is_empty() || has÷hmap[&stack.pop().unwrap()] != ch {
            if stack.is_empty() || hashmap[&ch] != stack.pop().unwrap() {
                return false;
            }
        } else {
            continue; // Ignore non-bracket characters
        }

        // match ch {
        //     '(' | '{' | '[' => stack.push(ch),
        //     ')' | '}' | ']' => {
        //         if stack.is_empty() || hashmap[&stack.pop().unwrap()] != ch {
        //             return false;
        //         }
        //     }
        //     _ => continue,  // Ignore non-bracket characters
        // }
    }
    stack.is_empty()
}

pub fn is_valid_2(s: String) -> bool {
    let mut stack = Vec::new();

    let mut hashmap: HashMap<char, char> = HashMap::new();
    hashmap.insert('(', ')');
    hashmap.insert('{', '}');
    hashmap.insert('[', ']');

    for ch in s.chars() {
        if hashmap.values().any(|&val| val == ch) {
            stack.push(ch);
        } else if hashmap.contains_key(&ch) {
            if stack.is_empty() || hashmap[&ch] != stack.pop().unwrap() {
                return false;
            }
        } else {
            continue; // Ignore non-bracket characters
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid(String::from("{a;lkjsdf}")), true);
        assert_eq!(is_valid(String::from("{}")), true);
        assert_eq!(is_valid(String::from(r"{a;lkjaaa}\}")), false);
    }
}
