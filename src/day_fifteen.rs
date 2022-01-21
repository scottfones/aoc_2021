//! Advent of Code 2021
//! Day Fifteen - Chiton
#![allow(dead_code)]

use crate::read_input;
use std::time::Instant;

pub(crate) fn day_fifteen_main() {
    println!("\nDay Fifteen - Chiton - Answers");
    let now = Instant::now();

    let mut input = read_input::read_file("day_fifteen_test_input.txt");

    println!("Execution time: {}ms", now.elapsed().as_millis());
}
