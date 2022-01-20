//! Advent of Code 2021
//! Day Eleven - Dumbo Octopus
#![allow(dead_code)]

use crate::read_input;
use ndarray::prelude::*;
use std::time::Instant;

pub(crate) fn day_eleven_main() {
    println!("\nDay Eleven Answers");
    let now = Instant::now();

    let input = read_input::read_file("day_eleven_input.txt");
    let octomap = OctoMap::new(input);

    part_one(octomap.clone());
    part_two(octomap);

    println!("Execution time: {}ms", now.elapsed().as_millis());
}

fn part_one(mut om: OctoMap) {
    for _ in 1..=100 {
        om.step();
    }
    println!("Part One, Flashes: {}", om.flash_count);
}

fn part_two(mut om: OctoMap) {
    let mut count = 0_u16;
    while om.energy_map.sum() != 0 {
        count += 1;
        om.step();
    }
    println!("Part Two, Sync Step: {}", count);
}

#[derive(Clone, Debug)]
struct OctoMap {
    energy_map: Array2<u16>,
    flash_map: Array2<u16>,
    flash_count: u16,
}

impl OctoMap {
    fn new(input: Vec<String>) -> OctoMap {
        let m = input.len();
        let n = input[0].len();

        // Create energy map
        let mut energy_map: Array2<u16> = Array2::zeros((m, n));
        for (i, line) in input.iter().enumerate() {
            let values_vec: Vec<u16> = line
                .chars()
                .map(|x| x.to_digit(10).unwrap() as u16)
                .collect();
            let values_arr = Array::from_vec(values_vec);
            values_arr.move_into(energy_map.slice_mut(s![i, ..]));
        }

        // Create flash map and count
        let flash_map: Array2<u16> = Array2::zeros((m, n));
        let flash_count = 0_u16;

        OctoMap {
            energy_map,
            flash_map,
            flash_count,
        }
    }

    fn step(&mut self) {
        self.energy_map = &self.energy_map + 1;

        let m = self.energy_map.nrows() - 1;
        let n = self.energy_map.ncols() - 1;
        let mut is_processing = true;
        while is_processing {
            is_processing = false;

            let mut flashers = Vec::new();
            for ((y, x), val) in self.energy_map.indexed_iter_mut() {
                if *val > 9 && self.flash_map[[y, x]] == 0 {
                    is_processing = true;
                    self.flash_map[[y, x]] = 1;
                    flashers.push((y, x));
                }
            }

            for (y, x) in flashers {
                match (y == 0, y == m, x == 0, x == n) {
                    (true, false, true, false) => {
                        // Top Row, Left Column
                        self.energy_map[[y, x + 1]] += 1;
                        self.energy_map[[y + 1, x]] += 1;
                        self.energy_map[[y + 1, x + 1]] += 1;
                    }
                    (true, false, false, false) => {
                        // Top Row, Intermediate Columns
                        self.energy_map[[y, x - 1]] += 1;
                        self.energy_map[[y, x + 1]] += 1;
                        self.energy_map[[y + 1, x - 1]] += 1;
                        self.energy_map[[y + 1, x]] += 1;
                        self.energy_map[[y + 1, x + 1]] += 1;
                    }
                    (true, false, false, true) => {
                        // Top Row, Right Column
                        self.energy_map[[y, x - 1]] += 1;
                        self.energy_map[[y + 1, x - 1]] += 1;
                        self.energy_map[[y + 1, x]] += 1;
                    }
                    (false, false, true, false) => {
                        // Intermediate Rows, Left Column
                        self.energy_map[[y - 1, x]] += 1;
                        self.energy_map[[y - 1, x + 1]] += 1;
                        self.energy_map[[y, x + 1]] += 1;
                        self.energy_map[[y + 1, x]] += 1;
                        self.energy_map[[y + 1, x + 1]] += 1;
                    }
                    (false, false, false, false) => {
                        // Intermediate Rows, Intermediate Columns
                        self.energy_map[[y - 1, x - 1]] += 1;
                        self.energy_map[[y - 1, x]] += 1;
                        self.energy_map[[y - 1, x + 1]] += 1;
                        self.energy_map[[y, x - 1]] += 1;
                        self.energy_map[[y, x + 1]] += 1;
                        self.energy_map[[y + 1, x - 1]] += 1;
                        self.energy_map[[y + 1, x]] += 1;
                        self.energy_map[[y + 1, x + 1]] += 1;
                    }
                    (false, false, false, true) => {
                        // Intermediate Rows, Right Column
                        self.energy_map[[y - 1, x - 1]] += 1;
                        self.energy_map[[y - 1, x]] += 1;
                        self.energy_map[[y, x - 1]] += 1;
                        self.energy_map[[y + 1, x - 1]] += 1;
                        self.energy_map[[y + 1, x]] += 1;
                    }
                    (false, true, true, false) => {
                        // Bottom Row, Left Column
                        self.energy_map[[y - 1, x]] += 1;
                        self.energy_map[[y - 1, x + 1]] += 1;
                        self.energy_map[[y, x + 1]] += 1;
                    }
                    (false, true, false, false) => {
                        // Bottom Row, Intermediate Columns
                        self.energy_map[[y, x - 1]] += 1;
                        self.energy_map[[y, x + 1]] += 1;
                        self.energy_map[[y - 1, x - 1]] += 1;
                        self.energy_map[[y - 1, x]] += 1;
                        self.energy_map[[y - 1, x + 1]] += 1;
                    }
                    (false, true, false, true) => {
                        // Bottom Row, Right Column
                        self.energy_map[[y, x - 1]] += 1;
                        self.energy_map[[y - 1, x - 1]] += 1;
                        self.energy_map[[y - 1, x]] += 1;
                    }
                    _ => unreachable!(),
                }
            }
        }

        for val in self.energy_map.iter_mut() {
            if *val > 9 {
                *val = 0;
            }
        }

        self.flash_count += self.flash_map.sum() as u16;
        self.flash_map.fill(0);
    }
}

#[test]
fn test_dayeleven_part_one_sample() {
    let input = read_input::read_file("day_eleven_test_input.txt");
    let mut octomap = OctoMap::new(input);

    for _ in 1..=100 {
        octomap.step();
    }
    assert_eq!(octomap.flash_count, 1656);
}

#[test]
fn test_dayeleven_part_one_actual() {
    let input = read_input::read_file("day_eleven_input.txt");
    let mut octomap = OctoMap::new(input);

    for _ in 1..=100 {
        octomap.step();
    }
    assert_eq!(octomap.flash_count, 1591);
}

#[test]
fn test_dayeleven_part_two_sample() {
    let input = read_input::read_file("day_eleven_test_input.txt");
    let mut octomap = OctoMap::new(input);

    let mut count = 0_u16;
    while octomap.energy_map.sum() != 0 {
        count += 1;
        octomap.step();
    }
    assert_eq!(count, 195);
}

#[test]
fn test_dayeleven_part_two_actual() {
    let input = read_input::read_file("day_eleven_input.txt");
    let mut octomap = OctoMap::new(input);

    let mut count = 0_u16;
    while octomap.energy_map.sum() != 0 {
        count += 1;
        octomap.step();
    }
    assert_eq!(count, 314);
}
