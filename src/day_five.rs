//! Advent of Code 2021
//! Day Five - Hydrothermal Venture
#![allow(dead_code)]

use crate::read_input;
use std::{collections::HashMap, str::FromStr};
use std::time::Instant;

pub(crate) fn day_five_main() {
    println!("\nDay Five Answers");
    let now = Instant::now();

    let input = read_input::read_file("day_five_input.txt");

    part_one(&input);
    part_two(&input);

    println!("Execution time: {}ms", now.elapsed().as_millis());
}

#[derive(Debug, PartialEq)]
struct Point<T: FromStr> {
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: FromStr,
{
    fn new(s: &str, sep: char) -> Option<Point<T>> {
        match s.find(sep) {
            None => None,
            Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(x), Ok(y)) => Some(Point { x, y }),
                _ => None,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
struct PointPair {
    a: Point<u32>,
    b: Point<u32>,
}

impl PointPair {
    fn new(s: &str, sep: &str) -> Option<PointPair> {
        let vals: Vec<&str> = s.split(sep).collect();
        let a = match Point::<u32>::new(vals[0], ',') {
            Some(p) => p,
            None => return None,
        };
        let b = match Point::<u32>::new(vals[1], ',') {
            Some(p) => p,
            None => return None,
        };

        Some(PointPair { a, b })
    }
    fn parse_diag(&self, counter: &mut HashMap<(u32, u32), u16>) {
        use std::cmp::Ordering::*;
        match (self.a.x.cmp(&self.b.x), self.a.y.cmp(&self.b.y)) {
            (Less, Less) => {
                let y = self.a.y;
                for (i, x) in (self.a.x..=self.b.x).enumerate() {
                    *counter.entry((x, y + i as u32)).or_insert(0) += 1;
                }
            }
            (Less, Equal) => {
                let y = self.a.y;
                for x in self.a.x..=self.b.x {
                    *counter.entry((x, y)).or_insert(0) += 1;
                }
            }
            (Less, Greater) => {
                let y = self.a.y;
                for (i, x) in (self.a.x..=self.b.x).enumerate() {
                    *counter.entry((x, y - i as u32)).or_insert(0) += 1;
                }
            }
            (Equal, Less) => {
                let x = self.a.x;
                for y in self.a.y..=self.b.y {
                    *counter.entry((x, y)).or_insert(0) += 1;
                }
            }
            (Equal, Equal) => {
                *counter.entry((self.a.x, self.a.y)).or_insert(0) += 1;
            }
            (Equal, Greater) => {
                let x = self.a.x;
                for y in self.b.y..=self.a.y {
                    *counter.entry((x, y)).or_insert(0) += 1;
                }
            }
            (Greater, Less) => {
                let y = self.b.y;
                for (i, x) in (self.b.x..=self.a.x).enumerate() {
                    *counter.entry((x, y - i as u32)).or_insert(0) += 1;
                }
            }
            (Greater, Equal) => {
                let y = self.b.y;
                for x in self.b.x..=self.a.x {
                    *counter.entry((x, y)).or_insert(0) += 1;
                }
            }
            (Greater, Greater) => {
                let y = self.b.y;
                for (i, x) in (self.b.x..=self.a.x).enumerate() {
                    *counter.entry((x, y + i as u32)).or_insert(0) += 1;
                }
            }
        }
    }
    fn parse_horiz_vert(&self, counter: &mut HashMap<(u32, u32), u16>) {
        match (self.a.x, self.a.y, self.b.x, self.b.y) {
            (x1, y1, x2, y2) if x1 == x2 => {
                let s = std::cmp::min(y1, y2);
                let t = std::cmp::max(y1, y2);
                for y in s..=t {
                    *counter.entry((x1, y)).or_insert(0) += 1;
                }
            }
            (x1, y1, x2, y2) if y1 == y2 => {
                let s = std::cmp::min(x1, x2);
                let t = std::cmp::max(x1, x2);
                for x in s..=t {
                    *counter.entry((x, y1)).or_insert(0) += 1;
                }
            }
            _ => (),
        }
    }
}

fn part_one(input: &[String]) -> u32 {
    let mut vent_map = HashMap::<(u32, u32), u16>::new();

    for raw_pp in input {
        if let Some(pp) = PointPair::new(raw_pp, " -> ") {
            pp.parse_horiz_vert(&mut vent_map);
        }
    }

    let mut cover_count = 0u32;
    for val in vent_map.values() {
        if val > &1u16 {
            cover_count += 1;
        }
    }

    println!("Horizontal and Vertical Count: {}", vent_map.len());
    cover_count
}

fn part_two(input: &[String]) -> u32 {
    let mut vent_map = HashMap::<(u32, u32), u16>::new();

    for raw_pp in input {
        if let Some(pp) = PointPair::new(raw_pp, " -> ") {
            pp.parse_diag(&mut vent_map);
        }
    }

    let mut cover_count = 0u32;
    for val in vent_map.values() {
        if val > &1u16 {
            cover_count += 1;
        }
    }

    println!("Diag Count: {}", cover_count);
    cover_count
}

#[test]
fn test_dayfive_point_new() {
    assert_eq!(Point::<u32>::new("", ','), None);
    assert_eq!(Point::<i32>::new("10,", ','), None);
    assert_eq!(Point::<i32>::new(",10", ','), None);
    assert_eq!(
        Point::<i32>::new("10,20", ','),
        Some(Point { x: 10, y: 20 })
    );
    assert_eq!(Point::<i32>::new("10,20xy", ','), None);
    assert_eq!(Point::<f64>::new("0.5x", 'x'), None);
    assert_eq!(
        Point::<f64>::new("0.5x1.5", 'x'),
        Some(Point { x: 0.5, y: 1.5 })
    );
}

#[test]
fn test_dayfive_pointpair_new() {
    assert_eq!(
        PointPair::new("0,9 -> 5,9", " -> "),
        Some(PointPair {
            a: Point { x: 0, y: 9 },
            b: Point { x: 5, y: 9 }
        })
    );
    assert_eq!(
        PointPair::new("8,0 -> 0,8", " -> "),
        Some(PointPair {
            a: Point { x: 8, y: 0 },
            b: Point { x: 0, y: 8 }
        })
    );
    assert_eq!(PointPair::new("8,0 -> 8", " -> "), None);
    assert_eq!(PointPair::new("80 -> 0,8", " -> "), None);
    assert_eq!(PointPair::new("8,0,0,8", " -> "), None);
    assert_eq!(PointPair::new("8,0,0,8", ","), None);
}

#[test]
fn test_dayfive_pointpair_parse_diag_less_less() {
    let pp = PointPair::new("3,1 -> 5,3", " -> ").unwrap();
    let mut vent_map = HashMap::<(u32, u32), u16>::new();
    pp.parse_diag(&mut vent_map);

    assert!(vent_map.contains_key(&(3_u32, 1_u32)));
    assert!(vent_map.contains_key(&(4_u32, 2_u32)));
    assert!(vent_map.contains_key(&(5_u32, 3_u32)));
    assert_eq!(vent_map.len(), 3);
}

#[test]
fn test_dayfive_pointpair_parse_diag_less_equal() {
    let pp = PointPair::new("1,5 -> 9,5", " -> ").unwrap();
    let mut vent_map = HashMap::<(u32, u32), u16>::new();
    pp.parse_diag(&mut vent_map);

    assert!(vent_map.contains_key(&(1_u32, 5_u32)));
    assert!(vent_map.contains_key(&(2_u32, 5_u32)));
    assert!(vent_map.contains_key(&(3_u32, 5_u32)));
    assert!(vent_map.contains_key(&(4_u32, 5_u32)));
    assert!(vent_map.contains_key(&(5_u32, 5_u32)));
    assert!(vent_map.contains_key(&(6_u32, 5_u32)));
    assert!(vent_map.contains_key(&(7_u32, 5_u32)));
    assert!(vent_map.contains_key(&(8_u32, 5_u32)));
    assert!(vent_map.contains_key(&(9_u32, 5_u32)));
    assert_eq!(vent_map.len(), 9);
}

#[test]
fn test_dayfive_pointpair_parse_diag_less_greater() {
    let pp = PointPair::new("3,7 -> 6,4", " -> ").unwrap();
    let mut vent_map = HashMap::<(u32, u32), u16>::new();
    pp.parse_diag(&mut vent_map);

    assert!(vent_map.contains_key(&(3_u32, 7_u32)));
    assert!(vent_map.contains_key(&(4_u32, 6_u32)));
    assert!(vent_map.contains_key(&(5_u32, 5_u32)));
    assert!(vent_map.contains_key(&(6_u32, 4_u32)));
    assert_eq!(vent_map.len(), 4);
}

#[test]
fn test_dayfive_pointpair_parse_diag_equal_less() {
    let pp = PointPair::new("9,4 -> 9,8", " -> ").unwrap();
    let mut vent_map = HashMap::<(u32, u32), u16>::new();
    pp.parse_diag(&mut vent_map);
    println!("{:?}", vent_map);

    assert!(vent_map.contains_key(&(9_u32, 4_u32)));
    assert!(vent_map.contains_key(&(9_u32, 5_u32)));
    assert!(vent_map.contains_key(&(9_u32, 6_u32)));
    assert!(vent_map.contains_key(&(9_u32, 7_u32)));
    assert!(vent_map.contains_key(&(9_u32, 8_u32)));
    assert_eq!(vent_map.len(), 5);
}

#[test]
fn test_dayfive_pointpair_parse_diag_equal_equal() {
    let pp = PointPair::new("2,2 -> 2,2", " -> ").unwrap();
    let mut vent_map = HashMap::<(u32, u32), u16>::new();
    pp.parse_diag(&mut vent_map);

    assert!(vent_map.contains_key(&(2_u32, 2_u32)));
    assert_eq!(vent_map.len(), 1);
}

#[test]
fn test_dayfive_pointpair_parse_diag_equal_greater() {
    let pp = PointPair::new("6,7 -> 6,3", " -> ").unwrap();
    let mut vent_map = HashMap::<(u32, u32), u16>::new();
    pp.parse_diag(&mut vent_map);

    assert!(vent_map.contains_key(&(6_u32, 3_u32)));
    assert!(vent_map.contains_key(&(6_u32, 4_u32)));
    assert!(vent_map.contains_key(&(6_u32, 5_u32)));
    assert!(vent_map.contains_key(&(6_u32, 6_u32)));
    assert!(vent_map.contains_key(&(6_u32, 7_u32)));
    assert_eq!(vent_map.len(), 5);
}

#[test]
fn test_dayfive_pointpair_parse_diag_greater_less() {
    let pp = PointPair::new("9,1 -> 5,5", " -> ").unwrap();
    let mut vent_map = HashMap::<(u32, u32), u16>::new();
    pp.parse_diag(&mut vent_map);

    assert!(vent_map.contains_key(&(9_u32, 1_u32)));
    assert!(vent_map.contains_key(&(8_u32, 2_u32)));
    assert!(vent_map.contains_key(&(7_u32, 3_u32)));
    assert!(vent_map.contains_key(&(6_u32, 4_u32)));
    assert!(vent_map.contains_key(&(5_u32, 5_u32)));
    assert_eq!(vent_map.len(), 5);
}

#[test]
fn test_dayfive_pointpair_parse_diag_greater_equal() {
    let pp = PointPair::new("4,2 -> 3,2", " -> ").unwrap();
    let mut vent_map = HashMap::<(u32, u32), u16>::new();
    pp.parse_diag(&mut vent_map);

    assert!(vent_map.contains_key(&(3_u32, 2_u32)));
    assert!(vent_map.contains_key(&(4_u32, 2_u32)));
    assert_eq!(vent_map.len(), 2);
}

#[test]
fn test_dayfive_pointpair_parse_diag_greater_greater() {
    let pp = PointPair::new("4,5 -> 2,3", " -> ").unwrap();
    let mut vent_map = HashMap::<(u32, u32), u16>::new();
    pp.parse_diag(&mut vent_map);

    assert!(vent_map.contains_key(&(2_u32, 3_u32)));
    assert!(vent_map.contains_key(&(3_u32, 4_u32)));
    assert!(vent_map.contains_key(&(4_u32, 5_u32)));
    assert_eq!(vent_map.len(), 3);
}

#[test]
fn test_dayfive_part_one() {
    let input = read_input::read_file("day_five_test_input.txt");
    assert_eq!(part_one(&input), 5);

    let input = read_input::read_file("day_five_input.txt");
    assert_eq!(part_one(&input), 6113);
}

#[test]
fn test_dayfive_part_two() {
    let input = read_input::read_file("day_five_test_input.txt");
    assert_eq!(part_two(&input), 12);

    let input = read_input::read_file("day_five_input.txt");
    assert_eq!(part_two(&input), 20373);
}
