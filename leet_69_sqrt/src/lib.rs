pub fn get_sqrt_first_group(x: i32) -> (i32, i32) {
    let mut first_group = x;
    let mut ten_power = 1;
    while first_group > 99 {
        first_group /= 100;
        ten_power *= 10;
    }
    (first_group, ten_power)
}

pub fn get_sqrt_under_100(x: i32) -> i32 {
    for i in 1..10 {
        if x / i == i {
            return i;
        }
        if x / i < i {
            return i - 1;
        }
    }
    0
}

pub fn my_sqrt(x: i32) -> i32 {
    let (first_group, ten_power) = get_sqrt_first_group(x);
    let sqrt_seed = get_sqrt_under_100(first_group) * ten_power;
    // println!("sqrt_seed: {}", sqrt_seed);

    // Check if sqrt_seed is already the correct answer
    if sqrt_seed == 0 {
        if x == 0 {
            return 0;
        }
    } else { 
        if x / sqrt_seed == sqrt_seed {
            return sqrt_seed;
        }
    }

    // println!("Starting loop from {} to {}", sqrt_seed, x / 2);
    for i in (sqrt_seed)..=(x / 2 + 1) {
        if i == 0 {
            if x == 0 {
                return 0;
            }
            continue;
        }
        if x / i == i {
            return i;
        }
        if x / i < i {
            return i - 1;
        }
    }
    x / 2 + 1 // should never reach this code.
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(my_sqrt(2147483647), 46340);
        assert_eq!(my_sqrt(2), 1);
        assert_eq!(my_sqrt(0), 0);
        assert_eq!(my_sqrt(1), 1);
        assert_eq!(my_sqrt(4), 2);
        assert_eq!(my_sqrt(5), 2);
        assert_eq!(my_sqrt(9), 3);
        assert_eq!(my_sqrt(10), 3);
        assert_eq!(my_sqrt(2147395600), 46340);
        assert_eq!(my_sqrt(900), 30);
        assert_eq!(my_sqrt(901), 30);
        assert_eq!(my_sqrt(950), 30);
        assert_eq!(my_sqrt(960), 30);
        assert_eq!(my_sqrt(961), 31);
        assert_eq!(my_sqrt(962), 31);
    }

    #[test]
    fn test_get_first_group() {
        assert_eq!(get_sqrt_first_group(100), (1, 10));
        assert_eq!(get_sqrt_first_group(10), (10, 1));
        assert_eq!(get_sqrt_first_group(99), (99, 1));
    }
}
