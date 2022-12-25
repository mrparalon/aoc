use std::collections::HashSet;
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
    let mut char_set = HashSet::<char>::new();
    for line in reader.lines() {
        let line = line.unwrap();
        line[..line.len() / 2].chars().for_each(|c| {
            char_set.insert(c);
        });
        for c in line[line.len() / 2..].chars() {
            if char_set.contains(&c) {
                res += convert_char_to_priority(&c);
                break;
            }
        }
        char_set.clear();
    }

    println!("{}", res);
}

fn convert_char_to_priority(c: &char) -> u32 {
    let ascii_code = *c as u32;
    println!("{} {}", c, ascii_code);
    if ascii_code >= 97 {
        return ascii_code - 97 + 1;
    }
    return ascii_code + 26 + 1 - 65;
}

fn second() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut res = 0;
    let mut char_set_1 = HashSet::<char>::new();
    let mut char_set_2 = HashSet::<char>::new();
    let mut iter = reader.lines();
    let mut is_done = false;
    while !is_done {
        let line_1: String;
        let line_2: String;
        match iter.next() {
            Some(line) => line_1 = line.unwrap(),
            None => break,
        }
        match iter.next() {
            Some(line) => line_2 = line.unwrap(),
            None => break,
        }
        
        match iter.next() {
            Some(iter_res) => {
                let line_3 = iter_res.unwrap();
                line_1.chars().for_each(|c| {
                    char_set_1.insert(c);
                });
                line_2.chars().for_each(|c| {
                    char_set_2.insert(c);
                });
                for c in line_3.chars() {
                    if char_set_1.contains(&c) && char_set_2.contains(&c) {
                        res += convert_char_to_priority(&c);
                        break;
                    }
                }
                char_set_1.clear();
                char_set_2.clear();
            }
            None => {
                is_done = true;
            }
        };
    }

    println!("{}", res);
}
