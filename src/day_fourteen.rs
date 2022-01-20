//! Advent of Code 2021
//! Day Fourteen - Extended Polymerization
#![allow(dead_code)]

use itertools::Itertools;

use crate::read_input;
use std::collections::HashMap;

pub(crate) fn day_fourteen_main() {
    println!("\nDay Fourteen Answers");

    let mut input = read_input::read_file("day_fourteen_input.txt");
    let poly = input.remove(0);

    let input_tup: Vec<(String, [String; 2])> = input
        .iter()
        .skip(1)
        .map(|line| parse_rule(line).unwrap())
        .collect();
    let poly_rules: HashMap<String, [String; 2]> = HashMap::from_iter(input_tup);

    let part_one = polymerize(10, &poly, &poly_rules);
    let part_two = polymerize(40, &poly, &poly_rules);

    println!("Part One, Diff: {part_one}");
    println!("Part Two, Diff: {part_two}");
}

fn polymerize(n: u8, poly: &str, poly_rules: &HashMap<String, [String; 2]>) -> u64 {
    let mut poly_counts: HashMap<&str, u64> = HashMap::new();
    for (i, _) in poly.char_indices().skip(1) {
        *poly_counts.entry(poly.get(i - 1..=i).unwrap()).or_insert(0) += 1;
    }

    for _ in 1..=n {
        let mut iter_counts: HashMap<&str, u64> = HashMap::with_capacity(poly_counts.len());

        for (key, val) in poly_counts.iter() {
            let rules_out = poly_rules.get(&key.to_string()).unwrap();
            *iter_counts.entry(rules_out[0].as_str()).or_insert(0) += val;
            *iter_counts.entry(rules_out[1].as_str()).or_insert(0) += val;
        }
        poly_counts = iter_counts;
    }

    let mut counts_map: HashMap<char, u64> = HashMap::new();
    for (key, val) in poly_counts.iter() {
        *counts_map.entry(key.chars().next().unwrap()).or_insert(0) += val;
        *counts_map.entry(key.chars().last().unwrap()).or_insert(0) += val;
    }

    let minmax = counts_map.iter().minmax_by_key(|entry| entry.1);

    if let Some((min, max)) = minmax.into_option() {
        let min_count = (*min.1 + 1) / 2;
        let max_count = (*max.1 + 1) / 2;
        max_count - min_count
    } else {
        unreachable!("error parsing minmax");
    }
}

fn parse_rule(line_map: &str) -> Option<(String, [String; 2])> {
    let tup_map: Vec<_> = line_map.trim().split(" -> ").collect();

    if tup_map.len().eq(&2) && tup_map[0].trim().len().eq(&2) && tup_map[1].trim().len().eq(&1) {
        let map_frm = tup_map[0].to_string();
        let map_to_a = map_frm.chars().next().unwrap().to_string() + tup_map[1];
        let map_to_b = {
            let mut map_to_b = tup_map[1].to_string();
            map_to_b.push(map_frm.chars().last().unwrap());
            map_to_b
        };

        Some((map_frm, [map_to_a, map_to_b]))
    } else {
        None
    }
}

#[test]
fn test_dayfourteen_parserule_none() {
    assert_eq!(parse_rule(""), None, "testing \"\"");
    assert_eq!(parse_rule(" -> "), None, "testing \" -> \"");
    assert_eq!(parse_rule("CH, B"), None, "testing \"CH, B\"");
    assert_eq!(parse_rule("C -> B"), None, "testing \"C -> B\"");
    assert_eq!(parse_rule("CH-> B"), None, "testing \"CH-> B\"");
    assert_eq!(parse_rule("CH ->B"), None, "testing \"CH ->B\"");
    assert_eq!(parse_rule("C -> BH"), None, "testing \"C -> BH\"");
    assert_eq!(parse_rule("CH -> BH"), None, "testing \"CH -> BH\"");
    assert_eq!(parse_rule("H CH -> B"), None, "testing \"H CH -> B\"");
}

#[test]
fn test_dayfourteen_parserule_some() {
    assert_eq!(
        parse_rule("CH -> B"),
        Some(("CH".to_string(), ["CB".to_string(), "BH".to_string()])),
        "testing \"CH -> B\""
    );
    assert_eq!(
        parse_rule(" CH -> B "),
        Some(("CH".to_string(), ["CB".to_string(), "BH".to_string()])),
        "testing \" CH -> B \""
    );
    assert_eq!(
        parse_rule("HH -> N"),
        Some(("HH".to_string(), ["HN".to_string(), "NH".to_string()])),
        "testing \"HH -> N\""
    );
    assert_eq!(
        parse_rule("CB -> H"),
        Some(("CB".to_string(), ["CH".to_string(), "HB".to_string()])),
        "testing \"CB -> H\""
    );
    assert_eq!(
        parse_rule("NH -> C"),
        Some(("NH".to_string(), ["NC".to_string(), "CH".to_string()])),
        "testing \"NH -> C\""
    );
    assert_eq!(
        parse_rule("HB -> C"),
        Some(("HB".to_string(), ["HC".to_string(), "CB".to_string()])),
        "testing \"HB -> C\""
    );
    assert_eq!(
        parse_rule("HC -> B"),
        Some(("HC".to_string(), ["HB".to_string(), "BC".to_string()])),
        "testing \"HC -> B\""
    );
    assert_eq!(
        parse_rule("HN -> C"),
        Some(("HN".to_string(), ["HC".to_string(), "CN".to_string()])),
        "testing \"HN -> C\""
    );
    assert_eq!(
        parse_rule("NN -> C"),
        Some(("NN".to_string(), ["NC".to_string(), "CN".to_string()])),
        "testing \"NN -> C\""
    );
    assert_eq!(
        parse_rule("BH -> H"),
        Some(("BH".to_string(), ["BH".to_string(), "HH".to_string()])),
        "testing \"BH -> H\""
    );
    assert_eq!(
        parse_rule("NC -> B"),
        Some(("NC".to_string(), ["NB".to_string(), "BC".to_string()])),
        "testing \"NC -> B\""
    );
    assert_eq!(
        parse_rule("NB -> B"),
        Some(("NB".to_string(), ["NB".to_string(), "BB".to_string()])),
        "testing \"NB -> B\""
    );
    assert_eq!(
        parse_rule("BN -> B"),
        Some(("BN".to_string(), ["BB".to_string(), "BN".to_string()])),
        "testing \"BN -> B\""
    );
    assert_eq!(
        parse_rule("BB -> N"),
        Some(("BB".to_string(), ["BN".to_string(), "NB".to_string()])),
        "testing \"BB -> N\""
    );
    assert_eq!(
        parse_rule("BC -> B"),
        Some(("BC".to_string(), ["BB".to_string(), "BC".to_string()])),
        "testing \"BC -> B\""
    );
    assert_eq!(
        parse_rule("CC -> N"),
        Some(("CC".to_string(), ["CN".to_string(), "NC".to_string()])),
        "testing \"CC -> N\""
    );
    assert_eq!(
        parse_rule("CN -> C"),
        Some(("CN".to_string(), ["CC".to_string(), "CN".to_string()])),
        "testing \"CN -> C\""
    );
}

#[test]
fn test_dayfourteen_polymerize_example() {
    let mut input = read_input::read_file("day_fourteen_test_input.txt");
    let poly = input.remove(0);

    let input_tup: Vec<(String, [String; 2])> = input
        .iter()
        .skip(1)
        .map(|line| parse_rule(line).unwrap())
        .collect();
    let poly_rules: HashMap<String, [String; 2]> = HashMap::from_iter(input_tup);

    assert_eq!(polymerize(10, &poly, &poly_rules), 1588);
    assert_eq!(polymerize(40, &poly, &poly_rules), 2188189693529);
}

#[test]
fn test_dayfourteen_polymerize_actual() {
    let mut input = read_input::read_file("day_fourteen_input.txt");
    let poly = input.remove(0);

    let input_tup: Vec<(String, [String; 2])> = input
        .iter()
        .skip(1)
        .map(|line| parse_rule(line).unwrap())
        .collect();
    let poly_rules: HashMap<String, [String; 2]> = HashMap::from_iter(input_tup);

    assert_eq!(polymerize(10, &poly, &poly_rules), 2375);
    assert_eq!(polymerize(40, &poly, &poly_rules), 1976896901756);
}
