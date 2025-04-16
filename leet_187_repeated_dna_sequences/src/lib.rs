struct Solution;

use std::collections::HashMap;

struct DNASequence {
    sequence: String,
}

struct DNASequenceIter<'a> {
    sequence: &'a str,
    current_pos: usize,
}

impl DNASequence {
    fn new(sequence: String) -> Self {
        Self { sequence }
    }

    fn iter(&self) -> DNASequenceIter {
        DNASequenceIter {
            sequence: &self.sequence,
            current_pos: 0,
        }
    }
}

impl<'a> Iterator for DNASequenceIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        const SEQ_LEN: usize = 10;
        if self.current_pos + SEQ_LEN <= self.sequence.len() {
            let slice = self.sequence[self.current_pos..self.current_pos + SEQ_LEN].to_string();
            self.current_pos += 1;
            Some(slice)
        } else {
            None
        }
    }
}


impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut return_vector = Vec::new();
        let mut counts: HashMap<String, usize> = HashMap::new();
        let dna = DNASequence::new(s);
        
        // Single pass: count and collect
        for seq in dna.iter() {
            match counts.get_mut(&seq) {
                Some(count) => {
                    *count += 1;
                    if *count == 2 {
                        return_vector.push(seq);
                    }
                },
                None => {
                    counts.insert(seq, 1);
                }
            }
        }
        
        return_vector
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string();
        let dna = DNASequence::new(s);
        let mut iter = dna.iter();
        assert_eq!(iter.next(), Some("AAAAACCCCC".to_string()));
        assert_eq!(iter.next(), Some("AAAACCCCCA".to_string()));
    }

    #[test]
    fn test_dna_to_hash() {
        let s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string();
        let dna = DNASequence::new(s);
        let mut counts: HashMap<String, usize> = HashMap::new();
        
        for seq in dna.iter() {
            *counts.entry(seq).or_insert(0) += 1;
        }
        
        assert_eq!(counts.len(), 21); // There should be 21 unique 10-character windows
    }

    #[test]
    fn it_works() {
        let s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string();
        let mut expected = ["AAAAACCCCC".to_string(),"CCCCCAAAAA".to_string()];
        let mut result = Solution::find_repeated_dna_sequences(s);
        expected.sort();
        result.sort();
        assert_eq!(result, expected);
    }
    #[test]
    fn it_works_2() {
        let s = "AAAAAAAAAAAAA".to_string();
        let expected = ["AAAAAAAAAA".to_string()];
        assert_eq!(Solution::find_repeated_dna_sequences(s), expected);
    }
}
