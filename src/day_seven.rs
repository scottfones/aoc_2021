//! Advent of Code 2021
//! Day Seven - The Treachery of Whales
#![allow(dead_code)]

use std::fs;

pub(crate) fn day_six_main() {
    println!("\nDay Seven Answers");
    let input = fs::read_to_string("input/day_seven_input.txt").expect("Error reading file");
    let mut values: Vec<u32> = input
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    values.sort_unstable();

    part_one(&values);
    part_two(&values);
}

fn part_one(values: &[u32]) -> u32 {
    let median = values[values.len() / 2];

    let mut fuel_count = 0_u32;
    for pos in values {
        fuel_count += i32::abs(*pos as i32 - median as i32) as u32
    }

    println!("Part One, Fuel: {}", fuel_count);
    fuel_count
}

fn part_two(values: &[u32]) -> u32 {
    let vals_sum: u32 = values.iter().sum();

    // Need to pay more attention to skew, so we switch to mean
    let raw_mean = vals_sum as f32 / values.len() as f32;
    let mean = {
        if f32::abs(raw_mean - raw_mean.round()) < 0.2 {
            raw_mean.round() as u32
        } else {
            raw_mean.round() as u32 - 1
        }
    };

    let mut fuel_count = 0_u32;
    for pos in values {
        let diff = i32::abs(*pos as i32 - mean as i32) as u32;
        fuel_count += (diff * (diff + 1)) / 2;
    }

    println!("Part Two, Fuel: {}", fuel_count);
    fuel_count
}

#[test]
fn test_dayseven_part_one() {
    let input = fs::read_to_string("input/day_seven_test_input.txt").expect("Error reading file");
    let mut values: Vec<u32> = input
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    values.sort_unstable();

    assert_eq!(part_one(&values), 37);

    let input = fs::read_to_string("input/day_seven_input.txt").expect("Error reading file");
    let mut values: Vec<u32> = input
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    values.sort_unstable();

    assert_eq!(part_one(&values), 343441);
}

#[test]
fn test_dayseven_part_two() {
    let input = fs::read_to_string("input/day_seven_test_input.txt").expect("Error reading file");
    let mut values: Vec<u32> = input
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    values.sort_unstable();

    assert_eq!(part_two(&values), 168);

    let input = fs::read_to_string("input/day_seven_input.txt").expect("Error reading file");
    let mut values: Vec<u32> = input
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    values.sort_unstable();

    assert_eq!(part_two(&values), 98925151);
}
