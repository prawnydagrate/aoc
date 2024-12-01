fn check_seq(seq: &str, nums: &[usize]) -> bool {
    let mut lengths = Vec::new();
    let chars: Vec<_> = seq.chars().collect();
    let mut idx = 0;
    while idx < chars.len() {
        let ch = chars[idx];
        if ch == '.' {
            idx += 1;
            continue;
        }
        // ch == '#'
        let mut length = 0;
        loop {
            let ch = chars[idx];
            if idx + 1 == chars.len() && ch == '#' {
                lengths.push(length + 1);
                break;
            }
            if ch != '#' {
                lengths.push(length);
                break;
            }
            idx += 1;
            length += 1;
        }
        idx += 1;
    }
    lengths == nums
}

fn find_combos(seq: &str, nums: &[usize]) -> usize {
    if !seq.contains('?') {
        if check_seq(seq, nums) {
            return 1;
        }
        return 0;
    }
    let qmark_idx = seq
        .chars()
        .enumerate()
        .find_map(|(idx, ch)| if ch == '?' { Some(idx) } else { None })
        .unwrap();
    let a: String = seq
        .chars()
        .take(qmark_idx)
        .chain(std::iter::once('.'))
        .chain(seq.chars().skip(qmark_idx + 1))
        .collect();
    let b: String = seq
        .chars()
        .take(qmark_idx)
        .chain(std::iter::once('#'))
        .chain(seq.chars().skip(qmark_idx + 1))
        .collect();
    find_combos(&a, nums) + find_combos(&b, nums)
}

fn solve(input: &str) -> usize {
    let mut total = 0;
    for (idx, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut seq_nums = line.split_ascii_whitespace();
        let seq = seq_nums.next().unwrap();
        let nums: Vec<usize> = seq_nums
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.trim().parse().unwrap())
            .collect();
        println!("line {idx}: {seq}; {nums:?}");
        total += find_combos(&seq, &nums);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::{check_seq, solve};

    #[test]
    fn example1() {
        let input = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(solve(input), 21);
    }

    #[test]
    fn test_check_seq() {
        let seq = "..###.#..####.#####........#..##.....#";
        let nums = [3, 1, 4, 5, 1, 2, 1];
        assert!(check_seq(seq, &nums));
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Got the answer: {}", solve(input));
}
