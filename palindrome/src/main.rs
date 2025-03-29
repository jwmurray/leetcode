fn main() {
    println!("Hello, world!");
}

pub fn is_palindrome(x: i32) -> bool {
    let mut num: u32 = x as u32;
    let mut buckets: Vec<u32> = Vec::new();
    while num > 0 {
        buckets.push(num % 10);
        num /= 10;
    }

    for i in 0..buckets.len() / 2 {
        if buckets[i] != buckets[buckets.len() - i - 1] {
            return false;
        }
    }
    true
}

pub fn is_palindrome2(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut num = x;
    let mut reversed = 0;

    while num > 0 {
        let digit = num % 10;
        reversed = reversed * 10 + digit;
        num /= 10;
    }

    x == reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(123), false);
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(1000021), false);
    }
}
