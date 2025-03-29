use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target sum to find
    #[arg(short, long, default_value = "9")]
    target: i32,

    /// Numbers to search through (comma-separated)
    #[arg(short, long, default_value = "2,7,11,15")]
    numbers: String,
}

fn main() {
    let args = Args::parse();

    // Parse the comma-separated numbers string into a Vec<i32>
    let nums: Vec<i32> = args
        .numbers
        .split(',')
        .map(|s| s.trim().parse().expect("Failed to parse number"))
        .collect();

    let vec = two_sum(nums, args.target); 
    println!("{:?}", vec);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hashmap: HashMap<i32, usize> = HashMap::new();
    for (index, &addend) in nums.iter().enumerate() {
        let complement = target - addend;
        if let Some(&complement_index) = hashmap.get(&complement) {
            return vec![complement_index as i32, index as i32];
        }
        hashmap.insert(addend, index);
    }
    panic!("no match found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_324() {
        // Example 2:

        let nums = vec![3, 2, 4];
        let target = 6;
        let output = vec![1, 2];
        assert_eq!(two_sum(nums, target), output);
    }

    #[test]
    fn test_two_sum() {
        // Example 1
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let vec = two_sum(nums, target);
        let output = vec![0, 1];
        assert_eq!(vec, output);

        // Example 3:

        let nums = vec![3, 3];
        let target = 6;
        let output = vec![0, 1];
        assert_eq!(two_sum(nums, target), output);

        // Example 4:

        let nums = vec![0, 4, 3, 0];
        let target = 0;
        let output = vec![0, 3];
        assert_eq!(two_sum(nums, target), output);

        // Example 5:
        let nums = vec![-1, -2, -3, -4, -5];
        let target = -8;
        let output = vec![2, 4];
        assert_eq!(two_sum(nums, target), output);

        // Example 6:
        let nums = vec![6, 5, 7, 8, 9, 3];
        let target = 10;
        let output = vec![2, 5];
        assert_eq!(two_sum(nums, target), output);
    }
}
