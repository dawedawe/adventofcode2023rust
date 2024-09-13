use regex::Regex;
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

pub fn parse_number_part2(line: &str) -> i32 {
    fn word_to_number(word: &str) -> &str {
        match word {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => word,
        }
    }

    let re =
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9)").unwrap();

    let first = re.find(line).expect("no first found").as_str();
    let mut last: Option<&str> = None;

    for i in (0..line.len()).rev() {
        if let Some(m) = re.find_at(line, i) {
            last = Some(m.as_str());
            break;
        }
    }

    let last = last.expect("no last found");

    (format!("{}{}", word_to_number(first), word_to_number(last)))
        .parse::<i32>()
        .unwrap()
}
pub fn day01_part2() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let numbers: Vec<i32> = input.lines().map(parse_number_part2).collect();
    println!("{}", numbers.into_iter().sum::<i32>());
}
