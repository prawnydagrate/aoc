use p12_again::solve as get_solution;
use std::time::SystemTime;

fn solve(input: &str) {
    for part in 1..=2 {
        let start = SystemTime::now();
        let sol = get_solution(input, part);
        let elapsed = start.elapsed().unwrap();
        println!("Part {part} solved in {elapsed:?}: {sol}");
    }
}

// #[cfg(test)]
// mod tests {
//     use super::solve;
//
//     #[test]
//     fn example1() {
//         let input = "\
// ";
//         assert_eq!(solve(input), todo!());
//     }
// }

fn main() {
    let input = include_str!("input.txt");
    solve(input);
}
