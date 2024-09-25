use std::fs;

const INPUT: &str = "day06input.txt";

fn parse_numbers(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>()
}

fn calc_winning_ways((time, record): (i32, i32)) -> i32 {
    let r = 1..time;
    r.map(|t| t * (time - t) > record).filter(|x| *x).count() as i32
}

pub fn day06_part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let lines: Vec<&str> = input.lines().collect();
    let times = parse_numbers(lines[0]);
    let records = parse_numbers(lines[1]);
    let races = times.into_iter().zip(records);
    let prod: i32 = races.map(calc_winning_ways).product();
    println!("{}", prod);
}
