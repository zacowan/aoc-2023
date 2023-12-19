use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(10);

type Direction = (i8, i8);

const NORTH: Direction = (-1, 0);
const SOUTH: Direction = (1, 0);
const EAST: Direction = (0, 1);
const WEST: Direction = (0, -1);

const DIRECTIONS: [Direction; 4] = [NORTH, SOUTH, EAST, WEST];

const START: char = 'S';

// Checks if it is possible to travel to pipe based on the direction you took to arrive there
fn is_connected(from_dir: Direction, pipe: char) -> bool {
    match pipe {
        '|' => from_dir == NORTH || from_dir == SOUTH,
        '-' => from_dir == EAST || from_dir == WEST,
        'L' => from_dir == SOUTH || from_dir == WEST,
        'J' => from_dir == SOUTH || from_dir == EAST,
        '7' => from_dir == NORTH || from_dir == EAST,
        'F' => from_dir == NORTH || from_dir == WEST,
        _ => false, // ground
    }
}

fn add_within_bounds(from: (usize, usize), dir: Direction, n: usize) -> Option<(usize, usize)> {
    let to_r = from.0 as i32 + dir.0 as i32;
    let to_c = from.1 as i32 + dir.1 as i32;

    if to_r < 0 || to_r >= n as i32 || to_c < 0 || to_c >= n as i32 {
        None
    } else {
        Some((to_r as usize, to_c as usize))
    }
}

/// bfs, count distances
pub fn part_one(input: &str) -> Option<u32> {
    let mut start: (usize, usize) = (0, 0);
    let graph: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == START {
                        start = (i, j);
                    };
                    c
                })
                .collect()
        })
        .collect();
    let n = graph.len();

    let mut distances_from_start = HashMap::new();
    distances_from_start.insert(start, 0u32);
    let mut to_visit: VecDeque<(usize, usize)> = VecDeque::from([start]);

    while let Some(curr) = to_visit.pop_front() {
        DIRECTIONS.iter().for_each(|dir| {
            let Some(next) = add_within_bounds(curr, *dir, n) else {
                return;
            };
            if distances_from_start.get(&next).is_some()
                || !is_connected(*dir, graph[next.0][next.1])
            {
                return;
            };
            let distance_from_start = distances_from_start.get(&curr).unwrap() + 1;
            distances_from_start.insert(next, distance_from_start);
            to_visit.push_back(next);
        })
    }

    Some(*distances_from_start.values().max().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, None);
    }
}
