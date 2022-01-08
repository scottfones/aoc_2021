//! Advent of Code 2021
//! Day Nine - Smoke Basin
#![allow(dead_code)]

use std::collections::HashSet;

use crate::read_input;
use ndarray::{s, Array, Array2};

pub(crate) fn day_nine_main() {
    println!("\nDay Nine Answers");

    let input = read_input::read_file("day_nine_input.txt");
    let basin = Basin::new(input);

    part_one(&basin);
    part_two(basin);
}

fn part_one(b: &Basin) {
    println!("Part One, Low Point Count: {}", b.low_points.len());
    println!("Part One, Total Risk Level: {}", b.get_total_risk_level());
}

fn part_two(b: Basin) -> u32 {
    let mut lp_sizes = Vec::new();
    for lp in &b.low_points {
        let mut lp_set = HashSet::new();
        lp_set.insert(*lp);

        let mut lp_res = expand_dir(&b, &lp_set);
        while !lp_res.is_empty() {
            lp_set = lp_set.union(&lp_res).copied().collect();
            lp_res = expand_dir(&b, &lp_set);
        }

        lp_sizes.push(lp_set.len() as u32);
    }

    lp_sizes.sort_unstable_by(|a, b| b.cmp(a));
    let prd: u32 = lp_sizes[..3].iter().product();
    println!("Part Two, Top Three Product: {}", prd);
    prd
}

fn expand_dir(basin: &Basin, lows: &HashSet<MapLocation>) -> HashSet<MapLocation> {
    let mut new_lows: HashSet<MapLocation> = HashSet::new();

    for dir in [
        SearchDir::Up,
        SearchDir::Down,
        SearchDir::Left,
        SearchDir::Right,
    ] {
        for lp in lows {
            match basin.get_step(lp, &dir) {
                Some(loc) if !lows.contains(&loc) && loc.v != 9 => {
                    new_lows.insert(loc);
                }
                _ => (),
            }
        }
    }
    new_lows
}

#[derive(Debug)]
enum SearchDir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct MapLocation {
    m: usize,
    n: usize,
    v: u8,
}

impl MapLocation {
    fn new(m: usize, n: usize, v: u8) -> MapLocation {
        MapLocation { m, n, v }
    }

    fn get_risk_level(&self) -> u8 {
        1 + self.v
    }
}

#[derive(Debug)]
struct Basin {
    height_map: Array2<u8>,
    low_points: Vec<MapLocation>,
}

impl Basin {
    fn new(readings: Vec<String>) -> Basin {
        let m = readings.len();
        let n = readings[0].len();

        // Create matrix map
        let mut height_map = Array2::<u8>::zeros((m, n));
        for (i, line) in readings.iter().enumerate() {
            let values_vec: Vec<u8> = line
                .chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect();
            let values_arr = Array::from_vec(values_vec);
            values_arr.assign_to(height_map.slice_mut(s![i, ..]));
        }

        // Find low points
        let mut low_points: Vec<MapLocation> = Vec::new();
        for ((y, x), val) in height_map.indexed_iter() {
            use std::cmp::Ordering::*;

            // Check horizontal
            let is_low_h = {
                match (x.cmp(&0), x.cmp(&(n - 1))) {
                    (Equal, Less) => val <= &height_map[[y, x + 1]],
                    (Greater, Less) => {
                        val < &height_map[[y, x - 1]] && val < &height_map[[y, x + 1]]
                    }
                    (Greater, Equal) => val <= &height_map[[y, x - 1]],
                    _ => panic!("Horizontal compare mismatch"),
                }
            };

            // Check vertical
            let is_low_v = {
                match (y.cmp(&0), y.cmp(&(m - 1))) {
                    (Equal, Less) => val <= &height_map[[y + 1, x]],
                    (Greater, Less) => {
                        val < &height_map[[y - 1, x]] && val < &height_map[[y + 1, x]]
                    }
                    (Greater, Equal) => val <= &height_map[[y - 1, x]],
                    _ => panic!("Vertical compare mismatch"),
                }
            };

            if is_low_h && is_low_v {
                low_points.push(MapLocation::new(y, x, *val));
            }
        }
        Basin {
            height_map,
            low_points,
        }
    }

    fn get_total_risk_level(&self) -> u32 {
        self.low_points.iter().fold(0, |acc, &x| acc + x.v as u32 + 1)
    }

    fn get_step(&self, lp: &MapLocation, search_dir: &SearchDir) -> Option<MapLocation> {
        let m = self.height_map.nrows() - 1;
        let n = self.height_map.ncols() - 1;
        match (lp.m, lp.n, &search_dir) {
            (0, _, SearchDir::Up) => None,
            (y, _, SearchDir::Down) if y == m => None,
            (_, 0, SearchDir::Left) => None,
            (_, x, SearchDir::Right) if x == n => None,
            _ => match search_dir {
                SearchDir::Up => Some(MapLocation {
                    m: lp.m - 1,
                    n: lp.n,
                    v: self.height_map[[lp.m - 1, lp.n]],
                }),
                SearchDir::Down => Some(MapLocation {
                    m: lp.m + 1,
                    n: lp.n,
                    v: self.height_map[[lp.m + 1, lp.n]],
                }),
                SearchDir::Left => Some(MapLocation {
                    m: lp.m,
                    n: lp.n - 1,
                    v: self.height_map[[lp.m, lp.n - 1]],
                }),
                SearchDir::Right => Some(MapLocation {
                    m: lp.m,
                    n: lp.n + 1,
                    v: self.height_map[[lp.m, lp.n + 1]],
                }),
            },
        }
    }
}

#[test]
fn test_daynine_part_one_example() {
    let input = read_input::read_file("day_nine_test_input.txt");
    let b = Basin::new(input);

    use ndarray::array;
    assert_eq!(b.height_map.row(0), array![2, 1, 9, 9, 9, 4, 3, 2, 1, 0]);
    assert_eq!(b.height_map.row(1), array![3, 9, 8, 7, 8, 9, 4, 9, 2, 1]);
    assert_eq!(b.height_map.row(2), array![9, 8, 5, 6, 7, 8, 9, 8, 9, 2]);
    assert_eq!(b.height_map.row(3), array![8, 7, 6, 7, 8, 9, 6, 7, 8, 9]);
    assert_eq!(b.height_map.row(4), array![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]);

    assert_eq!(b.low_points[0], MapLocation { m: 0, n: 1, v: 1 });
    assert_eq!(b.low_points[1], MapLocation { m: 0, n: 9, v: 0 });
    assert_eq!(b.low_points[2], MapLocation { m: 2, n: 2, v: 5 });
    assert_eq!(b.low_points[3], MapLocation { m: 4, n: 6, v: 5 });
    assert_eq!(b.low_points.len(), 4);

    assert_eq!(b.get_total_risk_level(), 15);
}

#[test]
fn test_daynine_part_one_actual() {
    let input = read_input::read_file("day_nine_input.txt");
    let b = Basin::new(input);

    assert_eq!(b.low_points.len(), 197);
    assert_eq!(b.get_total_risk_level(), 425);
}

#[test]
fn test_daynine_part_two_example() {
    let input = read_input::read_file("day_nine_test_input.txt");
    let b = Basin::new(input);

    assert_eq!(part_two(b), 1134);
}

#[test]
fn test_daynine_part_two_actual() {
    let input = read_input::read_file("day_nine_input.txt");
    let b = Basin::new(input);

    assert_eq!(part_two(b), 1135260);
}

#[test]
fn test_daynine_basin_validate_step() {
    let input = read_input::read_file("day_nine_test_input.txt");
    let b = Basin::new(input);

    let mut lp1 = HashSet::new();
    lp1.insert(&b.low_points[0]);

    assert_eq!(
        b.get_step(&b.low_points[0], &SearchDir::Up),
        None,
        "Input: m={}, n={}, dir={:?}",
        &b.low_points[0].m,
        &b.low_points[0].n,
        SearchDir::Up
    );
    assert_eq!(
        b.get_step(&b.low_points[0], &SearchDir::Down),
        Some(MapLocation { m: 1, n: 1, v: 9 }),
        "Input: m={}, n={}, dir={:?}",
        &b.low_points[0].m,
        &b.low_points[0].n,
        SearchDir::Down
    );
    assert_eq!(
        b.get_step(&b.low_points[0], &SearchDir::Left),
        Some(MapLocation { m: 0, n: 0, v: 2 }),
        "Input: m={}, n={}, dir={:?}",
        &b.low_points[0].m,
        &b.low_points[0].n,
        SearchDir::Left
    );
    assert_eq!(
        b.get_step(&b.low_points[0], &SearchDir::Right),
        Some(MapLocation { m: 0, n: 2, v: 9 }),
        "Input: m={}, n={}, dir={:?}",
        &b.low_points[0].m,
        &b.low_points[0].n,
        SearchDir::Right
    );
}
