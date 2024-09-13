use std::fs;

const INPUT: &str = "day01input.txt";

pub fn parse_number(line: &str) -> i32 {
    let numeric_chars: Vec<char> = line.chars().filter(|c| char::is_numeric(*c)).collect();
    (format!(
        "{}{}",
        numeric_chars.first().unwrap(),
        numeric_chars.last().unwrap()
    ))
    .parse::<i32>()
    .unwrap()
}

pub fn day01_part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let numbers: Vec<i32> = input.lines().map(parse_number).collect();
    println!("{}", numbers.into_iter().sum::<i32>());
}
