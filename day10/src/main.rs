#![feature(option_result_contains)]
fn main() {
    let (lines, errs): (Vec<_>, Vec<_>) = include_str!("../input.txt")
        .lines()
        .map(try_autocomplete_line)
        .partition(Result::is_ok);

    let completion_strings: Vec<_> = lines.into_iter().map(Result::unwrap).collect();
    let errs: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();

    dbg!(&completion_strings);
    dbg!(&errs);

    let err_score: usize = errs
        .into_iter()
        .map(|c| match c {
            b')' => 3,
            b']' => 57,
            b'}' => 1197,
            b'>' => 25137,
            _ => unreachable!(),
        })
        .sum();
    println!("Day 10.1: {}", err_score);

    let mut completion_scores: Vec<usize> = completion_strings
        .into_iter()
        .map(|s| {
            s.bytes().fold(0, |score, c| {
                let points = match c {
                    b')' => 1,
                    b']' => 2,
                    b'}' => 3,
                    b'>' => 4,
                    _ => unreachable!(),
                };
                score * 5 + points
            })
        })
        .collect();

    let middle_idx = completion_scores.len() / 2;
    let (_, middle, _) = completion_scores.select_nth_unstable(middle_idx);

    println!("Day 10.2: {}", middle);
}

const fn expected_closing(c: u8) -> u8 {
    match c {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        _ => unreachable!(),
    }
}

// Returns the string needed to autocomplete the line.
// Otherwise returns the first illegal character
fn try_autocomplete_line(s: &str) -> Result<String, u8> {
    let mut stack = Vec::new();
    for c in s.bytes() {
        match c {
            b'(' | b'{' | b'[' | b'<' => stack.push(expected_closing(c)),
            _ => {
                if !stack.pop().contains(&c) {
                    return Err(c);
                }
            }
        }
    }

    stack.reverse();
    Ok(String::from_utf8(stack).unwrap())
}
