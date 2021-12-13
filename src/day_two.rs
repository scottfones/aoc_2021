//! Advent of Code 2021
//! Day Two - Dive!
#![allow(dead_code)]

use crate::read_input;

pub(crate) fn day_two_main() {
    let input = read_input::read_file("day_two_input.txt");

    calc_dist_depth(&input);
    calc_aim(&input);
}

fn calc_dist_depth(values: &[String]) {
    let mut depth = 0;
    let mut dist = 0;

    for val in values {
        let command: Vec<&str> = val.split(' ').collect();

        let direction = command[0];
        let units = command[1].parse::<i32>().unwrap();

        if direction == "forward" {
            dist += units;
        } else if direction == "down" {
            depth += units;
        } else if direction == "up" {
            depth -= units;
        }
    }

    println!("Depth: {}", depth);
    println!("Distance: {}", dist);
    println!("Depth x Distance: {}", depth * dist);
}

fn calc_aim(values: &[String]) {
    let mut aim = 0;
    let mut depth = 0;
    let mut dist = 0;

    for val in values {
        let command: Vec<&str> = val.split(' ').collect();

        let direction = command[0];
        let units = command[1].parse::<i32>().unwrap();

        if direction == "forward" {
            dist += units;
            depth += aim * units;
        } else if direction == "down" {
            aim += units;
        } else if direction == "up" {
            aim -= units;
        }
    }

    println!("Aim: Depth: {}", depth);
    println!("Aim: Distance: {}", dist);
    println!("Aim: Depth x Distance: {}", depth * dist);
}
