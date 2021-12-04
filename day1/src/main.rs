#![feature(array_windows)]

fn main() {
    let input: Vec<u32> = include_str!("../input.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    let increases = input.array_windows().filter(|[a,b]| a < b).count();
    println!("Day 1.1: {}", increases);

    let increases = input.array_windows().filter(|[a,_,_,d]| a < d).count();
    println!("Day 1.2: {}", increases);
}
