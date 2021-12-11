const STATES: usize = 8 + 1;

fn main() {
    let mut state_counts = [0u64; STATES];

    // initital state set up
    include_str!("../input.txt")
        .split_terminator(",")
        .map(|s| s.parse().unwrap())
        .for_each(|n: usize| {
            state_counts[n] += 1;
        });

    // simulate days
    // for t in 0..80 {
    for t in 0..256 {
        let s0 = t % STATES;
        let birthdays = std::mem::take(&mut state_counts[s0]);

        // reset parents
        state_counts[(s0 + 7) % STATES] += birthdays;
        // add children
        state_counts[(s0 + 9) % STATES] += birthdays;
    }

    let total: u64 = state_counts.iter().sum();
    println!("Day 6.1: {}", total);
}
