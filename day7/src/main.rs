#![feature(int_abs_diff)]

fn min_f(xs: &[u32], x0: u32) -> u32 {
    xs.iter().map(|&x| x0.abs_diff(x)).sum()
}

fn min_f2(xs: &[u32], x0: u32) -> u32 {
    xs.iter().map(|&x| (0..=x0.abs_diff(x)).sum::<u32>()).sum()
}

fn main() {
    // initital state set up
    let xs = {
        let mut xs: Vec<u32> = include_str!("../input.txt")
            .split_terminator(",")
            .map(|s| s.parse().unwrap())
            .collect();
        xs.sort_unstable();
        xs
    };

    let x_min = *xs.first().unwrap();
    let x_max = *xs.last().unwrap();
    let min_gas = (x_min..=x_max).map(|n| min_f(&xs, n)).min().unwrap();
    let min_gas2 = (x_min..=x_max).map(|n| min_f2(&xs, n)).min().unwrap();

    println!("Day 7.1: {}", min_gas);
    println!("Day 7.2: {}", min_gas2);
}
