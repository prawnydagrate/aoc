use std::time::{Duration, Instant};

fn parse<'a>(input: &'a str) -> impl Iterator<Item = (usize, Vec<usize>)> + use<'a> {
    input.lines().map(|line| {
        let mut split = line.trim().split(": ");
        (
            split.next().unwrap().parse().unwrap(),
            split
                .next()
                .unwrap()
                .split(' ')
                .map(|n| n.trim().parse().unwrap())
                .collect(),
        )
    })
}

fn works(target: usize, nums: &[usize]) -> bool {
    let len = nums.len();
    if len == 1 {
        return nums[0] == target;
    }
    let last_idx = len - 1;
    let last = nums[last_idx];
    if target % last == 0 && works(target / last, &nums[..last_idx]) {
        return true; // multiplication
    }
    if last <= target && works(target - last, &nums[..last_idx]) {
        return true; // addition
    }
    false
}

pub fn solve(input: &str) -> (usize, Duration) {
    let start = Instant::now();
    (
        parse(input)
            .map(|eq| {
                let (target, nums) = eq;
                if works(target, &nums) {
                    target
                } else {
                    0
                }
            })
            .sum(),
        Instant::now().duration_since(start),
    )
}

// for ferris-elf
pub fn run(input: &str) -> usize {
    parse(input)
        .map(|eq| {
            let (target, nums) = eq;
            if works(target, &nums) {
                target
            } else {
                0
            }
        })
        .sum()
}
