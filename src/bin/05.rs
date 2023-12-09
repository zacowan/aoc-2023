advent_of_code::solution!(5);

struct MapRange {
    source_start: usize,
    source_end: usize,
    destination_start: usize,
}

#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

fn extract_numbers_delimited_by_space(numbers: &str) -> Vec<usize> {
    numbers
        .split_whitespace()
        .map(|n| n.trim().parse().unwrap())
        .collect()
}

fn extract_map_range(range: &str) -> MapRange {
    let numbers = extract_numbers_delimited_by_space(range);
    let range_length = numbers.get(2).unwrap();
    let destination_start = numbers.first().unwrap();
    let source_start = numbers.get(1).unwrap();
    let source_end = range_length + source_start;

    MapRange {
        source_start: *source_start,
        source_end,
        destination_start: *destination_start,
    }
}

fn extract_map_range_list(map: &str) -> Vec<MapRange> {
    map.split(":\n")
        .last()
        .unwrap()
        .lines()
        .map(extract_map_range)
        .collect()
}

/// Returns the source if it is not within the range.
/// Otherwise, returns a new source determined by the destination range.
///
/// source = destination + x
/// source - x = destination
///
/// new_source = destination
fn get_num_for_map(source: &usize, map_ranges: &[MapRange]) -> usize {
    let mapped_numbers: Vec<isize> = map_ranges
        .iter()
        .filter_map(|map_range| {
            if *source < map_range.source_start || *source > map_range.source_end {
                return None;
            };

            let x: isize = map_range.source_start as isize - map_range.destination_start as isize;
            Some(*source as isize - x)
        })
        .collect::<Vec<isize>>();

    if !mapped_numbers.is_empty() {
        return *mapped_numbers.first().unwrap() as usize;
    }

    *source
}

fn get_location_numbers_for_seeds(seeds: Vec<usize>, lines: Vec<&str>) -> Vec<usize> {
    let seed_to_soil = extract_map_range_list(lines.get(1).unwrap());
    let soil_to_fertilizer = extract_map_range_list(lines.get(2).unwrap());
    let fertilizer_to_water = extract_map_range_list(lines.get(3).unwrap());
    let water_to_light = extract_map_range_list(lines.get(4).unwrap());
    let light_to_temperature = extract_map_range_list(lines.get(5).unwrap());
    let temperature_to_humidity = extract_map_range_list(lines.get(6).unwrap());
    let humidity_to_location = extract_map_range_list(lines.get(7).unwrap());

    seeds
        .iter()
        .map(|seed| {
            let soil = get_num_for_map(seed, &seed_to_soil);
            let fertilizer = get_num_for_map(&soil, &soil_to_fertilizer);
            let water = get_num_for_map(&fertilizer, &fertilizer_to_water);
            let light = get_num_for_map(&water, &water_to_light);
            let temperature = get_num_for_map(&light, &light_to_temperature);
            let humidity = get_num_for_map(&temperature, &temperature_to_humidity);
            get_num_for_map(&humidity, &humidity_to_location)
        })
        .collect()
}

fn extract_seed_ranges(range: &str) -> Vec<Range> {
    let numbers = extract_numbers_delimited_by_space(range);
    let mut seed_ranges = vec![];
    let mut i: usize = 0;
    while i < numbers.len() {
        let range_start = *numbers.get(i).unwrap();
        let range_length = *numbers.get(i + 1).unwrap();
        let range_end = range_start + range_length;
        seed_ranges.push(Range {
            start: range_start,
            end: range_end,
        });
        i += 2;
    }
    seed_ranges
}

fn spit_range_for_map(range: &Range, map_range: &MapRange) -> Vec<Range> {
    let range_nums: Vec<usize> = (range.start..range.end).collect();

    if range_nums.is_empty() {
        return vec![];
    }

    // Gets numbers outside of the map range
    let outside_before: Vec<&usize> = range_nums
        .iter()
        .filter(|r| **r < map_range.source_start)
        .collect();
    let outside_after: Vec<&usize> = range_nums
        .iter()
        .filter(|r| **r > map_range.source_end)
        .collect();
    let outside_before = match outside_before.len() {
        0 => None,
        _ => Some(Range {
            start: **outside_before.iter().min().unwrap(),
            end: **outside_before.iter().max().unwrap(),
        }),
    };
    let outside_after = match outside_after.len() {
        0 => None,
        _ => Some(Range {
            start: **outside_after.iter().min().unwrap(),
            end: **outside_after.iter().max().unwrap(),
        }),
    };

    // Gets transformed numbers inside of map range
    let x: isize = map_range.source_start as isize - map_range.destination_start as isize;
    let inside: Vec<&usize> = range_nums
        .iter()
        .filter(|r| **r >= map_range.source_start && **r <= map_range.source_end)
        .collect();
    let inside = Some(Range {
        start: (**inside.iter().min().unwrap() as isize - x) as usize,
        end: (**inside.iter().max().unwrap() as isize - x + 1) as usize,
    });

    [outside_before, outside_after, inside]
        .iter()
        .filter_map(|x| match x.is_none() {
            true => None,
            false => Some(x.unwrap()),
        })
        .collect()
}

fn get_ranges_for_map(ranges: &[Range], map_ranges: &[MapRange]) -> Vec<Range> {
    let mut transformed_ranges: Vec<Range> = vec![];

    map_ranges.iter().for_each(|mr| {
        let new_ranges: Vec<Range> = ranges
            .iter()
            .flat_map(|r| {
                // If the range is outside the bounds of the map_range, do not alter the range
                if r.end < mr.source_start || r.start >= mr.source_end {
                    return vec![];
                };
                // Otherwise, split the range up into the parts that get transformed and the parts that remain the same
                spit_range_for_map(r, mr)
            })
            .collect();
        new_ranges.iter().for_each(|r| transformed_ranges.push(*r));
    });

    if transformed_ranges.is_empty() {
        ranges.to_vec()
    } else {
        transformed_ranges
    }
}

fn get_location_numbers_for_seed_ranges(seed_ranges: Vec<Range>, lines: Vec<&str>) -> Vec<usize> {
    let seed_to_soil = extract_map_range_list(lines.get(1).unwrap());
    let soil_to_fertilizer = extract_map_range_list(lines.get(2).unwrap());
    let fertilizer_to_water = extract_map_range_list(lines.get(3).unwrap());
    let water_to_light = extract_map_range_list(lines.get(4).unwrap());
    let light_to_temperature = extract_map_range_list(lines.get(5).unwrap());
    let temperature_to_humidity = extract_map_range_list(lines.get(6).unwrap());
    let humidity_to_location = extract_map_range_list(lines.get(7).unwrap());

    seed_ranges
        .iter()
        .map(|seed_range| {
            let initial_ranges = vec![*seed_range];
            let soil = get_ranges_for_map(&initial_ranges, &seed_to_soil);
            let fertilizer = get_ranges_for_map(&soil, &soil_to_fertilizer);
            let water = get_ranges_for_map(&fertilizer, &fertilizer_to_water);
            let light = get_ranges_for_map(&water, &water_to_light);
            let temperature = get_ranges_for_map(&light, &light_to_temperature);
            let humidity = get_ranges_for_map(&temperature, &temperature_to_humidity);
            let location = get_ranges_for_map(&humidity, &humidity_to_location);
            location
                .iter()
                .min_by(|a, b| a.start.cmp(&b.start))
                .unwrap()
                .start
        })
        .collect()
}

/// Find the lowest location number
pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split("\n\n").collect();

    let seeds =
        extract_numbers_delimited_by_space(lines.first().unwrap().split(": ").last().unwrap());
    let location_numbers = get_location_numbers_for_seeds(seeds, lines);

    Some(*location_numbers.iter().min().unwrap() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split("\n\n").collect();
    let seed_ranges = extract_seed_ranges(lines.first().unwrap().split(": ").last().unwrap());
    let location_numbers = get_location_numbers_for_seed_ranges(seed_ranges, lines);

    Some(*location_numbers.iter().min().unwrap() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
