use std::{collections::HashMap, fs, str::FromStr};

use regex::Regex;

const INPUT: &str = "day02input.txt";

#[derive(Debug, Hash, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(format!("unknown color {}", s)),
        }
    }
}

type CubeSet = HashMap<Color, usize>;

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<CubeSet>,
}

fn is_game_possible(game: &Game, maximums: &CubeSet) -> bool {
    let mut max_red_in_game = 0;
    let mut max_green_in_game = 0;
    let mut max_blue_in_game = 0;
    game.sets.iter().for_each(|s| {
        if let Some(n) = s.get(&Color::Red) {
            if *n > max_red_in_game {
                max_red_in_game = *n
            }
        }

        if let Some(n) = s.get(&Color::Green) {
            if *n > max_green_in_game {
                max_green_in_game = *n
            }
        }
        if let Some(n) = s.get(&Color::Blue) {
            if *n > max_blue_in_game {
                max_blue_in_game = *n
            }
        }
    });

    maximums.get(&Color::Red).unwrap() >= &max_red_in_game
        && maximums.get(&Color::Green).unwrap() >= &max_green_in_game
        && maximums.get(&Color::Blue).unwrap() >= &max_blue_in_game
}

// "Game 34"
fn parse_game_id(s: &str) -> usize {
    let re = Regex::new(r"Game (\d+)").unwrap();
    let caps = re.captures(s).unwrap();
    caps[1].parse().unwrap()
}

// " 2 green, 5 blue, 20 red"
fn parse_set(s: &str) -> CubeSet {
    let re = Regex::new(r"(\d+)\s+(\w+)").unwrap();
    let mut hash_map = HashMap::new();
    for cap in re.captures_iter(s) {
        let count: usize = cap[1].parse().unwrap();
        let color: Color = cap[2].parse().unwrap();
        hash_map.insert(color, count);
    }
    hash_map
}

// " 4 red, 4 blue; 10 blue, 8 red; 2 green, 5 blue, 20 red"
fn parse_sets(s: &str) -> Vec<CubeSet> {
    let sets: Vec<_> = s.split(';').collect();
    sets.into_iter().map(parse_set).collect()
}

// Game 34: 4 red, 4 blue; 10 blue, 8 red; 2 green, 5 blue, 20 red
fn parse_line(line: &str) -> Game {
    let splitted: Vec<&str> = line.split(":").collect();
    let game_id_part = splitted.first().unwrap();
    let sets_part = splitted.get(1).unwrap();
    let id = parse_game_id(game_id_part);
    let sets = parse_sets(sets_part);
    Game { id, sets }
}

pub fn day02_part1() {
    let input = fs::read_to_string(INPUT).expect("read_to_string failed");
    let games: Vec<Game> = input.lines().map(parse_line).collect();
    let mut maximums: CubeSet = HashMap::new();
    maximums.insert(Color::Red, 12);
    maximums.insert(Color::Green, 13);
    maximums.insert(Color::Blue, 14);
    let possible_ids: Vec<usize> = games
        .into_iter()
        .filter(|g| is_game_possible(g, &maximums))
        .map(|g| g.id)
        .collect();
    let sum: usize = possible_ids.into_iter().sum();
    println!("{}", sum);
}
