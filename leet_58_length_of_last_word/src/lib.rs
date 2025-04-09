pub fn length_of_last_word(s: String) -> i32 {
    let mut word_length = 0;
    let mut in_last_word = false;
    for ch in s.chars().rev() {
        match ch {
            ' ' => {
                if in_last_word {
                    return word_length;
                }
            }

            _ => {
                // all non-spaces are assumed to be letters.
                in_last_word = true;
                word_length += 1;
            }
        }
    }
    word_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "Hello World".to_string();
        let output = 5;
        assert_eq!(length_of_last_word(s), output);
        // Explanation: The last word is "World" with length 5.
    }

    #[test]
    fn test_example_2() {
        let s = "   fly me   to   the moon  ".to_string();
        let output = 4;
        assert_eq!(length_of_last_word(s), output);
        // Explanation: The last word is "moon" with length 4.
    }

    #[test]
    fn test_example_3() {
        let s = "luffy is still joyboy".to_string();
        let output = 6;
        assert_eq!(length_of_last_word(s), output);
        // Explanation: The last word is "joyboy" with length 6.
    }
}
