use std::fs;

const INPUT: &str = "day09input.txt";

fn extrapolate<F, G>(series: &[i32], f: F, g: G) -> i32
where
    F: Fn(i32, i32) -> i32,
    F: Clone,
    G: Fn(&[i32]) -> i32,
    G: Clone,
{
    let diffs = series
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<i32>>();
    if diffs.iter().all(|d| *d == 0) {
        g(series)
    } else {
        f(g(series), extrapolate(&diffs, f.clone(), g.clone()))
    }
}

fn get_lines() -> Vec<Vec<i32>> {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

pub fn part1() {
    let lines = get_lines();
    let sum = lines
        .iter()
        .map(|line| extrapolate(line, |x, y| x + y, |series| *series.last().unwrap()))
        .sum::<i32>();
    println!("{}", sum);
}

pub fn part2() {
    let lines = get_lines();
    let sum = lines
        .iter()
        .map(|line| extrapolate(line, |x, y| x - y, |series| *series.first().unwrap()))
        .sum::<i32>();
    println!("{}", sum);
}
