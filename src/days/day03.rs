use std::{cmp::max, cmp::min, fs};

const INPUT: &str = "day03input.txt";

pub fn check(
    lines: &[&str],
    indexes_of_number: &Vec<(usize, char)>,
    line_idx: usize,
    line_len: usize,
    lines_len: usize,
) -> Option<i32> {
    if !indexes_of_number.is_empty() {
        let (first_idx, _) = indexes_of_number.first().unwrap();
        let first_idx = max((*first_idx as i32) - 1, 0) as usize;
        let (last_idx, _) = indexes_of_number.last().unwrap();
        let last_idx = min(last_idx + 1, line_len - 1);
        let line_above = max((line_idx as i32) - 1, 0) as usize;
        let line_below = min(line_idx + 1, lines_len - 1);

        let mut symbol_found = false;
        for line in lines.iter().take(line_below + 1).skip(line_above) {
            let line_to_check = line.chars().collect::<Vec<char>>();
            for c in line_to_check.iter().take(last_idx + 1).skip(first_idx) {
                if !c.is_numeric() && *c != '.' {
                    symbol_found = true;
                    break;
                }
            }
            if symbol_found {
                break;
            }
        }
        if symbol_found {
            let mut found_num = String::from("");
            for (_i, c) in indexes_of_number {
                found_num.push(*c);
            }
            let n = found_num.parse::<i32>().unwrap();
            return Some(n);
        } else {
            return None;
        }
    }

    None
}

pub fn day03_part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let lines: Vec<&str> = input.lines().collect();
    let line_len = lines[0].len();
    let lines_len = lines.len();

    let mut found_nums: Vec<i32> = vec![];

    for line_idx in 0..lines.len() {
        let line = lines[line_idx];
        let mut indexes_of_number: Vec<(usize, char)> = vec![];

        for (i, c) in line.char_indices() {
            if c.is_numeric() {
                indexes_of_number.push((i, c));
            } else {
                if let Some(n) = check(&lines, &indexes_of_number, line_idx, line_len, lines_len) {
                    found_nums.push(n);
                }
                indexes_of_number.clear();
            }
        }
        if let Some(n) = check(&lines, &indexes_of_number, line_idx, line_len, lines_len) {
            found_nums.push(n);
        }
        indexes_of_number.clear();
    }

    let sum: i32 = found_nums.into_iter().sum();
    println!("{}", sum);
}
