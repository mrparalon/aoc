#![allow(dead_code)]
#![allow(unused_variables)]
use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};
use std::collections::VecDeque;

fn main() {
    first();
    second()
}

fn first() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines_iter = reader.lines();
    let mut stack = parse_stacks(&mut lines_iter);
    println!("First: {:?}", stack);
    for line in lines_iter {
        let line = line.unwrap();
        let line_splitted: Vec<&str> = line.split(" ").collect();
        let number: usize = line_splitted[1].parse().unwrap();
        let start: usize = line_splitted[3].parse().unwrap();
        let finish: usize = line_splitted[5].parse().unwrap();
        let mut temp: Vec<char> = vec![];
        for _ in 0..number {
            temp.push(stack[start-1].pop().unwrap());
        }
        stack[finish-1].append(&mut temp);
    }
    let mut res: String = String::new();
    for s in stack {
        res.push(s[s.len()-1])
    }
    println!("First: {:?}", res);
}

fn second() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines_iter = reader.lines();
    let mut stack = parse_stacks(&mut lines_iter);
    println!("First: {:?}", stack);
    for line in lines_iter {
        let line = line.unwrap();
        let line_splitted: Vec<&str> = line.split(" ").collect();
        let number: usize = line_splitted[1].parse().unwrap();
        let start: usize = line_splitted[3].parse().unwrap();
        let finish: usize = line_splitted[5].parse().unwrap();
        let mut temp: VecDeque<char> = VecDeque::new();
        for _ in 0..number {
            temp.push_front(stack[start-1].pop().unwrap());
        }
        stack[finish-1].append(&mut Vec::from(temp));
    }
    let mut res: String = String::new();
    for s in stack {
        res.push(s[s.len()-1])
    }
    println!("First: {:?}", res);
}

fn parse_stacks<T>(input: &mut Lines<T>) -> Vec<Vec<char>>
where
    T: BufRead,
{
    let mut res: Vec<Vec<char>> = vec![];
    for i in 0..9 {
        let mut new_vec: Vec<Vec<char>> = vec![vec![]];
        res.append(&mut new_vec);
    }
    let indexes = vec![1, 5, 9, 13, 17, 21, 25, 29, 33];
    for line in input.by_ref() {
        let line = line.unwrap();
        if line == "" {
            break;
        }
        for (index, i) in indexes.iter().enumerate() {
            let c = line.chars().nth(*i).unwrap();
            if c == '1' {
                break;
            }
            if c != ' ' {
                res[index].push(c);
            }
        }
    }
    res.into_iter().map(|x| x.into_iter().rev().collect()).collect()
}
