struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        // swap adjacent bits
        let x = ((x & 0x55555555) << 1) | ((x & 0xaaaaaaaa) >> 1);
        // adjacent pairs
        let x = ((x & 0x33333333) << 2) | ((x & 0xcccccccc) >> 2);
        // swap adajcent nibbles
        let x = ((x & 0x0f0f0f0f) << 4) | ((x & 0xf0f0f0f0) >> 4);
        // swap bytes
        let x = ((x & 0x00ff00ff) << 8) | ((x & 0xff00ff00) >> 8);
        // swap u16s
        let x = ((x & 0x0000ffff) << 16) | ((x & 0xffff0000) >> 16);

        x
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        let input:u32 = 0x5;
        let expected:u32 = 0xa0000000;
        assert_eq!(Solution::reverse_bits(input), expected);
    }
    #[test]
    fn test_reverse_bit_pairs() {
        let input:u32 = 0x33;
        let expected:u32 = 0xcc000000;
        assert_eq!(Solution::reverse_bits(input), expected);
    }
    #[test]
    fn test_reverse_bits_nibbles() {
        let input:u32 = 0x12;
        let expected:u32 = 0x48000000;
        assert_eq!(Solution::reverse_bits(input), expected);
    }
    #[test]
    fn test_reverse_bits_bytes() {
        let input:u32 = 0x1234;
        let expected:u32 = 0x2c480000;
        assert_eq!(Solution::reverse_bits(input), expected);
    }
    #[test]
    fn test_reverse_bits_u16s() {
        let input:u32 = 0x12345678;
        let expected:u32 = 0x1e6a2c48;
        assert_eq!(Solution::reverse_bits(input), expected);
    }
}
