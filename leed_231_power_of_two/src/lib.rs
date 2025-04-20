struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        
        let mut temp_n = n as u32;
        let mut power_count = 0;
        let power = loop {
            if temp_n == 1 {
                break power_count;
            }
            temp_n = temp_n >> 1;  // increment
            power_count += 1;
        };

        if 1 << power_count == n {
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_power_of_two(2), true);
        assert_eq!(Solution::is_power_of_two(0), false);
        assert_eq!(Solution::is_power_of_two(32), true);
        assert_eq!(Solution::is_power_of_two(9), false);
    }
}
