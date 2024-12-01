use std::collections::HashSet;

fn get_next_value(history: &[i32]) -> i32 {
    // find all differences
    // return history.last().unwrap() + get_next_value(differences)
    let hash_set = HashSet::<_>::from_iter(history.iter().cloned());
    println!("set: {hash_set:?}");
    if hash_set.len() == 1 {
        println!("returning last because all were equal");
        return *history.first().unwrap();
    }
    let differences: Vec<_> = history.windows(2).map(|adj| adj[1] - adj[0]).collect();
    println!("differences: {differences:?}");
    history.first().unwrap() - get_next_value(&differences)
}

fn solve(input: &str) -> i32 {
    let mut histories = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let numbers: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|strn| strn.parse().unwrap())
            .collect();
        histories.push(numbers);
    }
    println!("{histories:?}");
    let mut total = 0;
    for history in histories {
        let next_value = get_next_value(&history);
        println!("got next value: {next_value}");
        total += next_value;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example() {
        let input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(solve(input), 2);
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Got the sum: {}", solve(input));
}
