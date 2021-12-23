use std::collections::BTreeSet;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const SIZE: usize = WIDTH * HEIGHT;

fn main() {
    // initital state set up
    let grid: Vec<u8> = include_str!("../input.txt")
        .split_terminator("\n")
        .flat_map(|s| s.bytes().map(|b| b - b'0'))
        .collect();

    // let to_coord = |i| (i % WIDTH, i / WIDTH);
    // let to_idx = |(x, y)| y * WIDTH + x;
    let adjacent = |i: usize| {
        let row = i / WIDTH;
        [
            i.checked_sub(WIDTH),
            i.checked_sub(1).filter(|n| n / WIDTH == row),
            i.checked_add(WIDTH).filter(|&n| n < SIZE),
            i.checked_add(1).filter(|n| n / WIDTH == row),
        ]
        .into_iter()
        .flatten()
        .map(|i| (i, grid[i]))
    };
    let low_points = grid
        .iter()
        .enumerate()
        .filter(|(i, v)| adjacent(*i).all(|(_, n)| **v < n));

    let low_score: usize = low_points.clone().map(|(_, &n)| n as usize + 1).sum();

    let basins: Vec<_> = low_points
        .map(|(i, &v)| {
            let higher_adjacent = |(i, v)| adjacent(i).filter(move |&(_, n)| n < 9 && v < n);
            let mut basin = BTreeSet::from([i]);

            let mut to_visit: Vec<_> = higher_adjacent((i, v)).collect();
            while let Some((i, v)) = to_visit.pop() {
                let is_new = basin.insert(i);
                if is_new {
                    to_visit.extend(higher_adjacent((i, v)));
                }
            }

            basin
        })
        .collect();

    dbg!(&basins);

    let sizes = {
        let mut v: Vec<_> = basins.iter().map(|s| s.len()).collect();
        v.sort();
        v
    };

    let score: usize = sizes[sizes.len() - 3..].iter().product();

    println!("Day 9.1: {}", low_score);
    println!("Day 9.1: {}", score);
}
