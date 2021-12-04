enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn main() {
    let steps: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(step, n)| {
            let n = n.parse().unwrap();
            match step {
                "forward" => Command::Forward(n),
                "down" => Command::Down(n),
                "up" => Command::Up(n),
                _ => unreachable!(),
            }
        })
        .collect();

    let (x, d) = steps.iter().fold((0, 0), |(x, d), command| match command {
        Command::Forward(n) => (x + n, d),
        Command::Up(n) => (x, d - n),
        Command::Down(n) => (x, d + n),
    });

    println!("Day 2.1: {}", x * d);

    let (x, d, _) = steps
        .iter()
        .fold((0, 0, 0), |(x, d, aim), command| match command {
            Command::Forward(n) => (x + n, d + (n * aim), aim),
            Command::Up(n) => (x, d, aim - n),
            Command::Down(n) => (x, d, aim + n),
        });

    println!("Day 2.2: {}", x * d);
}
