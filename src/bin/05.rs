advent_of_code::solution!(5);

struct MapRange {
    source_start: usize,
    source_end: usize,
    destination_start: usize,
}

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
        for j in range_start..=range_end {
            seed_ranges.push(Range {
                start: range_start,
                end: range_end,
            });
        }
        i += 2;
    }
    seed_ranges
}

fn get_range_for_map(range: &Range, map_ranges: &[MapRange]) -> Range {
    Range { start: 0, end: 0 }
}

fn get_location_numbers_for_seed_ranges(seed_ranges: Vec<Range>, lines: Vec<&str>) -> Vec<usize> {
    let seed_to_soil = extract_map_range_list(lines.get(1).unwrap());
    let soil_to_fertilizer = extract_map_range_list(lines.get(2).unwrap());
    let fertilizer_to_water = extract_map_range_list(lines.get(3).unwrap());
    let water_to_light = extract_map_range_list(lines.get(4).unwrap());
    let light_to_temperature = extract_map_range_list(lines.get(5).unwrap());
    let temperature_to_humidity = extract_map_range_list(lines.get(6).unwrap());
    let humidity_to_location = extract_map_range_list(lines.get(7).unwrap());

    vec![]
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
    // let seed_ranges = extract_seed_ranges(lines.first().unwrap().split(": ").last().unwrap());

    None
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
        assert_eq!(result, None);
    }
}
