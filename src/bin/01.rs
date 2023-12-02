use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(1);

fn add_digit(digits: &mut Vec<String>, new_digit: String) {
    if digits.is_empty() {
        digits.push(new_digit.clone());
        digits.push(new_digit.clone());
        return;
    }
    if digits.len() == 2 {
        digits.pop();
    }
    digits.push(new_digit.clone());
}

// split input by newline
// inspect each line
// for each line, iterate through each character
// for each character, check if it is a digit
// if it is, store it as:
// 1) the first digit (if it's the first digit)
// 2) the last digit (the most recent last digit)
// after all characters are inspected, combine the first and last digit to get the answer for that line
// sum all of the digits for each line to get the result
pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split('\n');
    let all_digits_summed: u32 = lines
        .map(|l| -> u32 {
            let characters = l.chars();
            let mut digits: Vec<String> = vec![];
            for c in characters {
                if !c.is_numeric() {
                    continue;
                }
                add_digit(&mut digits, c.to_string());
            }
            digits.join("").parse::<u32>().unwrap()
        })
        .sum();
    Some(all_digits_summed)
}

fn get_digit_from_word(word: &str) -> Option<&str> {
    let word_to_digit_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    return word_to_digit_map.get(word).copied();
}

fn extract_digit_and_update_curr_substring(curr_substring: &mut String) -> String {
    while get_digit_from_word(curr_substring).is_none() {
        // pop left-most character
        let chars = &mut curr_substring.chars();
        chars.next();
        *curr_substring = chars.as_str().to_string();
    }
    let digit = get_digit_from_word(&curr_substring.clone())
        .unwrap()
        .to_string();
    digit.to_string()
}

// sliding window?
// build a substring until a digit is encountered
// once a digit is encountered, check the substring for all occurrences of digits
pub fn part_two(input: &str) -> Option<u32> {
    let digits_re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let lines = input.split('\n');
    let all_digits_summed: u32 = lines
        .map(|l| -> u32 {
            let characters = l.chars();
            let mut digits: Vec<String> = vec![];
            let mut curr_substring: String = "".to_string();
            for c in characters {
                if c.is_numeric() {
                    add_digit(&mut digits, c.to_string());
                    continue;
                }

                curr_substring += &c.to_string();

                while digits_re.is_match(&curr_substring) {
                    let digit = extract_digit_and_update_curr_substring(&mut curr_substring);
                    add_digit(&mut digits, digit);
                    // pop left-most character
                    let mut chars = curr_substring.chars();
                    chars.next();
                    curr_substring = chars.as_str().to_string();
                }
            }
            digits.join("").parse::<u32>().unwrap()
        })
        .sum();
    Some(all_digits_summed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
