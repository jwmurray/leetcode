use std::cmp::max;

fn to_vector(s: &str) -> Vec<u32> {
    let mut return_vector: Vec<u32> = vec![];

    for ch in s.chars() {
        match ch {
            '0' => return_vector.push(0),
            '1' => return_vector.push(1),
            _ => continue,
        }
    }
    return_vector
}

pub fn add_strings(a: String, b: String) -> String {
    let v1 = to_vector(&a);
    let v2 = to_vector(&b);
    let vector_sum = add_vectors(&v1, &v2);
    to_binary_string(&vector_sum)
}

pub fn add_vectors(v1: &Vec<u32>, v2: &Vec<u32>) -> Vec<u32> {
    let capacity = max(v1.len(), v2.len());
    let mut return_vector = Vec::with_capacity(capacity);
    let mut carry: u32 = 0;

    for i in (0..capacity).rev() {
        let mut digit = carry;
        if let Some(digit_v1) = v1.get(i) {
            digit += digit_v1;
        }
        if let Some(digit_v2) = v2.get(i) {
            digit += digit_v2;
        }
        if digit > 1 {
            carry = 1;
            digit -= 2;
        } else {
            carry = 0;
        }
        return_vector.push(digit);
    }
    if carry > 0 {
        return_vector.push(carry);
    }
    return_vector.reverse();
    return_vector
}

pub fn to_binary_string(vector: &Vec<u32>) -> String {
    let mut return_string: String = String::new();

    for digit in vector {
        match digit {
            0 => return_string.push('0'),
            1 => return_string.push('1'),
            _ => continue,
        }
    }
    return_string
}

pub fn add_binary(a: String, b: String) -> String {
    let num1 = u32::from_binary_string(a);
    let num2 = u32::from_binary_string(b);
    (num1 + num2).to_binary_string()
}

pub fn add_binary1(a: String, b: String) -> String {
    let num1 = u32::from_binary_string(a);
    let num2 = u32::from_binary_string(b);
    (num1 + num2).to_binary_string()
}

trait Binary {
    fn from_binary_string(s: String) -> Self;
    fn to_binary_string(&self) -> String;
}

impl Binary for u32 {
    fn from_binary_string(s: String) -> Self {
        u32::from_str_radix(&s, 2).unwrap_or(0)
    }

    fn to_binary_string(&self) -> String {
        format!("{:b}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vector() {
        let s1 = "1010".to_string();
        let s2 = "1011".to_string();
        let expected = "10101";
        assert_eq!(&add_binary(s1, s2), expected);
    }

    #[test]
    fn test_add_vector2() {
        let v1: Vec<u32> = vec![1, 1, 0, 0];
        let v2: Vec<u32> = vec![1, 1, 1, 1];
        let v_sum = add_vectors(&v1, &v2);
        assert_eq!(v_sum, vec![1, 1, 0, 1, 1])
    }
    #[test]
    fn test_to_vector() {
        let s = "110".to_string();
        let expected = vec![1, 1, 0];
        assert_eq!(to_vector(&s), expected);
    }

    #[test]
    fn test_to_binary_string() {
        let vector: Vec<u32> = vec![1, 1, 0];
        let expected = "110".to_string();
        assert_eq!(to_binary_string(&vector), expected);
    }

    #[test]
    fn test_add() {
        let s1 = "11".to_string();
        let s2 = "1".to_string();
        let result = add_binary(s1, s2);
        assert_eq!(result, "100");
    }

    #[test]
    fn test_binary_conversion() {
        let binary = "1010".to_string();
        let num = u32::from_binary_string(binary);
        assert_eq!(num, 10);

        let num: u32 = 4;
        let converted = num.to_binary_string();
        assert_eq!(converted, "100");
    }
}
