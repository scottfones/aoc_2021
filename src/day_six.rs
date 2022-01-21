//! Advent of Code 2021
//! Day Six - Lanternfish
#![allow(dead_code)]

use std::time::Instant;
use std::{collections::HashMap, fs};

pub(crate) fn day_six_main() {
    println!("\nDay Six - Lanternfish - Answers");
    let now = Instant::now();

    let input = fs::read_to_string("input/day_six_input.txt").expect("Error reading file");
    let values: Vec<u8> = input.split(',').map(|x| x.parse::<u8>().unwrap()).collect();

    let part_one_count = pop_est(&values, 80);
    let part_two_count = pop_est(&values, 256);

    println!("Part One, Fish Count: {part_one_count}");
    println!("Part Two, Fish Count: {part_two_count}");

    println!("Execution time: {}ms", now.elapsed().as_millis());
}

fn pop_est(values: &[u8], days: u16) -> u64 {
    let mut fishies: HashMap<u8, u64> = HashMap::from([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
    ]);

    for val in values {
        *fishies.entry(*val).or_insert(0) += 1;
    }

    for _ in 1..=days {
        let new_fishies: HashMap<u8, u64> = HashMap::from([
            (0, *fishies.get(&1).unwrap()),
            (1, *fishies.get(&2).unwrap()),
            (2, *fishies.get(&3).unwrap()),
            (3, *fishies.get(&4).unwrap()),
            (4, *fishies.get(&5).unwrap()),
            (5, *fishies.get(&6).unwrap()),
            (6, *fishies.get(&7).unwrap() + *fishies.get(&0).unwrap()),
            (7, *fishies.get(&8).unwrap()),
            (8, *fishies.get(&0).unwrap()),
        ]);

        fishies = new_fishies;
    }

    fishies.values().sum()
}

#[test]
fn test_daysix_pop_est_one() {
    let input = fs::read_to_string("input/day_six_test_input.txt").expect("Error reading file");
    let values: Vec<u8> = input.split(',').map(|x| x.parse::<u8>().unwrap()).collect();

    assert_eq!(pop_est(&values, 80), 5934);
}

#[test]
fn test_daysix_pop_est_two() {
    let input = fs::read_to_string("input/day_six_test_input.txt").expect("Error reading file");
    let values: Vec<u8> = input.split(',').map(|x| x.parse::<u8>().unwrap()).collect();

    assert_eq!(pop_est(&values, 256), 26984457539);
}
