use d06::{examples, part1, part2};

fn main() {
    let input = include_str!("../input.txt");
    // let input = examples::eginput0();
    println!("Part 1: {}", part1::solve(input));
    println!("Part 2: {}", part2::solve(input));
}
