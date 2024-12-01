fn flip_axis(v: Vec<&str>) -> Vec<String> {
    let mut ret = Vec::new();
    for i in 0..v[0].len() {
        ret.push(
            v.iter()
                .map(|s| s.chars().skip(i).next().unwrap())
                .collect::<String>(),
        );
    }
    ret
}

fn solve(input: &str) -> usize {
    let mut total = 0;
    for pattern in input.split("\n\n").map(|p| p.trim()) {
        let lines: Vec<_> = pattern.lines().enumerate().collect();
        'main: for two in lines.windows(2) {
            if two[0].1 == two[1].1 {
                let mut before = lines[..two[0].0].iter().rev();
                let mut after = lines[two[1].0 + 1..].iter();
                loop {
                    match (before.next(), after.next()) {
                        (Some(s1), Some(s2)) => {
                            if s1 != s2 {
                                continue 'main;
                            }
                        }
                        (None, None) => continue 'main,
                        _ => break,
                    }
                }
                total += two[1].0;
            }
        }
        let cols: Vec<_> = flip_axis(pattern.lines().collect())
            .into_iter()
            .enumerate()
            .collect();
        'main: for two in cols.windows(2) {
            if two[0].1 == two[1].1 {
                let mut before = cols[..two[0].0].iter().rev();
                let mut after = cols[two[1].0 + 1..].iter();
                loop {
                    match (before.next(), after.next()) {
                        (Some(s1), Some(s2)) => {
                            if s1 != s2 {
                                continue 'main;
                            }
                        }
                        _ => break,
                    }
                }
                total += 100 * two[1].0;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example1() {
        let input = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(solve(input), 405);
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Got the answer: {}", solve(input));
}
