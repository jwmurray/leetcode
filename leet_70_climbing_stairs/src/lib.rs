const fn precompute_fib() -> [i32; 46] {
    let mut cache = [0; 46];
    cache[1] = 1;
    cache[2] = 2;
    let mut i = 3;
    while i < 46 {
        cache[i] = cache[i - 1] + cache[i - 2];
        i += 1;
    }
    cache
}

static CACHE: [i32; 46] = precompute_fib();

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            1 => 1, // Only one way: take 1 step
            2 => 2, // Two ways: 1+1 or 2
            _ => Self::climb_stairs(n - 1) + Self::climb_stairs(n - 2),
        }
    }

    pub fn climb_stairs_with_cache(n: i32) -> i32 {
        CACHE[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Example 1:
        let n = 2;
        let expected = 2;
        assert_eq!(Solution::climb_stairs(n), expected);
        // Explanation: There are two ways to climb to the top.
        // 1. 1 step + 1 step
        // 2. 2 steps

        // Example 2:

        let n = 3;
        let expected = 3;
        assert_eq!(Solution::climb_stairs(n), expected);
        // Explanation: There are three ways to climb to the top.
        // 1. 1 step + 1 step + 1 step
        // 2. 1 step + 2 steps
        // 3. 2 steps + 1 step

        let n = 4;
        let expected = 5;
        assert_eq!(Solution::climb_stairs(n), expected);
        // Explanation: There are three ways to climb to the top.
        // 1. 1 step + 1 step + 1 step + 1 step
        // 2. 1 step + 2 steps + 1 step
        // 3. 2 steps + 1 step + 1 step
        // 4. 1 step + 1 step + 2 steps
        // 5. 2 steps + 2 steps
    }
}
