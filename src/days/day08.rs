use std::{collections::HashMap, fs};

use regex::Regex;

const INPUT: &str = "day08input.txt";

struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn from(line: &str) -> Node {
        let re =
            Regex::new(r"(([A-Z]|[0-9])+)\s=\s\((([A-Z]|[0-9])+)+,\s(([A-Z]|[0-9])+)+").unwrap();
        let captures = re.captures(line).unwrap();
        let name = String::from(captures.get(1).unwrap().as_str());
        let left = String::from(captures.get(3).unwrap().as_str());
        let right = String::from(captures.get(5).unwrap().as_str());
        Node { name, left, right }
    }
}

fn parse_directions(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn parse_map(lines: Vec<&str>) -> HashMap<String, Node> {
    let mut hm = HashMap::new();
    lines.iter().for_each(|line| {
        let n = Node::from(line);
        hm.insert(String::from(&n.name[..]), n);
    });

    hm
}

pub fn part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let lines: Vec<&str> = input.lines().collect();
    let directions = parse_directions(lines[0]);
    let map_lines = lines.into_iter().skip(2).collect::<Vec<&str>>();
    let map = parse_map(map_lines);
    let mut current = map.get("AAA").unwrap();
    let mut steps = 0;

    while current.name != "ZZZ" {
        let dir = directions[steps % directions.len()];
        let next = if dir == 'L' {
            &current.left[..]
        } else {
            &current.right[..]
        };

        current = map.get(next).unwrap();
        steps += 1;
    }

    println!("{steps}")
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn part2() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let lines: Vec<&str> = input.lines().collect();
    let directions = parse_directions(lines[0]);
    let map_lines = lines.into_iter().skip(2).collect::<Vec<&str>>();
    let map = parse_map(map_lines);
    let current_nodes = map
        .keys()
        .filter(|n| n.ends_with("A"))
        .map(|n| map.get(n).unwrap())
        .collect::<Vec<&Node>>();
    let mut needed_steps: Vec<usize> = vec![];

    for mut current in current_nodes {
        let mut steps = 0;
        while !current.name.ends_with("Z") {
            let dir = directions[steps % directions.len()];
            let next = if dir == 'L' {
                &current.left[..]
            } else {
                &current.right[..]
            };
            current = map.get(next).unwrap();
            steps += 1;
        }
        needed_steps.push(steps);
    }

    let lcm_needed = needed_steps.iter().cloned().reduce(lcm).unwrap();
    println!("{}", lcm_needed);
}
