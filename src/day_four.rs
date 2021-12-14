//! Advent of Code 2021
//! Day Four - Giant Squid
#![allow(dead_code)]

use std::fmt::Debug;

use crate::read_input;

#[derive(Clone, Copy, Debug, PartialEq)]
enum BoardValue {
    Marked(u8),
    Unmarked(u8),
}

impl BoardValue {
    fn is_marked(&self) -> bool {
        match self {
            BoardValue::Marked(_) => true,
            BoardValue::Unmarked(_) => false,
        }
    }
}

#[derive(Clone, Debug)]
struct BingoBoard {
    r0: Vec<BoardValue>,
    r1: Vec<BoardValue>,
    r2: Vec<BoardValue>,
    r3: Vec<BoardValue>,
    r4: Vec<BoardValue>,
    is_winner: bool,
    values: Vec<u8>,
}

impl BingoBoard {
    fn new(values: Vec<u8>) -> BingoBoard {
        let mut b_vals: Vec<BoardValue> = values.iter().map(|&x| BoardValue::Unmarked(x)).collect();

        BingoBoard {
            r0: b_vals.drain(0..5).collect(),
            r1: b_vals.drain(0..5).collect(),
            r2: b_vals.drain(0..5).collect(),
            r3: b_vals.drain(0..5).collect(),
            r4: b_vals.drain(0..5).collect(),
            is_winner: false,
            values,
        }
    }
    fn check_board(&self) -> bool {
        let col_bingo = self.check_cols();
        let row_bingo = self.check_rows();

        col_bingo || row_bingo
    }
    fn check_cols(&self) -> bool {
        let mut col_vals: Vec<bool> = Vec::new();
        for i in 0..5 {
            col_vals.push(self.r0[i].is_marked());
            col_vals.push(self.r1[i].is_marked());
            col_vals.push(self.r2[i].is_marked());
            col_vals.push(self.r3[i].is_marked());
            col_vals.push(self.r4[i].is_marked());

            if col_vals.iter().all(|&x| x) {
                return true;
            }
            col_vals.clear();
        }
        false
    }
    fn check_rows(&self) -> bool {
        let bingo_r0 = self.r0.iter().all(|&x| x.is_marked());
        let bingo_r1 = self.r1.iter().all(|&x| x.is_marked());
        let bingo_r2 = self.r2.iter().all(|&x| x.is_marked());
        let bingo_r3 = self.r3.iter().all(|&x| x.is_marked());
        let bingo_r4 = self.r4.iter().all(|&x| x.is_marked());

        bingo_r0 || bingo_r1 || bingo_r2 || bingo_r3 || bingo_r4
    }
    fn mark_board(&mut self, target: u8) -> bool {
        if self.values.contains(&target) {
            let val_index = self.values.iter().position(|&x| x == target).unwrap();

            let row = val_index / 5;
            let row_index = val_index % 5;

            match row {
                0 => self.r0[row_index] = BoardValue::Marked(target),
                1 => self.r1[row_index] = BoardValue::Marked(target),
                2 => self.r2[row_index] = BoardValue::Marked(target),
                3 => self.r3[row_index] = BoardValue::Marked(target),
                4 => self.r4[row_index] = BoardValue::Marked(target),
                _ => (),
            }
            true
        } else {
            false
        }
    }
    fn row_sum(&self, row: &[BoardValue]) -> u32 {
        let mut sum = 0_u32;
        for r_val in row {
            match *r_val {
                BoardValue::Marked(_) => (),
                BoardValue::Unmarked(b_val) => sum += b_val as u32,
            }
        }
        sum
    }
    fn score_board(&self, multi: u8) -> u32 {
        let mut board_sum = 0_u32;
        board_sum += self.row_sum(&self.r0);
        board_sum += self.row_sum(&self.r1);
        board_sum += self.row_sum(&self.r2);
        board_sum += self.row_sum(&self.r3);
        board_sum += self.row_sum(&self.r4);

        let board_score = board_sum * multi as u32;
        println!("Board Sum: {}", board_sum);
        println!("Board Score: {}", board_score);
        board_score
    }
}

impl PartialEq for BingoBoard {
    fn eq(&self, other: &Self) -> bool {
        self.r0 == other.r0
            && self.r1 == other.r1
            && self.r2 == other.r2
            && self.r3 == other.r3
            && self.r4 == other.r4
    }
}

pub(crate) fn day_four_main() {
    println!("\nDay Four Answers");
    let mut input = read_input::read_file("day_four_input.txt");

    let boards = get_boards(&mut input);
    let rand_vals = get_randoms(&input);

    play_to_win(boards.clone(), &rand_vals);
    play_to_lose(boards, &rand_vals);
}

fn get_boards(input: &mut Vec<String>) -> Vec<BingoBoard> {
    let boards_raw = input.split_off(2);

    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut board_builder: Vec<u8> = Vec::new();
    for row in boards_raw {
        let row_vals: Vec<&str> = row.split(' ').collect();

        if !row.is_empty() {
            for val in row_vals {
                if val.eq("") {
                    continue;
                }
                board_builder.push(val.parse::<u8>().unwrap())
            }
        } else {
            boards.push(BingoBoard::new(board_builder.clone()));
            board_builder.clear();
        }
    }
    boards.push(BingoBoard::new(board_builder));
    boards
}

#[test]
fn test_dayfour_get_boards() {
    use crate::day_four::BoardValue::Unmarked;

    let mut input = read_input::read_file("day_four_test_input.txt");
    let boards = get_boards(&mut input);

    let b1 = BingoBoard {
        r0: vec![
            Unmarked(22),
            Unmarked(13),
            Unmarked(17),
            Unmarked(11),
            Unmarked(0),
        ],
        r1: vec![
            Unmarked(8),
            Unmarked(2),
            Unmarked(23),
            Unmarked(4),
            Unmarked(24),
        ],
        r2: vec![
            Unmarked(21),
            Unmarked(9),
            Unmarked(14),
            Unmarked(16),
            Unmarked(7),
        ],
        r3: vec![
            Unmarked(6),
            Unmarked(10),
            Unmarked(3),
            Unmarked(18),
            Unmarked(5),
        ],
        r4: vec![
            Unmarked(1),
            Unmarked(12),
            Unmarked(20),
            Unmarked(15),
            Unmarked(19),
        ],
        is_winner: false,
        values: vec![
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ],
    };
    assert_eq!(boards[0], b1);

    let b2 = BingoBoard {
        r0: vec![
            Unmarked(3),
            Unmarked(15),
            Unmarked(0),
            Unmarked(2),
            Unmarked(22),
        ],
        r1: vec![
            Unmarked(9),
            Unmarked(18),
            Unmarked(13),
            Unmarked(17),
            Unmarked(5),
        ],
        r2: vec![
            Unmarked(19),
            Unmarked(8),
            Unmarked(7),
            Unmarked(25),
            Unmarked(23),
        ],
        r3: vec![
            Unmarked(20),
            Unmarked(11),
            Unmarked(10),
            Unmarked(24),
            Unmarked(4),
        ],
        r4: vec![
            Unmarked(14),
            Unmarked(21),
            Unmarked(16),
            Unmarked(12),
            Unmarked(6),
        ],
        is_winner: false,
        values: vec![
            3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12,
            6,
        ],
    };
    assert_eq!(boards[1], b2);

    let b3 = BingoBoard {
        r0: vec![
            Unmarked(14),
            Unmarked(21),
            Unmarked(17),
            Unmarked(24),
            Unmarked(4),
        ],
        r1: vec![
            Unmarked(10),
            Unmarked(16),
            Unmarked(15),
            Unmarked(9),
            Unmarked(19),
        ],
        r2: vec![
            Unmarked(18),
            Unmarked(8),
            Unmarked(23),
            Unmarked(26),
            Unmarked(20),
        ],
        r3: vec![
            Unmarked(22),
            Unmarked(11),
            Unmarked(13),
            Unmarked(6),
            Unmarked(5),
        ],
        r4: vec![
            Unmarked(2),
            Unmarked(0),
            Unmarked(12),
            Unmarked(3),
            Unmarked(7),
        ],
        is_winner: false,
        values: vec![
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ],
    };
    assert_eq!(boards[2], b3);
}

fn get_randoms(input: &[String]) -> Vec<u8> {
    let rand_str: Vec<&str> = input[0].split(',').collect();
    let rand_vals: Vec<u8> = rand_str.iter().map(|x| x.parse::<u8>().unwrap()).collect();
    rand_vals
}

#[test]
fn test_dayfour_get_randoms() {
    let input = read_input::read_file("day_four_test_input.txt");
    let rand_vals = get_randoms(&input);
    let exp_vals: Vec<u8> = vec![
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];
    assert_eq!(rand_vals, exp_vals);
}

fn play_to_win(mut boards: Vec<BingoBoard>, nums: &[u8]) -> Option<u32> {
    for num in nums.iter() {
        for board in boards.iter_mut() {
            if board.mark_board(*num) && board.check_board() {
                println!("!!!BINGO!!!");
                println!("Last drawn: {}", num);
                return Some(board.score_board(*num));
            }
        }
    }
    None
}

#[test]
fn test_dayfour_play_to_win() {
    let mut input = read_input::read_file("day_four_test_input.txt");
    let boards = get_boards(&mut input);
    let rand_vals = get_randoms(&input);
    assert_eq!(play_to_win(boards, &rand_vals), Some(4512));

    let mut input = read_input::read_file("day_four_input.txt");
    let boards = get_boards(&mut input);
    let rand_vals = get_randoms(&input);
    assert_eq!(play_to_win(boards, &rand_vals), Some(60368));
}

fn play_to_lose(mut boards: Vec<BingoBoard>, nums: &[u8]) -> Option<u32> {
    let board_count = boards.len() as i8;
    let mut win_count = 0_i8;

    for num in nums.iter() {
        for board in boards.iter_mut() {
            if board.is_winner {
                continue;
            }

            if board.mark_board(*num) && board.check_board() {
                win_count += 1;
                board.is_winner = true;
            }

            if win_count == board_count {
                println!("!!!LOSER!!!");
                println!("Last drawn: {}", num);
                return Some(board.score_board(*num));
            }
        }
    }
    None
}

#[test]
fn test_dayfour_play_to_lose() {
    let mut input = read_input::read_file("day_four_test_input.txt");
    let boards = get_boards(&mut input);
    let rand_vals = get_randoms(&input);
    assert_eq!(play_to_lose(boards, &rand_vals), Some(1924));

    let mut input = read_input::read_file("day_four_input.txt");
    let boards = get_boards(&mut input);
    let rand_vals = get_randoms(&input);
    assert_eq!(play_to_lose(boards, &rand_vals), Some(17435));
}
