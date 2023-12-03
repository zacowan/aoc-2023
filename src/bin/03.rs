use std::collections::HashSet;

advent_of_code::solution!(3);

#[derive(Clone)]
struct Node {
    value: char,
    is_adjacent_to_symbol: Option<bool>,
}

struct NodeWithRowAndCol {
    value: char,
    row: usize,
    col: usize,
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

fn get_adjacent_nodes(
    nodes: &[Vec<Node>],
    starting_node: (usize, usize),
) -> Vec<NodeWithRowAndCol> {
    let (i, j) = starting_node;
    let mut adjacent_nodes = vec![];
    for di in -1..2 {
        for dj in -1..2 {
            let (r, c) = (
                (i as isize + di as isize) as usize,
                (j as isize + dj as isize) as usize,
            );
            let row = nodes.get(r);
            if row.is_none() {
                continue;
            }
            let node = row.unwrap().get(c);
            if node.is_none() {
                continue;
            }
            adjacent_nodes.push(NodeWithRowAndCol {
                value: node.unwrap().value,
                row: r,
                col: c,
            });
        }
    }
    adjacent_nodes
}

fn do_callback_on_adjacent_nodes<F>(
    nodes: &mut [Vec<Node>],
    starting_node: (usize, usize),
    callback: F,
) where
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

            do_callback_on_adjacent_nodes(&mut nodes_copy, (i, j), |node| {
                node.set_adjacent_to_symbol(true);
            });
        }
    }
    nodes_copy
}

fn build_part_number_from_node(
    nodes: &[Vec<Node>],
    starting_node: &NodeWithRowAndCol,
    visited: &mut HashSet<(usize, usize)>,
) -> u32 {
    let mut part_number = starting_node.value.to_string();
    let row = nodes.get(starting_node.row).unwrap();
    // Look to the left
    let mut curr_col: i32 = (starting_node.col - 1).try_into().unwrap();
    while curr_col >= 0 {
        let node = row.get(curr_col as usize);
        if node.is_none() {
            break;
        }

        visited.insert((starting_node.row, curr_col as usize));

        if !node.unwrap().value.is_numeric() {
            break;
        }

        part_number = node.unwrap().value.to_string() + &part_number;
        curr_col -= 1;
    }
    // Look to the right
    curr_col = (starting_node.col + 1).try_into().unwrap();
    while (curr_col as usize) < row.len() {
        let node = row.get(curr_col as usize);
        if node.is_none() {
            break;
        }

        visited.insert((starting_node.row, curr_col as usize));

        if !node.unwrap().value.is_numeric() {
            break;
        }

        part_number += &node.unwrap().value.to_string();
        curr_col += 1;
    }

    part_number.parse().unwrap()
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
    let nodes: Vec<Vec<Node>> = input
        .lines()
        .map(|l| l.chars().map(build_node).collect())
        .collect();

    // look for "*", get adjacent part numbers
    let mut gear_ratios: Vec<u32> = vec![];
    for (i, row) in nodes.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            if node.value != '*' {
                continue;
            }

            let adjacent_nodes = get_adjacent_nodes(&nodes, (i, j));
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            let adjacent_part_numbers: Vec<u32> = adjacent_nodes
                .iter()
                .filter_map(|n| {
                    if !n.value.is_numeric() || visited.contains(&(n.row, n.col)) {
                        return None;
                    }

                    Some(build_part_number_from_node(&nodes, n, &mut visited))
                })
                .collect();

            if adjacent_part_numbers.len() == 2 {
                gear_ratios.push(adjacent_part_numbers.iter().product());
            }
        }
    }

    Some(gear_ratios.iter().sum())
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
        assert_eq!(result, Some(467835));
    }
}
