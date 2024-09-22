use std::fs;

use regex::Regex;

const INPUT: &str = "day05input.txt";

struct RangeSpecs {
    dest_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

struct RangeMap {
    ranges: Vec<RangeSpecs>,
}

impl RangeMap {
    fn get_dest_for(&self, source: &i64) -> i64 {
        self.ranges
            .iter()
            .find_map(|rs| {
                let range = rs.source_range_start..rs.source_range_start + rs.range_length;
                if range.contains(source) {
                    let offset = source - rs.source_range_start;
                    Some(rs.dest_range_start + offset)
                } else {
                    None
                }
            })
            .unwrap_or(*source)
    }
}

fn parse_seeds(line: &str) -> Vec<i64> {
    let re = Regex::new(r"(\d+)").unwrap();
    re.captures_iter(line)
        .map(|c| c[0].parse::<i64>().unwrap())
        .collect()
}

fn parse_map(lines: Vec<&str>) -> RangeMap {
    let ranges = lines
        .iter()
        .map(|line| {
            let mut splits = line.split_whitespace();
            let dest_range_start = splits.next().unwrap().parse::<i64>().unwrap();
            let source_range_start = splits.next().unwrap().parse::<i64>().unwrap();
            let range_length = splits.next().unwrap().parse::<i64>().unwrap();
            RangeSpecs {
                dest_range_start,
                source_range_start,
                range_length,
            }
        })
        .collect();
    RangeMap { ranges }
}

pub fn day05_part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let lines: Vec<&str> = input.lines().collect();
    let splits: Vec<&[&str]> = lines.split(|line| line.is_empty()).collect();

    let seeds_line = splits[0][0];
    let seeds = parse_seeds(seeds_line);

    let seed_to_soil = parse_map(splits[1].split_at(1).1.to_vec());
    let soil_to_fertilizer = parse_map(splits[2].split_at(1).1.to_vec());
    let fertilizer_to_water = parse_map(splits[3].split_at(1).1.to_vec());
    let water_to_light = parse_map(splits[4].split_at(1).1.to_vec());
    let light_to_temperature = parse_map(splits[5].split_at(1).1.to_vec());
    let temperature_to_humidity = parse_map(splits[6].split_at(1).1.to_vec());
    let humidity_to_location = parse_map(splits[7].split_at(1).1.to_vec());

    let min_location = seeds
        .iter()
        .map(|seed| {
            let soil = seed_to_soil.get_dest_for(seed);
            let fertilizer = soil_to_fertilizer.get_dest_for(&soil);
            let water = fertilizer_to_water.get_dest_for(&fertilizer);
            let light = water_to_light.get_dest_for(&water);
            let temperature = light_to_temperature.get_dest_for(&light);
            let humidity = temperature_to_humidity.get_dest_for(&temperature);
            humidity_to_location.get_dest_for(&humidity)
        })
        .min()
        .unwrap();
    println!("{}", min_location);
}
