fn line_to_number(line: &str) -> usize {
    line.split(":")
        .nth(1)
        .unwrap()
        .chars()
        .filter(|ch| ch.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn solve(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let (race_time, record_distance) = (line_to_number(lines[0]), line_to_number(lines[1]));
    let mut nwins = 0;
    for hold_ms in 1..race_time {
        let dur_remaining = race_time - hold_ms;
        if dur_remaining * hold_ms > record_distance {
            nwins += 1;
        }
    }
    nwins
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example() {
        let input = "\
Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(solve(input), 71503);
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Got the answer: {}", solve(input));
}
