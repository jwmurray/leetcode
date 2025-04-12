struct Solution;

// Create a const function to create an array of the weights for bytes
// So that we save a little runtime.
const fn hamming_byte_weight() -> [u8; 256] {
    let mut return_array = [0_u8; 256];
    let mut n: usize = 0;
    while n < 256 {
        // Calculate the weight of n, using shift_count as the counter for the loop:
        let mut weight: u8 = 0;
        let mut shift_count: usize = 0;

        while shift_count < 8 {
            let mask = 1_u8 << shift_count;
            if n as u8 & mask > 0 {
                weight += 1; // add 1 if the mask bit matches a bit in n
            }
            shift_count += 1;
        }
        return_array[n] = weight;
        n += 1;
    }
    return_array
}

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut weight: i32 = 0;
        let mut temp_n = n as u32;
        let hamming_byte_array = hamming_byte_weight();
        while temp_n > 0 {
            let last_byte: u8 = (temp_n & 0xFF) as u8; // Get the lowest 8 bits
            weight += hamming_byte_array[last_byte as usize] as i32;
            temp_n >>= 8; // Shift right by 8 bits to process the next byte
        }
        weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = hamming_byte_weight();

        assert_eq!(arr[1], 1);
        assert_eq!(arr[255], 8);

        assert_eq!(Solution::hamming_weight(2147483645), 30);
    }
}
