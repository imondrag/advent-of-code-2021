#![feature(split_array)]
use std::collections::HashMap;

use bitflags::bitflags;

type S = Segments;
bitflags! {
    struct Segments: u8 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
        const D = 1 << 3;
        const E = 1 << 4;
        const F = 1 << 5;
        const G = 1 << 6;

        // segments for real numbers
        const ZERO = 0b1110111;
        const ONE = 0b0100100;
        const TWO = 0b1011101;
        const THREE = 0b1101101;
        const FOUR = 0b0101110;
        const FIVE = 0b1101011;
        const SIX = 0b1111011;
        const SEVEN = 0b0100101;
        const EIGHT = 0b1111111;
        const NINE = 0b1101111;
    }
}

impl std::str::FromStr for Segments {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = s
            .bytes()
            .map(|c| 1u8 << (c - b'a'))
            .reduce(std::ops::BitOr::bitor)
            .ok_or("Could not reduce-or bits")?;

        Segments::from_bits(bits).ok_or("Invalid input")
    }
}

fn main() {
    // initital state set up
    let notes: Vec<(Vec<Segments>, Vec<Segments>)> = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let (signals, output) = s.split_once(" | ").unwrap();
            (
                signals
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
                output
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            )
        })
        .collect();

    let easy_digits = [S::ONE, S::FOUR, S::SEVEN, S::EIGHT];
    let unique_lens = easy_digits.map(|d| d.bits.count_ones());

    let easy_output = notes
        .iter()
        .flat_map(|(_, out)| out.iter().map(|s| s.bits.count_ones()))
        .filter(|l| unique_lens.contains(l))
        .count();

    let decoded: Vec<usize> = notes
        .into_iter()
        .map(|(mut signals, out)| {
            signals.sort_unstable_by_key(|s| s.bits.count_ones());
            let ([e1, e7, e4], rest) = signals.split_array_ref();
            let (l235 @ [_, _, _], rest) = rest.split_array_ref();
            let (l069 @ [_, _, _], _l8) = rest.split_array_ref();

            // figure decoding
            let e_adg = l235
                .iter()
                .copied()
                .reduce(std::ops::BitAnd::bitand)
                .unwrap();
            let e_abfg = l069
                .iter()
                .copied()
                .reduce(std::ops::BitAnd::bitand)
                .unwrap();

            // NUMBERS
            let zero = e_abfg | !e_adg;
            let one = *e1;
            let two = e_adg | !e_abfg;
            let three = e_adg | *e1;
            let four = *e4;
            let five = e_adg | e_abfg;
            let six = five | !*e1;
            let seven = *e7;
            let eight = Segments::all();
            let nine = e_adg | *e4;

            // mapping
            let map = HashMap::from([
                (zero, 0),
                (one, 1),
                (two, 2),
                (three, 3),
                (four, 4),
                (five, 5),
                (six, 6),
                (seven, 7),
                (eight, 8),
                (nine, 9),
            ]);

            let decoded_row: Vec<usize> = out
                .into_iter()
                .map(|s| map.get(&s).copied().unwrap())
                .collect();

            decoded_row
                .into_iter()
                .rev()
                .enumerate()
                .map(|(i, digit)| digit * 10usize.pow(i as u32))
                .sum()
        })
        .collect();

    println!("Day 8.1: {}", easy_output);
    println!("Day 8.2: {}", decoded.into_iter().sum::<usize>());
}
