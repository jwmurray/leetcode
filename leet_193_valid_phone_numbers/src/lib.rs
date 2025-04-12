// Read from the file file.txt and output all valid phone numbers to stdout.
use regex::Regex;

pub fn read_file_to_strings(filepath: &str) -> Result<Vec<String>, std::io::Error> {
    let lines = std::fs::read_to_string(filepath)?;
    let regex_pattern = Regex::new(r"^((\(\d{3}\) )|(\d{3}-))\d{3}-\d{4}").unwrap();
    Ok(lines
        .lines()
        .map(|l| l.to_string())
        .filter(|s_ref| regex_pattern.is_match(s_ref))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        // file contents:
        //987-123-4567
        //123 456 7890
        //(123) 456-7890
        let result = read_file_to_strings("file.txt");
        assert_eq!(result.is_ok(), true);

        let expected = vec!["987-123-4567", "(123) 456-7890"];

        assert_eq!(result.unwrap(), expected);
    }
}

/*
Bash equivalent:
#!/bin/bash

validate_phone_numbers() {
    grep -E '^((\([0-9]{3}\) )|([0-9]{3}-))[0-9]{3}-[0-9]{4}$' file.txt
}

validate_phone_numbers
*/
