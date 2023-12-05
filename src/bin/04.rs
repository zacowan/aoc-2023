use std::cmp;
use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Clone)]
struct Scratchcard {
    num_wins: u32,
}

fn extract_numbers(numbers: &str) -> Vec<u32> {
    numbers
        .split(' ')
        .filter_map(|n| match n.trim().parse() {
            Ok(number) => Some(number),
            Err(_) => None,
        })
        .collect()
}

fn calculate_num_wins(winning_numbers: &[u32], scratch_numbers: &[u32]) -> u32 {
    scratch_numbers
        .iter()
        .map(|n| match winning_numbers.contains(n) {
            true => 1,
            false => 0,
        })
        .sum()
}

fn build_scratchcard_from_card_line(card_line: &str) -> Scratchcard {
    let card_and_numbers: Vec<&str> = card_line.split(": ").collect();
    let numbers = card_and_numbers.get(1).unwrap();
    let numbers: Vec<&str> = numbers.split('|').map(|n| n.trim()).collect();

    let winning_numbers = extract_numbers(numbers.first().unwrap());
    let scratch_numbers = extract_numbers(numbers.get(1).unwrap());
    let num_wins = calculate_num_wins(&winning_numbers, &scratch_numbers);

    Scratchcard { num_wins }
}

fn get_scratchcard_points(scratchcard: Scratchcard) -> u32 {
    if scratchcard.num_wins == 0 {
        return 0;
    }
    2_u32.pow(scratchcard.num_wins.saturating_sub(1))
}

/// First match = 1 point
/// Each additional match = points * 2
pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(build_scratchcard_from_card_line)
            .map(get_scratchcard_points)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let original_scratchcards: Vec<Scratchcard> = input
        .lines()
        .map(build_scratchcard_from_card_line)
        .collect();

    let mut scratchcard_copy_counts: HashMap<usize, u32> = HashMap::new();
    for (index, _) in original_scratchcards.iter().enumerate() {
        scratchcard_copy_counts.insert(index, 0);
    }

    for (index, scratchcard) in original_scratchcards.iter().enumerate() {
        let num_copies = scratchcard_copy_counts.get(&index).unwrap() + 1;
        let last_card_to_clone: usize = cmp::min(
            (u32::try_from(index).unwrap() + scratchcard.num_wins)
                .try_into()
                .unwrap(),
            original_scratchcards.len() - 1,
        );
        for i in (index + 1)..=last_card_to_clone {
            let existing_copies = scratchcard_copy_counts.get(&i).unwrap();
            scratchcard_copy_counts.insert(i, existing_copies + num_copies);
        }
    }

    let num_original_scratchcards: u32 = original_scratchcards.len().try_into().unwrap();
    let num_scratchcard_copies: u32 = scratchcard_copy_counts.values().sum();

    Some(num_original_scratchcards + num_scratchcard_copies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
