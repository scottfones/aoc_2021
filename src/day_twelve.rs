//! Advent of Code 2021
//! Day Twelve - Passage Pathing
#![allow(dead_code)]

use crate::read_input;
use std::collections::HashMap;

pub(crate) fn day_twelve_main() {
    println!("\nDay Twelve Answers");

    let input = read_input::read_file("day_twelve_input.txt");
    let path_dict = build_path_dict(&input);

    part_one(&path_dict);
    part_two(&path_dict);
}

fn part_one(path_dict: &HashMap<&str, Vec<&str>>) {
    let mut path_tree = CaveNode::new("start".to_string());
    for val in path_dict.get("start").unwrap() {
        path_tree.add(vec!["start".to_string()], val.to_string(), path_dict);
    }

    let path_count = count_paths(&path_tree);
    println!("Part One, Path Count: {}", path_count);
}

fn part_two(path_dict: &HashMap<&str, Vec<&str>>) {
    let mut path_tree = CaveNode::new("start".to_string());
    for val in path_dict.get("start").unwrap() {
        path_tree.add2(vec!["start".to_string()], val.to_string(), path_dict);
    }

    let path_count = count_paths(&path_tree);
    println!("Part Two, Path Count: {}", path_count);
}

#[derive(Debug)]
enum CaveType {
    End,
    Large,
    Small,
    Start,
}

#[derive(Debug)]
struct CaveNode {
    cave_type: CaveType,
    id: String,
    paths: Vec<CaveNode>,
}

impl CaveNode {
    fn new(new_id: String) -> CaveNode {
        match new_id {
            id if id.eq("start") => CaveNode {
                cave_type: CaveType::Start,
                id,
                paths: Vec::<CaveNode>::new(),
            },
            id if id.eq("end") => CaveNode {
                cave_type: CaveType::End,
                id,
                paths: Vec::<CaveNode>::new(),
            },
            id if id.chars().next().unwrap().is_lowercase() => CaveNode {
                cave_type: CaveType::Small,
                id,
                paths: Vec::<CaveNode>::new(),
            },
            id if id.chars().next().unwrap().is_uppercase() => CaveNode {
                cave_type: CaveType::Large,
                id,
                paths: Vec::<CaveNode>::new(),
            },
            _ => unreachable!(),
        }
    }

    /// Add method satisfying the part one constraints.
    fn add(
        &mut self,
        mut chain: Vec<String>,
        new_id: String,
        path_dict: &HashMap<&str, Vec<&str>>,
    ) -> bool {
        let mut is_term_path = false;
        let mut new_node = CaveNode::new(new_id.clone());

        match (&self.cave_type, &new_node.cave_type) {
            (_, &CaveType::Small) if chain.contains(&new_id) => false,
            (_, &CaveType::End) => {
                self.paths.push(new_node);
                true
            }
            _ => {
                chain.push(new_id.clone());
                if let Some(values) = path_dict.get(&new_id.as_str()) {
                    for val in values {
                        is_term_path |= new_node.add(chain.clone(), val.to_string(), path_dict);
                    }
                }

                if is_term_path {
                    self.paths.push(new_node);
                }
                is_term_path
            }
        }
    }

    /// Add method satisfying the part two constraints.
    fn add2(
        &mut self,
        mut chain: Vec<String>,
        new_id: String,
        path_dict: &HashMap<&str, Vec<&str>>,
    ) -> bool {
        let mut is_term_path = false;
        let mut new_node = CaveNode::new(new_id.clone());

        match (&self.cave_type, &new_node.cave_type) {
            (_, &CaveType::Small) if chain.contains(&new_id) => {
                let mut is_doubled = false;
                for lower in chain
                    .iter()
                    .filter(|s| s.chars().next().unwrap().is_lowercase())
                {
                    if chain.iter().filter(|s| s.eq(&lower)).count() > 1 {
                        is_doubled = true;
                        break;
                    }
                }

                if !is_doubled {
                    chain.push(new_id.clone());
                    if let Some(values) = path_dict.get(&new_id.as_str()) {
                        for val in values {
                            is_term_path |=
                                new_node.add2(chain.clone(), val.to_string(), path_dict);
                        }
                    }

                    if is_term_path {
                        self.paths.push(new_node);
                    }
                }

                is_term_path
            }
            (_, &CaveType::End) => {
                self.paths.push(new_node);
                true
            }
            _ => {
                chain.push(new_id.clone());
                if let Some(values) = path_dict.get(&new_id.as_str()) {
                    for val in values {
                        is_term_path |= new_node.add2(chain.clone(), val.to_string(), path_dict);
                    }
                }

                if is_term_path {
                    self.paths.push(new_node);
                }
                is_term_path
            }
        }
    }
}

fn build_path_dict(lines: &[String]) -> HashMap<&str, Vec<&str>> {
    let mut path_dict: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split('-').collect();
        match (parts[0], parts[1]) {
            (a, b) if a.eq("start") || b.eq("end") => {
                let path = path_dict.entry(a).or_insert_with(|| vec![b]);
                if !path.contains(&b) {
                    path.push(b);
                }
            }
            (a, b) if b.eq("start") || a.eq("end") => {
                let path = path_dict.entry(b).or_insert_with(|| vec![a]);
                if !path.contains(&a) {
                    path.push(a);
                }
            }
            (a, b) => {
                let path = path_dict.entry(a).or_insert_with(|| vec![b]);
                if !path.contains(&b) {
                    path.push(b);
                }

                let path = path_dict.entry(b).or_insert_with(|| vec![a]);
                if !path.contains(&a) {
                    path.push(a);
                }
            }
        }
    }
    path_dict
}

fn count_paths(cave_node: &CaveNode) -> u32 {
    let mut path_count = 0_u32;

    match cave_node.cave_type {
        CaveType::End => 1,
        _ => {
            for node in &cave_node.paths {
                path_count += count_paths(node)
            }
            path_count
        }
    }
}

#[test]
fn test_daytwelve_build_path_dict_sample_small() {
    let input = read_input::read_file("day_twelve_test_input_small.txt");
    let pd = build_path_dict(&input);

    assert_eq!(pd.get("start"), Some(&vec!["A", "b"]));
    assert_eq!(pd.get("d"), Some(&vec!["b"]));
    assert_eq!(pd.get("b"), Some(&vec!["A", "d", "end"]));
    assert_eq!(pd.get("c"), Some(&vec!["A"]));
    assert_eq!(pd.get("A"), Some(&vec!["c", "b", "end"]));
    assert_eq!(pd.len(), 5);
}

#[test]
fn test_daytwelve_part_one_sample_small() {
    let input = read_input::read_file("day_twelve_test_input_small.txt");
    let path_dict = build_path_dict(&input);

    let mut path_tree = CaveNode::new("start".to_string());
    for val in path_dict.get("start").unwrap() {
        path_tree.add(vec!["start".to_string()], val.to_string(), &path_dict);
    }

    let paths = count_paths(&path_tree);
    assert_eq!(paths, 10)
}

#[test]
fn test_daytwelve_part_one_sample_medium() {
    let input = read_input::read_file("day_twelve_test_input_medium.txt");
    let path_dict = build_path_dict(&input);

    let mut path_tree = CaveNode::new("start".to_string());
    for val in path_dict.get("start").unwrap() {
        path_tree.add(vec!["start".to_string()], val.to_string(), &path_dict);
    }

    let paths = count_paths(&path_tree);
    assert_eq!(paths, 19)
}

#[test]
fn test_daytwelve_part_one_sample_large() {
    let input = read_input::read_file("day_twelve_test_input_large.txt");
    let path_dict = build_path_dict(&input);

    let mut path_tree = CaveNode::new("start".to_string());
    for val in path_dict.get("start").unwrap() {
        path_tree.add(vec!["start".to_string()], val.to_string(), &path_dict);
    }

    let paths = count_paths(&path_tree);
    assert_eq!(paths, 226)
}

#[test]
fn test_daytwelve_part_one_actual() {
    let input = read_input::read_file("day_twelve_input.txt");
    let path_dict = build_path_dict(&input);

    let mut path_tree = CaveNode::new("start".to_string());
    for val in path_dict.get("start").unwrap() {
        path_tree.add(vec!["start".to_string()], val.to_string(), &path_dict);
    }

    let paths = count_paths(&path_tree);
    assert_eq!(paths, 4707)
}

#[test]
fn test_daytwelve_part_two_sample_small() {
    let input = read_input::read_file("day_twelve_test_input_small.txt");
    let path_dict = build_path_dict(&input);

    let mut path_tree = CaveNode::new("start".to_string());
    for val in path_dict.get("start").unwrap() {
        path_tree.add2(vec!["start".to_string()], val.to_string(), &path_dict);
    }

    let paths = count_paths(&path_tree);
    assert_eq!(paths, 36)
}

#[test]
fn test_daytwelve_part_two_sample_medium() {
    let input = read_input::read_file("day_twelve_test_input_medium.txt");
    let path_dict = build_path_dict(&input);

    let mut path_tree = CaveNode::new("start".to_string());
    for val in path_dict.get("start").unwrap() {
        path_tree.add2(vec!["start".to_string()], val.to_string(), &path_dict);
    }

    let paths = count_paths(&path_tree);
    assert_eq!(paths, 103)
}

#[test]
fn test_daytwelve_part_two_sample_large() {
    let input = read_input::read_file("day_twelve_test_input_large.txt");
    let path_dict = build_path_dict(&input);

    let mut path_tree = CaveNode::new("start".to_string());
    for val in path_dict.get("start").unwrap() {
        path_tree.add2(vec!["start".to_string()], val.to_string(), &path_dict);
    }

    let paths = count_paths(&path_tree);
    assert_eq!(paths, 3509)
}
