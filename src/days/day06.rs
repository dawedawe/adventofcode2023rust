use std::fs;

const INPUT: &str = "day06input.txt";

fn parse_numbers(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i64>>()
}

fn parse_numbers_part2(line: &str) -> i64 {
    let number: i64 = line.split_whitespace().skip(1).collect::<String>().parse().unwrap();
    number
}

fn calc_winning_ways((time, record): (i64, i64)) -> i64 {
    let r = 1..time;
    r.map(|t| t * (time - t) > record).filter(|x| *x).count() as i64
}

pub fn day06_part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let lines: Vec<&str> = input.lines().collect();
    let times = parse_numbers(lines[0]);
    let records = parse_numbers(lines[1]);
    let races = times.into_iter().zip(records);
    let prod: i64 = races.map(calc_winning_ways).product();
    println!("{}", prod);
}

pub fn day06_part2() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let lines: Vec<&str> = input.lines().collect();
    let time = parse_numbers_part2(lines[0]);
    let record = parse_numbers_part2(lines[1]);
    let winning_ways = calc_winning_ways((time, record));
    println!("{}", winning_ways);
}
