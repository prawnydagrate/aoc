use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn solve(input: &str) -> usize {
    let instructions_map_lines: Vec<_> = input.split("\n").map(|s| s.trim()).collect();
    let instructions = &instructions_map_lines[0];
    let map_lines = &instructions_map_lines[2..];
    let mut map = HashMap::new();
    let mut start_nodes = Vec::new();
    for line in map_lines {
        if line.is_empty() {
            continue;
        }
        let identifier_lrpair: Vec<_> = line.split(" = ").collect();
        let (identifier, lrpair) = (identifier_lrpair[0], identifier_lrpair[1]);
        if identifier.ends_with('A') {
            start_nodes.push(identifier);
        }
        let lrpair: Vec<_> = lrpair
            .replace('(', "")
            .replace(')', "")
            .split(", ")
            .map(|s| s.to_owned())
            .collect();
        map.insert(identifier, lrpair);
    }
    let results: Vec<_> = start_nodes
        .iter()
        .map(|&node| {
            let mut current_node = node;
            instructions
                .chars()
                .cycle()
                .enumerate()
                .find_map(|(idx, instruction)| {
                    let next_node = &map.get(current_node).unwrap()[match instruction {
                        'L' => 0,
                        'R' => 1,
                        _ => unreachable!(),
                    }];
                    if next_node.ends_with('Z') {
                        return Some(idx + 1);
                    }
                    current_node = next_node.as_str();
                    None
                })
                .unwrap()
        })
        .collect();
    lcm(&results)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example() {
        let input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(solve(input), 6);
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Starting the computation");
    let start = SystemTime::now();
    let answer = solve(input);
    let end = SystemTime::now();
    println!("Got the answer: {answer}");
    println!("Shit took {:?}", end.duration_since(start).unwrap());
    let mut file = match File::create("aoc2023-day8-part2-answer.txt") {
        Ok(f) => f,
        Err(e) => return eprintln!("error creating file: {e:?}"),
    };
    match file.write_all(format!("{answer}").as_bytes()) {
        Err(e) => return eprintln!("error writing to created file: {e:?}"),
        _ => (),
    }
}
