pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut return_vector = Vec::with_capacity(digits.len());
    let mut carry: bool = true;
    for i in (0..(digits.len())).rev() {
        let mut digit = digits[i];
        if carry {
            digit += 1;
        }
        if digit > 9 {
            carry = true;
            digit -= 10;
        } else {
            carry = false;
        }
        return_vector.push(digit);
    }
    if carry {
        return_vector.push(1);
    }

    return_vector.reverse();
    return_vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        // example_1

        let digits = vec![1, 2, 3];
        let expected = vec![1, 2, 4];
        assert_eq!(plus_one(digits), expected);
        // example_2

        let digits = vec![4, 3, 2, 1];
        let expected = vec![4, 3, 2, 2];
        assert_eq!(plus_one(digits), expected);
        // example_3

        let digits = vec![9];
        let expected = vec![1, 0];
        assert_eq!(plus_one(digits), expected);
    }
}
