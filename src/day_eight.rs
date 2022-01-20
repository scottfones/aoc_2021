//! Advent of Code 2021
//! Day Eight - Seven Segment Search
#![allow(dead_code)]

use crate::read_input;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

pub(crate) fn day_eight_main() {
    println!("\nDay Eight Answers");
    let now = Instant::now();

    let input = read_input::read_file("day_eight_input.txt");

    part_one(&input);
    part_two(&input);

    println!("Execution time: {}ms", now.elapsed().as_millis());
}

#[derive(Debug)]
struct SevenSegment {
    up_top: char,
    up_left: char,
    up_right: char,
    mid: char,
    dn_left: char,
    dn_right: char,
    dn_bottom: char,
}

impl SevenSegment {
    /// Identifies the seven segment display partitions given
    ///  a series of input patterns.
    fn new(seq: Vec<&str>) -> SevenSegment {
        let mut five_segs: Vec<HashSet<char>> = Vec::new();
        let mut six_segs: Vec<HashSet<char>> = Vec::new();
        let mut unique_map: HashMap<usize, HashSet<char>> = HashMap::new();
        for s in seq {
            match s.len() {
                2 | 3 | 4 | 7 => {
                    unique_map.insert(s.len(), s.chars().collect());
                }
                5 => five_segs.push(s.chars().collect()),
                6 => six_segs.push(s.chars().collect()),
                _ => (),
            }
        }

        let one = unique_map.get(&2).unwrap().clone();
        let four = unique_map.get(&4).unwrap().clone();
        let seven = unique_map.get(&3).unwrap().clone();
        let eight = unique_map.get(&7).unwrap().clone();

        // UP_TOP
        let up_top = *seven.difference(&one).next().unwrap();

        // DN_BOTTOM
        let mut jig_dn_bottom = four.clone();
        jig_dn_bottom.insert(up_top);
        let mut dn_bottom = '1';
        for seq in &six_segs {
            let a: HashSet<&char> = seq.difference(&jig_dn_bottom).collect();
            if a.len() == 1 {
                dn_bottom = **a.iter().next().unwrap();
            }
        }

        // DN_LEFT
        let mut jig_dn_left = four;
        jig_dn_left.insert(up_top);
        jig_dn_left.insert(dn_bottom);
        let dn_left = *eight.difference(&jig_dn_left).next().unwrap();

        // UP_LEFT
        let mut jig_up_left = seven.clone();
        jig_up_left.insert(dn_bottom);
        jig_up_left.insert(dn_left);
        let mut up_left = '1';
        for seq in &six_segs {
            let a: HashSet<&char> = seq.difference(&jig_up_left).collect();
            if a.len() == 1 {
                up_left = **a.iter().next().unwrap();
            }
        }

        // MID
        let mut jig_mid = seven;
        jig_mid.insert(up_left);
        jig_mid.insert(dn_left);
        jig_mid.insert(dn_bottom);
        let mid = *eight.difference(&jig_mid).next().unwrap();

        // DN_RIGHT
        let mut jig_down_right = HashSet::new();
        jig_down_right.insert(up_top);
        jig_down_right.insert(up_left);
        jig_down_right.insert(mid);
        jig_down_right.insert(dn_left);
        jig_down_right.insert(dn_bottom);
        let mut dn_right = '1';
        for seq in &six_segs {
            let a: HashSet<&char> = seq.difference(&jig_down_right).collect();
            if a.len() == 1 {
                dn_right = **a.iter().next().unwrap();
            }
        }

        // UP_RIGHT
        let up_right = *one
            .difference(&[dn_right].iter().cloned().collect())
            .next()
            .unwrap();

        SevenSegment {
            up_top,
            up_left,
            up_right,
            mid,
            dn_left,
            dn_right,
            dn_bottom,
        }
    }

    /// Decode all encoded patterns for a given input to yield a numerical value
    fn decode(&self, seq: Vec<&str>) -> u16 {
        let mut output = "".to_string();

        for s in seq {
            output.push(self.match_pattern(s).unwrap());
        }

        output.parse().unwrap()
    }

    /// Match an individual, encoded output pattern
    fn match_pattern(&self, pat: &str) -> Result<char, &str> {
        match (
            pat.contains(&self.up_top.to_string().as_str()),
            pat.contains(&self.up_left.to_string().as_str()),
            pat.contains(&self.up_right.to_string().as_str()),
            pat.contains(&self.mid.to_string().as_str()),
            pat.contains(&self.dn_left.to_string().as_str()),
            pat.contains(&self.dn_right.to_string().as_str()),
            pat.contains(&self.dn_bottom.to_string().as_str()),
        ) {
            (true, true, true, false, true, true, true) => Ok('0'),
            (false, false, true, false, false, true, false) => Ok('1'),
            (true, false, true, true, true, false, true) => Ok('2'),
            (true, false, true, true, false, true, true) => Ok('3'),
            (false, true, true, true, false, true, false) => Ok('4'),
            (true, true, false, true, false, true, true) => Ok('5'),
            (true, true, false, true, true, true, true) => Ok('6'),
            (true, false, true, false, false, true, false) => Ok('7'),
            (true, true, true, true, true, true, true) => Ok('8'),
            (true, true, true, true, false, true, true) => Ok('9'),
            _ => Err("Unable to decode"),
        }
    }
}

fn part_one(values: &[String]) -> u32 {
    let mut unique_count = 0_u32;
    for line in values {
        let line_parts: Vec<&str> = line.split(" | ").collect();
        let output: Vec<&str> = line_parts[1].split(' ').collect();

        for pat in output {
            match pat.len() {
                2 | 4 | 3 | 7 => unique_count += 1,
                _ => (),
            }
        }
    }

    println!("Part One, Count: {}", unique_count);
    unique_count
}

fn part_two(values: &[String]) -> u32 {
    let mut accumulator = 0_u32;

    for line in values {
        let line_parts: Vec<&str> = line.split(" | ").collect();
        let patterns: Vec<&str> = line_parts[0].split(' ').collect();
        let encoded: Vec<&str> = line_parts[1].split(' ').collect();

        let sevseg = SevenSegment::new(patterns);
        accumulator += sevseg.decode(encoded) as u32;
    }

    println!("Part Two, Sum: {}", accumulator);
    accumulator
}

#[test]
fn test_dayeight_part_one() {
    let input = read_input::read_file("day_eight_test_input.txt");
    assert_eq!(part_one(&input), 26);
}

#[test]
fn test_dayeight_part_two() {
    let values = read_input::read_file("day_eight_test_input.txt");
    let mut accumulator = 0_u32;

    for line in values {
        let line_parts: Vec<&str> = line.split(" | ").collect();
        let patterns: Vec<&str> = line_parts[0].split(' ').collect();
        let encoded: Vec<&str> = line_parts[1].split(' ').collect();

        let sevseg = SevenSegment::new(patterns);
        accumulator += sevseg.decode(encoded) as u32;
    }
    assert_eq!(accumulator, 61229);
}

#[test]
fn test_dayeight_sevensegment_new() {
    let seq: Vec<&str> = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab"
        .split(' ')
        .collect();
    let sevseg = SevenSegment::new(seq);

    assert!(sevseg.up_top == 'd');
    assert!(sevseg.up_left == 'e');
    assert!(sevseg.up_right == 'a');
    assert!(sevseg.mid == 'f');
    assert!(sevseg.dn_left == 'g');
    assert!(sevseg.dn_right == 'b');
    assert!(sevseg.dn_bottom == 'c');
}

#[test]
fn test_dayeight_sevensegment_decode_single() {
    let input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let values: Vec<&str> = input.split(" | ").collect();
    let sig_pat: Vec<&str> = values[0].split(' ').collect();
    let encoded: Vec<&str> = values[1].split(' ').collect();

    let sevseg = SevenSegment::new(sig_pat);
    assert_eq!(sevseg.decode(encoded), 5353);
}

#[test]
fn test_dayeight_sevensegment_decode_testinput() {
    let input = read_input::read_file("day_eight_test_input.txt");
    let exp_vals: Vec<u16> = vec![8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315];

    for (i, line) in input.iter().enumerate() {
        let line_parts: Vec<&str> = line.split(" | ").collect();
        let patterns: Vec<&str> = line_parts[0].split(' ').collect();
        let encoded: Vec<&str> = line_parts[1].split(' ').collect();

        let sevseg = SevenSegment::new(patterns);
        assert_eq!(sevseg.decode(encoded), exp_vals[i]);
    }
}
