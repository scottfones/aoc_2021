//! Advent of Code 2021
//! Day Six - Lanternfish

use std::{fs, collections::HashMap};

pub(crate) fn day_six_main() {
    println!("\nDay Six Answers");
    let input = fs::read_to_string("input/day_six_input.txt").expect("Error reading file");
    let values: Vec<u8> = input.split(',').map(|x| x.parse::<u8>().unwrap()).collect();

    pop_est(&values, 80);
    pop_est(&values, 256);
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

    let mut fish_count = 0_u64;
    for val in fishies.values() {
        fish_count += val;
    }
    println!("256 Days, Total Fish: {}", fish_count);
    fish_count
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