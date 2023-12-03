advent_of_code::solution!(3);

#[derive(Clone)]
struct Node {
    value: char,
    is_adjacent_to_symbol: Option<bool>,
}

impl Node {
    fn set_adjacent_to_symbol(&mut self, value: bool) {
        self.is_adjacent_to_symbol = Some(value);
    }
}

fn build_node(value: char) -> Node {
    Node {
        value,
        is_adjacent_to_symbol: None,
    }
}

fn mutable_bfs_with_callback<F>(nodes: &mut [Vec<Node>], starting_node: (usize, usize), callback: F)
where
    F: Fn(&mut Node),
{
    let (i, j) = starting_node;
    for di in -1..2 {
        for dj in -1..2 {
            let row = nodes.get_mut((i as isize + di as isize) as usize);
            if row.is_none() {
                continue;
            }
            let node = row.unwrap().get_mut((j as isize + dj as isize) as usize);
            if node.is_none() {
                continue;
            }
            callback(node.unwrap());
        }
    }
}

fn mark_nodes_adjacent_to_symbol(nodes: Vec<Vec<Node>>) -> Vec<Vec<Node>> {
    let mut nodes_copy = nodes.clone();
    for (i, row) in nodes.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            if node.value == '.' || node.value.is_numeric() {
                continue;
            }

            mutable_bfs_with_callback(&mut nodes_copy, (i, j), |node| {
                node.set_adjacent_to_symbol(true);
            });
        }
    }
    nodes_copy
}

/// Get sum of part numbers.
///
/// Numbers are considered a "part number" if they are adjacent horizontally/vertically/diagonally.
pub fn part_one(input: &str) -> Option<u32> {
    // build the data structure
    let nodes: Vec<Vec<Node>> = input
        .lines()
        .map(|l| l.chars().map(build_node).collect())
        .collect();
    // traverse, mark spaces that are adjacent to a symbol
    let nodes = mark_nodes_adjacent_to_symbol(nodes);
    // traverse, get list of part numbers
    let mut part_numbers: Vec<u32> = vec![];
    for row in nodes {
        let mut curr_number: String = "".to_string();
        let mut is_curr_number_a_part_number = false;
        for node in row {
            if node.value.is_numeric() {
                curr_number += &node.value.to_string();
                if !is_curr_number_a_part_number && node.is_adjacent_to_symbol.is_some() {
                    is_curr_number_a_part_number = node.is_adjacent_to_symbol.unwrap();
                }
                continue;
            }

            if is_curr_number_a_part_number && !curr_number.is_empty() {
                let part_number: u32 = curr_number.parse().unwrap();
                part_numbers.push(part_number);
            }

            curr_number.clear();
            is_curr_number_a_part_number = false;
        }
        if is_curr_number_a_part_number && !curr_number.is_empty() {
            let part_number: u32 = curr_number.parse().unwrap();
            part_numbers.push(part_number);
        }
    }
    Some(part_numbers.iter().sum())
}

/// Get sum of gear ratios (gr = pn1 * pn2).
///
/// A "gear" is a "*" that is next to exactly two part numbers.
#[allow(unused_variables)] // TODO
pub fn part_two(input: &str) -> Option<u32> {
    // build the data structure

    // traverse, mark spaces that are adjacent to a symbol

    // traverse, for each part number, check if it is near a "*" and add it to the list of part numbers
    // TODO: in practice, how do you actually do the above with this data structure?

    // traverse, get sum of all gear ratios

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
