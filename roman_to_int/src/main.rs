fn roman_to_int(s: &str) -> i32 {
    let mut result = 0;
    let mut prev_value = 0;

    for c in s.chars().rev() {
        let curr_value = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        if curr_value < prev_value {
            result -= curr_value;
        } else {
            result += curr_value;
        }
        prev_value = curr_value;
    }

    result
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int("III"), 3);
        assert_eq!(roman_to_int("IV"), 4);
        assert_eq!(roman_to_int("IX"), 9);
        assert_eq!(roman_to_int("LVIII"), 58);
        assert_eq!(roman_to_int("MCMXCIV"), 1994);
    }
}
