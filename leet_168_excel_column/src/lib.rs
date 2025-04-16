struct Solution;

impl Solution {
    ///  For the low order char, 0-25 -> A-Z
    /// For all other chars, 1-26 -> A-Z
    
    pub fn convert_to_title(column_number: i32) -> String {
        let mut n = column_number as i64;
        let mut result = String::new();
        
        while n > 0 {
            n -= 1;
            let ch = Solution::index_to_char((n % 26) as i32);
            result.insert(0, ch);
            n /= 26;
        }
        
        result
    }
    
    fn index_to_char(index: i32) -> char {
        // Use a const array of chars for O(1) lookups
        const CHARS: [char; 26] = ['A', 'B', 'C', 'D', 'E',
            'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
            'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
        ];
        CHARS[index as usize]
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let column_number: i32 = 1;
        let output =  "A".to_string();
        assert_eq!(Solution::convert_to_title(column_number), output);
        
        let column_number: i32 = 28;
        let output =  "AB".to_string();
        assert_eq!(Solution::convert_to_title(column_number), output);
    }

    #[test]
    fn test_2() {
        
        let column_number: i32 = 701;  // == 26*26 + 25
        let output =  "ZY".to_string();
        assert_eq!(Solution::convert_to_title(column_number), output);
        
        let column_number: i32 = 26*26+1;
        let output =  "ZA".to_string();
        assert_eq!(Solution::convert_to_title(column_number), output);
        
        let column_number: i32 = 26*26+26;
        let output =  "ZZ".to_string();
        assert_eq!(Solution::convert_to_title(column_number), output);
        
        // Test i32::MAX
        let column_number: i32 = i32::MAX;
        let output = "FXSHRXW".to_string();
        assert_eq!(Solution::convert_to_title(column_number), output);
        
        // let column_number: i32 = 26*26*26+0;
        // let output =  "ZZ".to_string();
        // assert_eq!(Solution::convert_to_title(column_number), output);
        
        // let column_number: i32 = 26*26+1;
        // let output =  "AAA".to_string();
        // assert_eq!(Solution::convert_to_title(column_number), output);
        
        
    }
}
