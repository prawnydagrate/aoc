use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

fn solve(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let nlines = lines.len();
    let line_idx_rng = 0..(nlines as i32);
    let mut asterisks = HashMap::new();
    for (line_idx, line) in lines.iter().enumerate() {
        let mut ch_idx = 0;
        let line_len = line.len();
        let ch_idx_rng = 0..(line_len as i32);
        let bytes = line.as_bytes();
        while ch_idx < line_len {
            let ch = bytes[ch_idx] as char;
            if !ch.is_ascii_digit() {
                ch_idx += 1;
                continue;
            }
            let mut digit_indices = vec![ch_idx];
            loop {
                ch_idx += 1;
                if ch_idx == line_len || !(bytes[ch_idx] as char).is_ascii_digit() {
                    break;
                }
                digit_indices.push(ch_idx);
            }
            let mut adj_to_symbol = false;
            'check: for digit_idx in digit_indices.iter() {
                for adj_line_idx in [line_idx as i32 - 1, line_idx as i32, line_idx as i32 + 1] {
                    if !line_idx_rng.contains(&adj_line_idx) {
                        continue;
                    }
                    for adj_ch_idx in [
                        *digit_idx as i32 - 1,
                        *digit_idx as i32,
                        *digit_idx as i32 + 1,
                    ] {
                        if !ch_idx_rng.contains(&adj_ch_idx) {
                            continue;
                        }
                        let adj_ch =
                            lines[adj_line_idx as usize].as_bytes()[adj_ch_idx as usize] as char;
                        if adj_ch == '*' {
                            let full_number = digit_indices
                                .iter()
                                .map(|&digit_idx| (bytes[digit_idx] as char).to_string())
                                .collect::<String>()
                                .parse()
                                .unwrap();
                            asterisks
                                .entry(Position {
                                    row: adj_line_idx as usize,
                                    col: adj_ch_idx as usize,
                                })
                                .and_modify(|numbers: &mut Vec<usize>| numbers.push(full_number))
                                .or_insert(vec![full_number]);
                            break 'check;
                        }
                    }
                }
            }
            ch_idx += 1;
        }
    }
    let mut total = 0;
    for pair in asterisks.values() {
        if pair.len() == 2 {
            total += pair[0] * pair[1];
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example() {
        let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(solve(input), 467835);
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Got the sum {}", solve(input));
}
