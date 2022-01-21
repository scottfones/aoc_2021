//! Advent of Code 2021
//! Day One - Sonar Sweep
#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

pub(crate) fn day_one_main() {
    println!("\nDay One - Sonar Sweep - Answers");
    let now = Instant::now();

    let mut input = Vec::new();
    if let Ok(lines) = read_lines("./input/day_one_input.txt") {
        for line in lines {
            input.push(line.unwrap().parse::<i32>().unwrap());
        }
    }

    print_raw_count(&input);
    print_sliding_count(&input);

    println!("Execution time: {}ms", now.elapsed().as_millis());
}

fn print_raw_count(values: &[i32]) {
    let mut inc_counter = 0;
    let mut prev_val = &values[0];

    for val in values[1..].iter() {
        if prev_val < val {
            inc_counter += 1;
        }
        prev_val = val;
    }

    println!("Raw Depth Increase Count: {}", inc_counter);
}

/// Compare sum of a sliding window three values wide
fn print_sliding_count(values: &[i32]) {
    let mut inc_counter = 0;

    let n = values.len();
    for i in 1..n - 2 {
        let prev_val = values[i - 1] + values[i] + values[i + 1];
        let cur_val = values[i] + values[i + 1] + values[i + 2];

        if prev_val < cur_val {
            inc_counter += 1;
        }
    }

    println!("Sliding Depth Increase Count: {}", inc_counter);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
