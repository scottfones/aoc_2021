//! Advent of Code 2021
//! Day Three - Binary Diagnostic
#![allow(dead_code)]

use std::fmt::Debug;
use std::time::Instant;

use crate::read_input;

pub(crate) fn day_three_main() {
    println!("\nDay Three - Binary Diagnostic - Answers");
    let now = Instant::now();

    let input = read_input::read_file("day_three_input.txt");
    let _ = calc_gamma_eps(&input);
    let _ = calc_oxy_co2(&input);

    println!("Execution time: {}ms", now.elapsed().as_millis());
}

fn calc_gamma_eps(values: &[String]) -> u32 {
    let mut accumulator = vec![0_f32; values[0].len()];

    for line in values {
        for (i, c) in line.chars().enumerate() {
            accumulator[i] += c.to_digit(10).unwrap() as f32
        }
    }

    let mut gamma_arr = vec![0_u32; values[0].len()];
    let mut epsilon_arr = vec![0_u32; values[0].len()];

    for (i, val) in accumulator.iter_mut().enumerate() {
        *val /= values.len() as f32;
        if *val < 0.5 {
            gamma_arr[i] = 0;
            epsilon_arr[i] = 1;
        } else {
            gamma_arr[i] = 1;
            epsilon_arr[i] = 0;
        }
    }

    let gamma = bin_vec_to_dec(&gamma_arr);
    let epsilon = bin_vec_to_dec(&epsilon_arr);

    println!("Part One, Gamma x Epsilon: {}", gamma * epsilon);
    gamma * epsilon
}

#[test]
fn test_daythree_calc_gamma_eps() {
    let input_a = read_input::read_file("day_three_test_input.txt");
    assert_eq!(calc_gamma_eps(&input_a), 198);

    let input_b = read_input::read_file("day_three_input.txt");
    assert_eq!(calc_gamma_eps(&input_b), 2035764);
}

fn bin_string_to_dec(s_input: String) -> u32 {
    let mut dec_val = 0_u32;

    let n = s_input.len() as u32;
    for (i, val) in s_input.chars().enumerate() {
        dec_val += val.to_digit(10).unwrap() * 2_u32.pow(n - (i as u32 + 1));
    }
    dec_val
}

#[test]
fn test_daythree_bin_string_to_dec() {
    assert_eq!(bin_string_to_dec("10111".to_string()), 23);
    assert_eq!(bin_string_to_dec("01010".to_string()), 10);
}

fn bin_vec_to_dec(bin_vec: &[u32]) -> u32 {
    let mut dec_val = 0_u32;

    let n = bin_vec.len() as u32;
    for (i, val) in bin_vec.iter().enumerate() {
        dec_val += val * 2_u32.pow(n - (i as u32 + 1));
    }
    dec_val
}

#[test]
fn test_daythree_bin_vec_to_dec() {
    assert_eq!(bin_vec_to_dec(&[1, 0, 1, 1, 1]), 23);
    assert_eq!(bin_vec_to_dec(&[0, 1, 0, 1, 0]), 10);
}

#[derive(Debug)]
enum LifeTree {
    Empty,
    NonEmpty(Box<BitNode>),
}

impl LifeTree {
    fn add(&mut self, seq_val: String) {
        match *self {
            LifeTree::Empty => {
                let node_seq = seq_val.chars().take(1).collect();
                let mut new_node = BitNode::new_node(node_seq);

                if seq_val.len() > 1 {
                    if seq_val.chars().nth(1).unwrap().to_string().eq("0") {
                        new_node.count_zero += 1;
                        new_node.tree_zero.add(seq_val.chars().skip(1).collect());
                    } else {
                        new_node.count_one += 1;
                        new_node.tree_one.add(seq_val.chars().skip(1).collect());
                    }
                }
                *self = LifeTree::NonEmpty(Box::new(new_node));
            }
            LifeTree::NonEmpty(ref mut node) => {
                node.count += 1;
                if seq_val.len() > 1 {
                    if seq_val.chars().nth(1).unwrap() == '0' {
                        node.count_zero += 1;
                        node.tree_zero.add(seq_val.chars().skip(1).collect());
                    } else {
                        node.count_one += 1;
                        node.tree_one.add(seq_val.chars().skip(1).collect());
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct BitNode {
    count: u32,
    count_zero: u32,
    count_one: u32,
    tree_zero: LifeTree,
    tree_one: LifeTree,
    sequence: String,
}

impl BitNode {
    fn new_root() -> BitNode {
        BitNode {
            count: 0,
            count_zero: 0,
            count_one: 0,
            tree_zero: LifeTree::Empty,
            tree_one: LifeTree::Empty,
            sequence: "".to_string(),
        }
    }
    fn new_node(seq: String) -> BitNode {
        BitNode {
            count: 1,
            count_zero: 0,
            count_one: 0,
            tree_zero: LifeTree::Empty,
            tree_one: LifeTree::Empty,
            sequence: seq,
        }
    }
}

fn calc_oxy_co2(values: &[String]) -> u32 {
    let mut life_tree = LifeTree::NonEmpty(Box::new(BitNode::new_root()));

    for val in values {
        life_tree.add(format!(" {}", val));
    }

    let max_seq = find_max(&life_tree);
    let min_seq = find_min(&life_tree);

    let max_dec = bin_string_to_dec(max_seq);
    let min_dec = bin_string_to_dec(min_seq);

    let life_product = max_dec * min_dec;
    println!("Part Two, Oxy x CO2: {}", life_product);
    life_product
}

/// Used to find the oxygen generator rating according
/// to the following bit criteria:
///   - Determine the most common value (0 or 1) in the current bit position,
///     and keep only numbers with that bit in that position.
///   - If 0 and 1 are equally common, keep values with a 1 in the position
///     being considered.
fn find_max(tree: &LifeTree) -> String {
    match tree {
        LifeTree::Empty => "".to_string(),
        LifeTree::NonEmpty(node) => {
            if node.count_one >= node.count_zero {
                format!("{}{}", node.sequence, find_max(&node.tree_one))
            } else {
                format!("{}{}", node.sequence, find_max(&node.tree_zero))
            }
        }
    }
}

/// Used to find the CO2 scrubber rating according to the
/// following bit criteria:
///   - Determine the least common value (0 or 1) in the current bit position,
///     and keep only numbers with that bit in that position.
///   - If 0 and 1 are equally common, keep values with a 0 in the position
///     being considered.
fn find_min(tree: &LifeTree) -> String {
    match tree {
        LifeTree::Empty => "".to_string(),
        LifeTree::NonEmpty(node) => {
            if node.count_zero <= node.count_one && node.count_zero > 0 {
                format!("{}{}", node.sequence, find_min(&node.tree_zero))
            } else if node.count_one > 0 {
                format!("{}{}", node.sequence, find_min(&node.tree_one))
            } else if node.count_zero > 0 {
                format!("{}{}", node.sequence, find_min(&node.tree_zero))
            } else {
                node.sequence.clone()
            }
        }
    }
}

#[test]
fn test_daythree_find_max() {
    let mut life_tree = LifeTree::NonEmpty(Box::new(BitNode::new_root()));

    let values = read_input::read_file("day_three_test_input.txt");
    for val in &values {
        life_tree.add(format!(" {}", val));
    }

    assert_eq!(find_max(&life_tree), "10111".to_string());
}

#[test]
fn test_daythree_find_min() {
    let mut life_tree = LifeTree::NonEmpty(Box::new(BitNode::new_root()));

    let values = read_input::read_file("day_three_test_input.txt");
    for val in &values {
        life_tree.add(format!(" {}", val));
    }

    println!("{:#?}", life_tree);
    assert_eq!(find_min(&life_tree), "01010".to_string());
}
