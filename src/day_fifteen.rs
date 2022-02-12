//! Advent of Code 2021
//! Day Fifteen - Chiton
#![allow(dead_code)]

use crate::read_input;
use ndarray::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::time::Instant;

pub(crate) fn day_fifteen_main() {
    println!("\nDay Fifteen - Chiton - Answers");
    let now = Instant::now();

    let input = read_input::read_file("day_fifteen_input.txt");
    let chiton_map = create_map(input);

    match path_search(chiton_map) {
        Some(cost) => println!("Part One, Cost: {cost}"),
        None => println!("Part One, No Path Found"),
    }

    println!("Execution time: {}ms", now.elapsed().as_millis());
}

/// Dijkstra’s algorithm from
/// Artificial Intelligence: A Modern Approach Fourth Ed., Russell & Norvig
fn path_search(chiton_map: Array2<u32>) -> Option<u32> {
    let start_node = SearchNode::new(0, 0, 0, 0, 0);
    let mut frontier = BinaryHeap::new();
    frontier.push(start_node);
    let mut basecamp: HashMap<NodeState, SearchNode> = HashMap::new();

    while let Some(front_node) = frontier.pop() {
        match basecamp.get(&front_node.state) {
            Some(base_node) if base_node.path_cost <= front_node.path_cost => continue,
            _ => {
                basecamp.insert(front_node.state, front_node.clone());
            }
        }

        match get_next_acts(&chiton_map, &front_node) {
            None => return Some(front_node.path_cost),
            Some(next_nodes) => {
                for node in next_nodes {
                    frontier.push(node);
                }
            }
        }
    }

    None
}

fn create_map(input: Vec<String>) -> Array2<u32> {
    let m = input.len();
    let n = input[0].len();
    let mut chiton_map: Array2<u32> = Array2::zeros((m, n));
    for (i, line) in input.iter().enumerate() {
        let values_vec: Vec<u32> = line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u32)
            .collect();
        let values_arr = Array::from_vec(values_vec);
        values_arr.move_into(chiton_map.slice_mut(s![i, ..]));
    }
    chiton_map
}

fn get_next_acts(chiton_map: &Array2<u32>, node: &SearchNode) -> Option<Vec<SearchNode>> {
    let max_m = chiton_map.nrows() - 1;
    let max_n = chiton_map.ncols() - 1;

    if node.state.m.eq(&max_m) && node.state.n.eq(&max_n) {
        return None;
    }

    let mut next_nodes: Vec<SearchNode> = Vec::with_capacity(3);

    // Match Up and Down
    match (node.state.m.cmp(&0), node.state.m.cmp(&max_m)) {
        (Ordering::Equal, Ordering::Less) => {
            let m = node.state.m + 1;
            let n = node.state.n;
            let path_cost = node.path_cost + chiton_map[[m, n]];
            let p_m = node.state.m;
            let p_n = node.state.n;
            let node_down = SearchNode::new(m, n, path_cost, p_m, p_n);
            next_nodes.push(node_down);
        }
        (Ordering::Greater, Ordering::Less) => {
            let m = node.state.m - 1;
            let n = node.state.n;
            let path_cost = node.path_cost + chiton_map[[m, n]];
            let p_m = node.state.m;
            let p_n = node.state.n;
            let node_up = SearchNode::new(m, n, path_cost, p_m, p_n);
            next_nodes.push(node_up);

            let m = node.state.m + 1;
            let n = node.state.n;
            let path_cost = node.path_cost + chiton_map[[m, n]];
            let node_down = SearchNode::new(m, n, path_cost, p_m, p_n);
            next_nodes.push(node_down);
        }
        (Ordering::Less, Ordering::Equal) => {
            let m = node.state.m - 1;
            let n = node.state.n;
            let path_cost = node.path_cost + chiton_map[[m, n]];
            let p_m = node.state.m;
            let p_n = node.state.n;
            let node_up = SearchNode::new(m, n, path_cost, p_m, p_n);
            next_nodes.push(node_up);
        }
        _ => {}
    }

    // Match Left and Right
    match (node.state.n.cmp(&0), node.state.n.cmp(&max_n)) {
        (Ordering::Equal, Ordering::Less) => {
            let m = node.state.m;
            let n = node.state.n + 1;
            let path_cost = node.path_cost + chiton_map[[m, n]];
            let p_m = node.state.m;
            let p_n = node.state.n;
            let node_right = SearchNode::new(m, n, path_cost, p_m, p_n);
            next_nodes.push(node_right);
        }
        (Ordering::Greater, Ordering::Less) => {
            let m = node.state.m;
            let n = node.state.n - 1;
            let path_cost = node.path_cost + chiton_map[[m, n]];
            let p_m = node.state.m;
            let p_n = node.state.n;
            let node_left = SearchNode::new(m, n, path_cost, p_m, p_n);
            next_nodes.push(node_left);

            let m = node.state.m;
            let n = node.state.n + 1;
            let path_cost = node.path_cost + chiton_map[[m, n]];
            let node_right = SearchNode::new(m, n, path_cost, p_m, p_n);
            next_nodes.push(node_right);
        }
        (Ordering::Less, Ordering::Equal) => {
            let m = node.state.m;
            let n = node.state.n - 1;
            let path_cost = node.path_cost + chiton_map[[m, n]];
            let p_m = node.state.m;
            let p_n = node.state.n;
            let node_left = SearchNode::new(m, n, path_cost, p_m, p_n);
            next_nodes.push(node_left);
        }
        _ => {}
    }

    Some(next_nodes)
}

#[derive(Clone, Debug)]
enum NodeAction {
    Down,
    Left,
    None,
    Right,
    Up,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct NodeState {
    m: usize,
    n: usize,
}

impl NodeState {
    fn new(m: usize, n: usize) -> NodeState {
        NodeState { m, n }
    }

    fn magnitude(&self) -> usize {
        self.m * self.m + self.n * self.n
    }
}

#[derive(Clone, Debug)]
struct SearchNode {
    action: NodeAction,
    state: NodeState,
    path_cost: u32,
    parent: NodeState,
}

impl SearchNode {
    fn new(m: usize, n: usize, path_cost: u32, p_m: usize, p_n: usize) -> SearchNode {
        use std::cmp::Ordering::*;
        let action = match (m.cmp(&p_m), n.cmp(&p_n)) {
            (Less, Equal) => NodeAction::Up,
            (Equal, Equal) => NodeAction::None,
            (Equal, Less) => NodeAction::Left,
            (Equal, Greater) => NodeAction::Right,
            (Greater, Equal) => NodeAction::Down,
            _ => unreachable!("new node direction cannot be diagonal"),
        };

        let state = NodeState::new(m, n);
        let parent = NodeState::new(p_m, p_n);

        SearchNode {
            action,
            state,
            path_cost,
            parent,
        }
    }
}

impl Eq for SearchNode {}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Note reverse ordering to force min-heap
        // If path cost is equal, prefer larger distance
        other
            .path_cost
            .cmp(&self.path_cost)
            .then_with(|| self.state.magnitude().cmp(&other.state.magnitude()))
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.path_cost.eq(&other.path_cost)
            && self.parent.eq(&other.parent)
            && self.state.eq(&other.state)
    }
}

#[test]
fn test_dayfifteen_create_map() {
    let input = vec!["012".to_string(), "345".to_string(), "678".to_string()];
    let test_map = create_map(input);

    assert_eq!(test_map.ndim(), 2);
    for ((m, n), &v) in test_map.indexed_iter() {
        let i = (3 * m + n) as u32;
        assert_eq!(v, i, "testing: m:{m}, n:{n} and v{v}");
    }
}

#[test]
fn test_dayfifteen_search_node_queue_order() {
    let a = SearchNode::new(0, 1, 5, 0, 0);
    let b = SearchNode::new(1, 0, 1, 0, 0);
    let c = SearchNode::new(0, 0, 0, 0, 0);
    let d = SearchNode::new(1, 1, 1, 0, 1);

    let mut heap = BinaryHeap::new();
    heap.push(a);
    heap.push(b);
    heap.push(c);
    heap.push(d);

    assert_eq!(heap.pop().unwrap().path_cost, 0);
    assert_eq!(heap.peek().unwrap().path_cost, 1);
    assert_eq!(heap.pop().unwrap().state.magnitude(), 2);
    assert_eq!(heap.peek().unwrap().path_cost, 1);
    assert_eq!(heap.pop().unwrap().state.magnitude(), 1);
    assert_eq!(heap.pop().unwrap().path_cost, 5);
}

#[test]
fn test_dayfifteen_path_search_part_one_example() {
    let input = read_input::read_file("day_fifteen_test_input.txt");
    let chiton_map = create_map(input);

    assert_eq!(path_search(chiton_map), Some(40));
}

#[test]
fn test_dayfifteen_path_search_part_one_actual() {
    let input = read_input::read_file("day_fifteen_input.txt");
    let chiton_map = create_map(input);

    assert_eq!(path_search(chiton_map), Some(687));
}
