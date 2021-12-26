//! Advent of Code 2021
//! Day Ten - Syntax Scoring
#![allow(dead_code)]

use std::collections::HashMap;
use crate::read_input;

pub(crate) fn day_ten_main() {
    println!("\nDay Ten Answers");

    let input = read_input::read_file("day_ten_input.txt");

    part_one(&input);
    part_two(input);
}

fn part_one(lines: &[String]) -> u32 {
    let match_map = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let mut unmatched = Vec::new();
    for line in lines {
        let mut parse_queue = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => parse_queue.push(c),
                ')' | ']' | '}' | '>'
                    if *match_map.get(&c).unwrap() == parse_queue.pop().unwrap() => {}
                _ => unmatched.push(c),
            }
        }
    }

    let score_map: HashMap<char, u32> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut score = 0_u32;
    for val in unmatched {
        score += score_map.get(&val).unwrap();
    }
    println!("Score: {}", score);
    score
}

fn part_two(lines: Vec<String>) -> u64 {
    let match_map = HashMap::from([('(', ")"), ('[', "]"), ('{', "}"), ('<', ">")]);
    let score_map: HashMap<char, u64> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let mut scores = Vec::new();
    for line in lines {
        let mut sb = "".to_string();
        if let Some(incomplete) = parse_line(&line) {
            for c in incomplete {
                sb.push_str(match_map.get(&c).unwrap());
            }
            let mut score = 0_u64;
            for ch in sb.chars().rev() {
                score = score * 5 + score_map.get(&ch).unwrap();
            }
            scores.push(score)
        }
    }
    scores.sort_unstable();
    let score = scores[scores.len() / 2];
    println!("Score: {}", score);
    score
}

fn parse_line(line: &str) -> Option<Vec<char>> {
    let match_map = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let mut parse_queue = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => parse_queue.push(c),
            ')' | ']' | '}' | '>' if *match_map.get(&c).unwrap() == parse_queue.pop().unwrap() => {}
            _ => return None,
        }
    }
    Some(parse_queue)
}

#[test]
fn test_dayten_part_one_example() {
    let input = read_input::read_file("day_ten_test_input.txt");
    assert_eq!(part_one(&input), 26397);
}

#[test]
fn test_dayten_part_one_actual() {
    let input = read_input::read_file("day_ten_input.txt");
    assert_eq!(part_one(&input), 319233);
}

#[test]
fn test_dayten_part_two_example() {
    let input = read_input::read_file("day_ten_test_input.txt");
    assert_eq!(part_two(input), 288957);
}

#[test]
fn test_dayten_part_two_actual() {
    let input = read_input::read_file("day_ten_input.txt");
    assert_eq!(part_two(input), 1118976874);
}
