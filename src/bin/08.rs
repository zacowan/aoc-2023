use std::collections::HashMap;

advent_of_code::solution!(8);

const INITIAL_COORDINATE: &str = "AAA";
const FINAL_COORDINATE: &str = "ZZZ";

enum Instruction {
    Left,
    Right,
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    let lines: Vec<&str> = input.lines().collect();
    let first_line = lines.first().unwrap().trim();
    first_line
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Failed to parse instruction {}", c),
        })
        .collect()
}

fn get_coordinates(input: &str) -> HashMap<String, (String, String)> {
    let coordinates: Vec<(String, (String, String))> = input
        .lines()
        .rev()
        .filter_map(|l| {
            let Some((curr, next)) = l.split_once('=') else {
                return None;
            };
            let binding = next.trim().replace(['(', ')'], "");
            let next = binding.split_once(',').unwrap();
            let (next_left, next_right) = next;
            Some((
                curr.trim().to_owned(),
                (next_left.trim().to_owned(), next_right.trim().to_owned()),
            ))
        })
        .collect();
    let mut hm = HashMap::new();
    coordinates.iter().for_each(|(a, b)| {
        hm.insert(a.to_owned(), b.to_owned());
    });
    hm
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = get_instructions(input);
    let coordinates = get_coordinates(input);
    let mut instruction_index = 0;
    let mut curr_coord = INITIAL_COORDINATE;
    let mut num_steps = 0;
    while curr_coord != FINAL_COORDINATE {
        let instruction = instructions.get(instruction_index).unwrap();
        let coordinate_options = coordinates.get(curr_coord).unwrap();
        let new_coordinate = match instruction {
            Instruction::Left => coordinate_options.0.as_str(),
            Instruction::Right => coordinate_options.1.as_str(),
        };
        curr_coord = new_coordinate;
        num_steps += 1;
        instruction_index += 1;
        if instruction_index == instructions.len() {
            instruction_index = 0;
        };
    }
    Some(num_steps)
}

fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd::euclid_u128(a, b)
}

/// LCM of the steps it takes for each coordinate to reach the end
pub fn part_two(input: &str) -> Option<u128> {
    let instructions = get_instructions(input);
    let coordinates = get_coordinates(input);
    let initial_coordinates: Vec<&str> = coordinates
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|s| s.as_str())
        .collect();

    let steps_to_reach_z: Vec<u128> = initial_coordinates
        .iter()
        .map(|coord| {
            let mut curr_coord: &str = coord;
            let mut num_steps = 0;
            let mut instruction_index = 0;
            while !curr_coord.ends_with('Z') {
                let instruction = instructions.get(instruction_index).unwrap();
                let coordinate_options = coordinates.get(curr_coord).unwrap();
                let new_coordinate = match instruction {
                    Instruction::Left => coordinate_options.0.as_str(),
                    Instruction::Right => coordinate_options.1.as_str(),
                };
                curr_coord = new_coordinate;
                num_steps += 1;
                instruction_index += 1;
                if instruction_index == instructions.len() {
                    instruction_index = 0;
                };
            }
            num_steps
        })
        .collect();

    steps_to_reach_z.into_iter().reduce(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
