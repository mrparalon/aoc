use std::cmp::max;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    first();
    second()
}

fn first() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut res = 0;
    let mut value = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" {
            res = max(value, res);
            value = 0;
            continue;
        }
        let parsed_value: i32 = line.parse().unwrap();
        value += parsed_value;
    }
    println!("{}", res);
}

fn second() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut value = 0;
    let mut calories: Vec<i32> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" {
            calories.push(value);
            value = 0;
            continue;
        }
        let parsed_value: i32 = line.parse().unwrap();
        value += parsed_value;
    }
    calories.sort();
    println!("{}", calories[calories.len()-3..].iter().sum::<i32>());
}
