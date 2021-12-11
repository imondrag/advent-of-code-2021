#![feature(drain_filter)]
use std::collections::HashMap;

const WIDTH: usize = 5;
const ROW_MASK: usize = 0b11111;
const COL_MASK: usize = 0b00001_00001_00001_00001_00001;

#[derive(Debug, Clone)]
struct Board {
    m: HashMap<u32, usize>, // tile value -> index to see status
    s: usize,
}

impl Board {
    fn set_check_win(&mut self, n: u32) -> bool {
        // we remove since we won't need the number anymore
        if let Some(i) = self.m.remove(&n) {
            self.s |= 1 << i;
            let row_start = (i / WIDTH) * WIDTH;
            let col_start = i % WIDTH;

            let row = (self.s >> row_start) & ROW_MASK == ROW_MASK;
            let col = (self.s >> col_start) & COL_MASK == COL_MASK;
            return row || col;
        }
        false
    }

    fn remaining_nums(&self) -> impl Iterator<Item = u32> + '_ {
        self.m.keys().copied()
    }
}

fn main() {
    let (nums, boards) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let mut boards: Vec<Board> = boards
        .split_terminator("\n\n")
        .map(|b| {
            let m = b
                .split_whitespace()
                .enumerate()
                .map(|(i, val)| (val.parse().unwrap(), i))
                .collect();
            Board { m, s: 0 }
        })
        .collect();

    let winning_boards = nums
        .split_terminator(',')
        .map(|s| s.parse().unwrap())
        .filter_map(|n: u32| {
            boards
                .drain_filter(|b| b.set_check_win(n))
                .map(|b| (b, n))
                .next()
        });

    let mut scores = winning_boards.map(|(b, n)| n * b.remaining_nums().sum::<u32>());

    println!("Day 4.1: {}", scores.next().unwrap());
    println!("Day 4.2: {}", scores.last().unwrap());
}
