use std::fs;

const INPUT: &str = "day09input.txt";

fn extrapolate(series: &[i32]) -> i32 {
    let diffs = series
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<i32>>();
    if diffs.iter().all(|d| *d == 0) {
        *series.last().unwrap()
    } else {
        series.last().unwrap() + extrapolate(&diffs)
    }
}

pub fn part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let lines = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let sum = lines.iter().map(|line| extrapolate(line)).sum::<i32>();
    println!("{}", sum);
}
