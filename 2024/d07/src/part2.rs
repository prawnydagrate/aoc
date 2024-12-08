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

fn nways(target: usize, nums: &[usize]) -> usize {
    let len = nums.len();
    if len == 1 {
        return if nums[0] == target { 1 } else { 0 };
    }
    let last_idx = len - 1;
    let last = nums[last_idx];
    let loglast = (last as f64).log10();
    let tenpw = 10_usize.pow(if loglast % 1. == 0. {
        loglast + 1.
    } else {
        loglast.ceil()
    } as u32) as usize;
    (if last > target {
        0
    } else {
        nways(target - last, &nums[..last_idx]) // addition
    } + if target % last != 0 {
        0
    } else {
        nways(target / last, &nums[..last_idx]) // multiplication
    } + if last > target {
        0
    } else {
        let sublast = target - last;
        if sublast % tenpw != 0 {
            0
        } else {
            nways(sublast / tenpw, &nums[..last_idx]) // concatenation
        }
    })
}

pub fn solve(input: &str) -> (usize, Duration) {
    let start = Instant::now();
    (
        parse(input)
            .map(|eq| {
                let (target, nums) = eq;
                let nways = nways(target, &nums);
                if nways > 0 {
                    target as usize
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
            let nways = nways(target, &nums);
            if nways > 0 {
                target as usize
            } else {
                0
            }
        })
        .sum()
}
