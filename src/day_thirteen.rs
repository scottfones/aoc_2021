//! Advent of Code 2021
//! Day Thirteen - Transparent Origami
#![allow(dead_code)]

use crate::read_input;
use std::time::Instant;
use std::{collections::HashSet, fmt};

pub(crate) fn day_thirteen_main() {
    println!("\nDay Thirteen - Transparent Origami - Answers");
    let now = Instant::now();

    let input = read_input::read_file("day_thirteen_input.txt");

    let seps: Vec<&[String]> = input.split(|x| x.is_empty()).collect();
    let cords_set: HashSet<Coordinate> = seps[0].iter().fold(HashSet::new(), |mut hs, x| {
        hs.insert(Coordinate::new(x).unwrap());
        hs
    });
    let mut instrs: Vec<_> = seps[1]
        .iter()
        .map(|x| FoldDirection::new(x).unwrap())
        .collect();

    let mut origami = PaperOrigami::new(cords_set).unwrap();

    origami = origami.fold(instrs.remove(0)).unwrap();
    println!("Part One, First Fold Cords: {}", origami.cords.len());

    for inst in instrs {
        origami = origami.fold(inst).unwrap();
    }
    println!("Part Two, {}", origami);

    println!("Execution time: {}ms", now.elapsed().as_millis());
}

#[derive(Debug, PartialEq)]
enum FoldDirection {
    Horizontal(usize),
    Vertical(usize),
}

impl FoldDirection {
    fn new(str_dir: &str) -> Option<FoldDirection> {
        let iso_dir = str_dir
            .trim()
            .split_whitespace()
            .find(|&x| x.starts_with("x=") | x.starts_with("y="));

        match iso_dir {
            Some(iso) => {
                let value = {
                    match iso[2..].trim().parse::<usize>() {
                        Ok(val) => val,
                        _ => return None,
                    }
                };

                match iso.chars().next() {
                    Some('x') => Some(FoldDirection::Horizontal(value)),
                    Some('y') => Some(FoldDirection::Vertical(value)),
                    _ => unreachable!(),
                }
            }
            None => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
struct Coordinate {
    m: usize,
    n: usize,
}

impl Coordinate {
    fn new(str_cord: &str) -> Option<Coordinate> {
        match str_cord.find(',') {
            None => None,
            Some(index) => {
                let n = match str_cord[..index].trim().parse::<usize>() {
                    Ok(val) => val,
                    _ => return None,
                };
                let m = match str_cord[index + 1..].trim().parse::<usize>() {
                    Ok(val) => val,
                    _ => return None,
                };
                Some(Coordinate { m, n })
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct PaperOrigami {
    cords: HashSet<Coordinate>,
    paper: Vec<Vec<char>>,
    size_m: usize,
    size_n: usize,
}

impl PaperOrigami {
    fn new(cords: HashSet<Coordinate>) -> Option<PaperOrigami> {
        if cords.is_empty() {
            return None;
        }

        let mut size_m = 0_usize;
        let mut size_n = 0_usize;

        for cord in &cords {
            if size_m < cord.m {
                size_m = cord.m;
            }
            if size_n < cord.n {
                size_n = cord.n;
            }
        }
        size_m += 1;
        size_n += 1;

        let mut paper = vec![vec![' '; size_n]; size_m];
        for cord in &cords {
            paper[cord.m][cord.n] = '#';
        }

        Some(PaperOrigami {
            cords,
            paper,
            size_m,
            size_n,
        })
    }

    fn fold(&self, fold_dir: FoldDirection) -> Option<PaperOrigami> {
        match fold_dir {
            FoldDirection::Horizontal(index) => Some(self.fold_horizontal(index).unwrap()),
            FoldDirection::Vertical(index) => Some(self.fold_vertical(index).unwrap()),
        }
    }

    fn fold_horizontal(&self, fold_col: usize) -> Option<PaperOrigami> {
        let mut new_cords: HashSet<Coordinate> = HashSet::with_capacity(self.cords.len());

        use std::cmp::Ordering::*;
        for cord in &self.cords {
            match cord.n.cmp(&fold_col) {
                Less => {
                    new_cords.insert(*cord);
                }
                Greater => {
                    new_cords.insert(Coordinate {
                        m: cord.m,
                        n: cord.n - 2 * (cord.n - fold_col),
                    });
                }
                _ => unreachable!(),
            }
        }

        PaperOrigami::new(new_cords)
    }

    fn fold_vertical(&self, fold_row: usize) -> Option<PaperOrigami> {
        let mut new_cords: HashSet<Coordinate> = HashSet::with_capacity(self.cords.len());

        use std::cmp::Ordering::*;
        for cord in &self.cords {
            match cord.m.cmp(&fold_row) {
                Less => {
                    new_cords.insert(*cord);
                }
                Greater => {
                    new_cords.insert(Coordinate {
                        m: cord.m - 2 * (cord.m - fold_row),
                        n: cord.n,
                    });
                }
                _ => unreachable!(),
            }
        }

        PaperOrigami::new(new_cords)
    }
}

impl fmt::Display for PaperOrigami {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Paper Origami ({}x{}, {} Coordinates)",
            self.size_m,
            self.size_n,
            self.cords.len()
        )?;
        for line in &self.paper {
            writeln!(f, "\t{}", String::from_iter(line))?;
        }
        Ok(())
    }
}

#[test]
fn test_daythirteen_folddirection_new_none() {
    assert_eq!(FoldDirection::new(""), None, "testing \"\"");
    assert_eq!(FoldDirection::new(" "), None, "testing \" \"");
    assert_eq!(FoldDirection::new("s="), None, "testing \"s=\"");
    assert_eq!(FoldDirection::new("s=8"), None, "testing \"s=8\"");
    assert_eq!(FoldDirection::new("x=a"), None, "testing \"x=a\"");
    assert_eq!(FoldDirection::new("y= 9"), None, "testing \"y= 9\"");
    assert_eq!(FoldDirection::new("X=11"), None, "testing \"X=11\"");
    assert_eq!(FoldDirection::new("yy=4"), None, "testing \"yy=4\"");
}

#[test]
fn test_daythirteen_folddirection_new_some() {
    assert_eq!(
        FoldDirection::new("y=1"),
        Some(FoldDirection::Vertical(1)),
        "testing \"y=1\""
    );
    assert_eq!(
        FoldDirection::new("y=7 x=0"),
        Some(FoldDirection::Vertical(7)),
        "testing \"y=7 x=0\""
    );
    assert_eq!(
        FoldDirection::new(" turn y=10"),
        Some(FoldDirection::Vertical(10)),
        "testing \" turn y=10\""
    );
    assert_eq!(
        FoldDirection::new("fold along y=20 "),
        Some(FoldDirection::Vertical(20)),
        "testing \"fold along y=20 \""
    );
    assert_eq!(
        FoldDirection::new("y=30 fold along "),
        Some(FoldDirection::Vertical(30)),
        "testing \"y=30 fold along \""
    );
    assert_eq!(
        FoldDirection::new("fold y=40 along "),
        Some(FoldDirection::Vertical(40)),
        "testing \"fold y=40 along \""
    );

    assert_eq!(
        FoldDirection::new("x=1"),
        Some(FoldDirection::Horizontal(1)),
        "testing \"x=1\""
    );
    assert_eq!(
        FoldDirection::new(" turn x=10 "),
        Some(FoldDirection::Horizontal(10)),
        "testing \" turn x=10 \""
    );
    assert_eq!(
        FoldDirection::new("fold along x=20"),
        Some(FoldDirection::Horizontal(20)),
        "testing \"fold along x=20\""
    );
    assert_eq!(
        FoldDirection::new("x=30 fold along "),
        Some(FoldDirection::Horizontal(30)),
        "testing \"x=30 fold along \""
    );
    assert_eq!(
        FoldDirection::new("fold x=40 along "),
        Some(FoldDirection::Horizontal(40)),
        "testing \"fold x=40 along \""
    );
}

#[test]
fn test_daythirteen_coordinate_new_none() {
    assert_eq!(Coordinate::new(""), None, "testing \"\"");
    assert_eq!(Coordinate::new(" "), None, "testing \" \"");
    assert_eq!(Coordinate::new(","), None, "testing \",\"");
    assert_eq!(Coordinate::new("a"), None, "testing \"a\"");
    assert_eq!(Coordinate::new("a,b"), None, "testing \"a,b\"");
    assert_eq!(Coordinate::new("1"), None, "testing \"1\"");
    assert_eq!(Coordinate::new("1,"), None, "testing \"1,\"");
    assert_eq!(Coordinate::new("a,1"), None, "testing \"a,1\"");
    assert_eq!(Coordinate::new("1,a"), None, "testing \"1,a\"");
    assert_eq!(Coordinate::new("1,2,"), None, "testing \"1,2,\"");
    assert_eq!(Coordinate::new("1a,2"), None, "testing \"1a,2\"");
    assert_eq!(Coordinate::new("1,2a"), None, "testing \"1,2a\"");
}

#[test]
fn test_daythirteen_coordinate_new_some() {
    assert_eq!(
        Coordinate::new("1,2"),
        Some(Coordinate { m: 2, n: 1 }),
        "testing \"1,2\""
    );
    assert_eq!(
        Coordinate::new(" 3,4 "),
        Some(Coordinate { m: 4, n: 3 }),
        "testing \" 3,4 \""
    );
    assert_eq!(
        Coordinate::new("5 , 6"),
        Some(Coordinate { m: 6, n: 5 }),
        "testing \"5 , 6\""
    );
    assert_eq!(
        Coordinate::new(" 7 , 8 "),
        Some(Coordinate { m: 8, n: 7 }),
        "testing \" 8 , 7 \""
    );
}

#[test]
fn test_daythirteen_paperorigami_new_none() {
    assert_eq!(
        PaperOrigami::new(HashSet::new()),
        None,
        "testing empty hash set"
    );
}

#[test]
fn test_daythirteen_paperorigami_example() {
    let input = read_input::read_file("day_thirteen_test_input.txt");

    let seps: Vec<&[String]> = input.split(|x| x.is_empty()).collect();
    let cords_set: HashSet<Coordinate> = seps[0].iter().fold(HashSet::new(), |mut hs, x| {
        hs.insert(Coordinate::new(x).unwrap());
        hs
    });

    let mut origami = PaperOrigami::new(cords_set).unwrap();
    assert_eq!(origami.cords.len(), 18);
    assert_eq!(origami.size_m, 15);
    assert_eq!(origami.size_n, 11);

    origami = origami.fold_vertical(7).unwrap();
    assert_eq!(origami.cords.len(), 17);

    origami = origami.fold_horizontal(5).unwrap();
    assert_eq!(origami.cords.len(), 16);
    assert_eq!(origami.size_m, 5);
    assert_eq!(origami.size_n, 5);
}

#[test]
fn test_daythirteen_paperorigami_actual() {
    let input = read_input::read_file("day_thirteen_input.txt");

    let seps: Vec<&[String]> = input.split(|x| x.is_empty()).collect();
    let cords_set: HashSet<Coordinate> = seps[0].iter().fold(HashSet::new(), |mut hs, x| {
        hs.insert(Coordinate::new(x).unwrap());
        hs
    });

    let mut origami = PaperOrigami::new(cords_set).unwrap();
    assert_eq!(origami.cords.len(), 950);
    assert_eq!(origami.size_m, 894);
    assert_eq!(origami.size_n, 1311);

    let instrs: Vec<_> = seps[1]
        .iter()
        .map(|x| FoldDirection::new(x).unwrap())
        .collect();
    for instr in instrs {
        origami = origami.fold(instr).unwrap();
    }
    assert_eq!(origami.cords.len(), 98);
    assert_eq!(origami.size_m, 6);
    assert_eq!(origami.size_n, 39);
}
