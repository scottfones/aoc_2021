//! Mod to read the input files for AoC puzzles

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_file(filename: String) -> Vec<String> {
    let mut input = Vec::new();
    if let Ok(lines) = read_lines("./input/".to_string() + &filename) {
        for line in lines {
            input.push(line.unwrap());
        }
    }

    input
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
