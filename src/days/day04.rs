use std::{collections::HashMap, fs};

use regex::Regex;

const INPUT: &str = "day04input.txt";

fn parse_line(line: &str) -> (i32, Vec<i32>, Vec<i32>) {
    let re = Regex::new(r"Card\s+(\d+):((\s+\d+)+)\s\|((\s+\d+)+)").unwrap();
    let captures = re.captures(line).unwrap();
    let id = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let winning: Vec<i32> = captures
        .get(2)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let have: Vec<i32> = captures
        .get(4)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    (id, winning, have)
}

pub fn day04_part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let lines: Vec<&str> = input.lines().collect();

    let sum: i32 = lines
        .into_iter()
        .map(|line| {
            let (_id, winning, have) = parse_line(line);
            let have_and_winning = have
                .into_iter()
                .filter(|h| winning.contains(h))
                .collect::<Vec<i32>>()
                .len() as i32;
            if have_and_winning >= 1 {
                (2.0_f32).powi(have_and_winning - 1) as i32
            } else {
                0
            }
        })
        .sum();
    println!("{}", sum);
}

struct Card {
    winning: Vec<i32>,
    have: Vec<i32>,
}

impl Card {
    fn wins(&self) -> i32 {
        self.have
            .iter()
            .filter(|h| self.winning.contains(h))
            .collect::<Vec<&i32>>()
            .len() as i32
    }
}

pub fn day04_part2() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let lines: Vec<&str> = input.lines().collect();
    let mut id_to_card: HashMap<i32, Card> = HashMap::new();
    let mut id_to_instances: HashMap<i32, i32> = HashMap::new();

    lines.into_iter().for_each(|line| {
        let (id, winning, have) = parse_line(line);
        let card = Card { winning, have };
        id_to_card.insert(id, card);
        id_to_instances.insert(id, 1);
    });

    for id in 1..=id_to_card.len() as i32 {
        let card = id_to_card.get(&id).unwrap();
        let wins = card.wins();
        if wins > 0 {
            for i in (id + 1)..=(id + wins) {
                let new_instance_count = id_to_instances[&i] + id_to_instances[&id];
                id_to_instances.insert(i, new_instance_count);
            }
        }
    }

    let sum: i32 = id_to_instances.values().sum();
    println!("{}", sum);
}
