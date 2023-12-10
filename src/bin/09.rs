use itertools::*;

advent_of_code::solution!(9);

fn sequence_diff(sequence: Vec<i64>) -> Vec<i64> {
    let mut sequence_diff: Vec<i64> = vec![];
    let mut i = 0;
    while i < sequence.len() - 1 {
        let diff = sequence[i + 1] - sequence[i];
        sequence_diff.push(diff);
        i += 1;
    }
    sequence_diff
}

pub fn part_one(input: &str) -> Option<i64> {
    let lines = input.lines();
    let histories = lines.map(|l| {
        l.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>()
    });
    let result = histories
        .map(|x| {
            let mut sequences = vec![x.clone()];
            let mut curr_sequence = x;
            while !curr_sequence
                .iter()
                .all_equal_value()
                .is_ok_and(|x| *x == 0)
            {
                let new_sequence = sequence_diff(curr_sequence);
                sequences.push(new_sequence.clone());
                curr_sequence = new_sequence;
            }

            let mut num = 0;
            let mut i = sequences.len() - 1;
            let mut j = sequences[i].len() - 1;
            while i > 0 {
                sequences[i].push(num);
                i -= 1;
                j += 1;
                num += sequences[i][j];
            }

            num
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<i64> {
    let lines = input.lines();
    let histories = lines.map(|l| {
        l.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>()
    });
    let result = histories
        .map(|mut x| {
            x.reverse();
            let mut sequences = vec![x.clone()];
            let mut curr_sequence = x;
            while !curr_sequence
                .iter()
                .all_equal_value()
                .is_ok_and(|x| *x == 0)
            {
                let new_sequence = sequence_diff(curr_sequence);
                sequences.push(new_sequence.clone());
                curr_sequence = new_sequence;
            }

            let mut num = 0;
            let mut i = sequences.len() - 1;
            let mut j = sequences[i].len() - 1;
            while i > 0 {
                sequences[i].push(num);
                i -= 1;
                j += 1;
                num += sequences[i][j];
            }

            num
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
