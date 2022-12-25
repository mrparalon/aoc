use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    first();
    second()
}

#[derive(Debug)]
enum Figure {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum GameResult {
    Win,
    Draw,
    Loss,
}

trait Result {
    fn score(&self, other: &Figure) -> i32;
    fn self_score(&self) -> i32;
    fn other_by_game_result(&self, game_result: &GameResult) -> &Figure;
}

impl Result for Figure {
    fn score(&self, other: &Figure) -> i32 {
        match (self, other) {
            (Figure::Rock, Figure::Scissors) => 6,
            (Figure::Paper, Figure::Rock) => 6,
            (Figure::Scissors, Figure::Paper) => 6,
            (Figure::Rock, Figure::Paper) => 0,
            (Figure::Paper, Figure::Scissors) => 0,
            (Figure::Scissors, Figure::Rock) => 0,
            _ => 3,
        }
    }
    fn self_score(&self) -> i32 {
        match self {
            Figure::Rock => 1,
            Figure::Paper => 2,
            Figure::Scissors => 3,
        }
    }
    fn other_by_game_result(&self, game_result: &GameResult) -> &Figure {
        match (self, game_result) {
            (Figure::Rock, GameResult::Loss) => &Figure::Paper,
            (Figure::Paper, GameResult::Loss) => &Figure::Scissors,
            (Figure::Scissors, GameResult::Loss) => &Figure::Rock,
            (Figure::Rock, GameResult::Win) => &Figure::Scissors,
            (Figure::Paper, GameResult::Win) => &Figure::Rock,
            (Figure::Scissors, GameResult::Win) => &Figure::Paper,
            _ => self.clone(),
        }
    }
}

fn first() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut res = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let splited = line.split(' ').collect::<Vec<&str>>();
        let (first, second) = (parse_to_fugure(splited[0]), parse_to_fugure(splited[1]));
        res += second.score(&first) + second.self_score();
    }

    println!("{}", res);
}

fn parse_to_fugure (s: &str) -> Figure {
    match s {
        "A" | "X" => Figure::Rock,
        "B" | "Y" => Figure::Paper,
        "C" | "Z" => Figure::Scissors,
        _ => panic!("Unknown figure"),
    }
}

fn parse_to_game_result (s: &str) -> GameResult {
    match s {
        "X" => GameResult::Win,
        "Y" => GameResult::Draw,
        "Z" => GameResult::Loss,
        _ => panic!("Unknown game result"),
    }
}

fn second() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut res = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let splited = line.split(' ').collect::<Vec<&str>>();
        let (first, game_result) = (parse_to_fugure(splited[0]), parse_to_game_result(splited[1]));
        let second = first.other_by_game_result(&game_result);
        res += second.score(&first) + second.self_score();
        //println!("{:?} {:?} {:?}", first, second, game_result);
    }

    println!("{}", res);
}
