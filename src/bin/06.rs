use std::cmp::Ordering;

use regex::Regex;

advent_of_code::solution!(6);

struct RaceConstraint {
    time: u32,
    distance: u32,
}

struct LargeRaceConstraint {
    time: u128,
    distance: u128,
}

const TIME_RE: &str = r"Time:\s+(?P<nums>((\d+)\s*)*)";
const DISTANCE_RE: &str = r"Distance:\s+(?P<nums>((\d+)\s*)*)";

fn get_race_constraints(input: &str) -> Vec<RaceConstraint> {
    let time_re = Regex::new(TIME_RE).unwrap();
    let Some(times) = time_re.captures(input) else {
        panic!("No match for times");
    };
    let times: Vec<u32> = times["nums"]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let distance_re = Regex::new(DISTANCE_RE).unwrap();
    let Some(distances) = distance_re.captures(input) else {
        panic!("No match for distances");
    };
    let distances: Vec<u32> = distances["nums"]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut race_constraints = vec![];
    for (i, t) in times.iter().enumerate() {
        let d = distances.get(i).unwrap();
        race_constraints.push(RaceConstraint {
            time: *t,
            distance: *d,
        });
    }

    race_constraints
}

/// speed = time_spent_waiting_at_start
/// distance_traveled = speed * (allotted_time - time_spent_waiting_at_start)
pub fn part_one(input: &str) -> Option<u32> {
    let race_constaints = get_race_constraints(input);
    let num_ways_beat_records = race_constaints
        .iter()
        .map(|rc| {
            let mut times_beat_record = 0;

            for i in 1..rc.time {
                let d = i * rc.time.saturating_sub(i);
                if d > rc.distance {
                    times_beat_record += 1;
                };
            }

            times_beat_record
        })
        .product();
    Some(num_ways_beat_records)
}

fn get_combined_race_constraint(input: &str) -> LargeRaceConstraint {
    let time_re = Regex::new(TIME_RE).unwrap();
    let Some(times) = time_re.captures(input) else {
        panic!("No match for times");
    };
    let times: Vec<&str> = times["nums"].split_whitespace().collect();
    let time_combined: u128 = times.join("").parse().unwrap();

    let distance_re = Regex::new(DISTANCE_RE).unwrap();
    let Some(distances) = distance_re.captures(input) else {
        panic!("No match for distances");
    };
    let distances: Vec<&str> = distances["nums"].split_whitespace().collect();
    let distance_combined: u128 = distances.join("").parse().unwrap();

    LargeRaceConstraint {
        time: time_combined,
        distance: distance_combined,
    }
}

/// binary search for the diff of the distance to be 0?
pub fn part_two(input: &str) -> Option<u128> {
    let race_constraint = get_combined_race_constraint(input);

    let times: Vec<u128> = (1..race_constraint.time).collect();
    let min_time = times
        .iter()
        .filter(|&t| {
            let d = t * (race_constraint.time.saturating_sub(*t));
            d > race_constraint.distance
        })
        .collect::<Vec<&u128>>();
    let min_time = *min_time.first().unwrap();

    let ways_to_beat_record = {
        let end_time = race_constraint.time.saturating_sub(*min_time);
        end_time.saturating_sub(*min_time) + 1
    };

    Some(ways_to_beat_record)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
