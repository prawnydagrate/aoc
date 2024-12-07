use d07::{examples, part1, part2};

fn main() {
    let input = include_str!("../input.txt");
    // let input = examples::eginput0();
    // Part 1
    {
        let (ans, dur) = part1::solve(input);
        println!("Part 1: {ans} in {dur:?}");
    }
    // Part 2
    {
        let (ans, dur) = part2::solve(input);
        println!("Part 2: {ans} in {dur:?}");
    }
}
