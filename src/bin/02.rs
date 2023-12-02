advent_of_code::solution!(2);

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn extract_round(round_substring: &str) -> Round {
    let cubes = round_substring.split(',').map(|s| s.trim());
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    cubes.for_each(|c| {
        if c.contains("red") {
            red = c.split_whitespace().next().unwrap().parse::<u32>().unwrap()
        } else if c.contains("green") {
            green = c.split_whitespace().next().unwrap().parse::<u32>().unwrap()
        } else if c.contains("blue") {
            blue = c.split_whitespace().next().unwrap().parse::<u32>().unwrap()
        }
    });

    Round { red, green, blue }
}

fn extract_rounds(rounds_substring: &str) -> Vec<Round> {
    rounds_substring
        .split(';')
        .map(|s| s.trim())
        .map(extract_round)
        .collect()
}

fn extract_game_id(game_id_substring: &str) -> u32 {
    game_id_substring
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

fn extract_game(game_line: &str) -> Game {
    let game_line_split: Vec<&str> = game_line.split(':').map(|s| s.trim()).collect();
    let game_id_substring = game_line_split.first();
    let rounds_substring = game_line_split.get(1);

    let game_id = extract_game_id(game_id_substring.unwrap());
    let rounds = extract_rounds(rounds_substring.unwrap());

    Game {
        id: game_id,
        rounds,
    }
}

/// Game consists of N rounds, where in each round:
/// 1) random assortment of cubes are drawn
/// 2) cubes are placed back in the bag
///
/// Available cubes: 12 red, 13 green, 14 blue.
///
/// Return the sum of game ids that are possible with the available cubes.
pub fn part_one(input: &str) -> Option<u32> {
    let games = input.lines().map(extract_game);

    let game_ids_summed = games
        .map(|game| {
            let invalid_rounds: u32 = game
                .rounds
                .iter()
                .map(|r| {
                    if r.red > 12 || r.green > 13 || r.blue > 14 {
                        1
                    } else {
                        0
                    }
                })
                .sum();

            if invalid_rounds > 0 {
                0
            } else {
                game.id
            }
        })
        .sum();

    Some(game_ids_summed)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input.lines().map(extract_game);

    let cube_power_sets_summed: u32 = games
        .map(|game| {
            let (mut min_red, mut min_green, mut min_blue): (u32, u32, u32) = (0, 0, 0);
            game.rounds.iter().for_each(|r| {
                if r.red > min_red {
                    min_red = r.red;
                }
                if r.green > min_green {
                    min_green = r.green
                }
                if r.blue > min_blue {
                    min_blue = r.blue
                }
            });

            min_red * min_green * min_blue
        })
        .sum();

    Some(cube_power_sets_summed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
