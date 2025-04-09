use std::ops::{Index, IndexMut};

pub struct BinaryVector {
    bits: Vec<u8>,
}

impl std::fmt::Display for BinaryVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .bits
            .iter()
            .rev() // for display, reverse the bit order
            .map(|&bit| if bit == 1 { '1' } else { '0' })
            .collect();
        write!(f, "{}", s)
    }
}

impl BinaryVector {
    pub fn from_string(s: &str) -> Self {
        let bits: Vec<u8> = s
            .chars()
            .rev() // Save bit string with low bit first
            .map(|c| if c == '1' { 1 } else { 0 })
            .collect();
        BinaryVector { bits }
    }
    pub fn reverse(&mut self) {
        self.bits.reverse();
    }
    pub fn new() -> Self {
        BinaryVector { bits: vec![] }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        BinaryVector {
            bits: Vec::with_capacity(capacity),
        }
    }
    fn get(&self, index: usize) -> Option<u8> {
        self.bits.get(index).copied()
    }
    fn push(&mut self, value: u8) {
        self.bits.push(value);
    }
}

impl Index<usize> for BinaryVector {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.bits[index]
    }
}

impl IndexMut<usize> for BinaryVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.bits[index]
    }
}

pub fn add_vector(va: BinaryVector, vb: BinaryVector) -> BinaryVector {
    let va_len = va.bits.len();
    let vb_len = vb.bits.len();
    let max_len = std::cmp::max(va_len, vb_len);
    let mut bv = BinaryVector::with_capacity(max_len);
    let mut carry: u8 = 0;
    for i in (0..max_len) {
        let mut sum: u8 = carry + va.get(i).unwrap_or_default() + vb.get(i).unwrap_or_default();
        if sum > 1 {
            carry = 1;
            sum -= 2;
        } else {
            carry = 0;
        }
        bv.push(sum);
    }
    if carry > 0 {
        bv.push(carry);
    }
    bv
}

pub fn add_binary(a: String, b: String) -> String {
    let va = BinaryVector::from_string(&a);
    let vb = BinaryVector::from_string(&b);
    let bv = add_vector(va, vb);
    bv.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_binary() {
        let s1 = "1010".to_string();
        let s2 = "1011".to_string();
        let expected = "10101";
        assert_eq!(&add_binary(s1, s2), expected);
    }

    #[test]
    fn test_add_binary2() {
        let s1 = "11".to_string();
        let s2 = "1".to_string();
        let expected = "100";
        assert_eq!(&add_binary(s1, s2), expected);
    }
}
