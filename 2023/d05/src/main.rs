use std::{cmp, ops::Range};

#[derive(Debug)]
struct MapLine {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl MapLine {
    fn from_slice(v: &[usize]) -> Self {
        Self {
            destination_range_start: v[0],
            source_range_start: v[1],
            range_length: v[2],
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    source_ranges: Vec<Range<usize>>,
    destination_ranges: Vec<Range<usize>>,
}

impl Map {
    fn from_lines(lines: &str) -> Self {
        let (mut source_ranges, mut destination_ranges) = (Vec::new(), Vec::new());
        for line in lines.lines() {
            let line = line.trim();
            if line.is_empty() || line.contains(':') {
                continue;
            }
            let map_line = MapLine::from_slice(
                &line
                    .split(' ')
                    .map(|nstr| nstr.trim().parse().unwrap())
                    .collect::<Vec<_>>(),
            );
            let MapLine {
                source_range_start,
                destination_range_start,
                range_length,
            } = map_line;
            source_ranges.push(source_range_start..source_range_start + range_length);
            destination_ranges
                .push(destination_range_start..destination_range_start + range_length);
        }
        Self {
            source_ranges,
            destination_ranges,
        }
    }

    fn find_destination_value(&self, source_value: usize) -> usize {
        for (idx, source_range) in self.source_ranges.iter().enumerate() {
            if source_range.contains(&source_value) {
                let destination_range = &self.destination_ranges[idx];
                return destination_range
                    .clone()
                    .nth(source_value - source_range.start)
                    .unwrap();
            }
        }
        source_value
    }
}

fn find_location_number(seed: usize, maps: &Vec<Map>, idx: usize) -> usize {
    if idx == maps.len() - 1 {
        return maps.last().unwrap().find_destination_value(seed);
    }
    find_location_number(maps[idx].find_destination_value(seed), maps, idx + 1)
}

fn solve(input: &str) -> usize {
    let sections: Vec<_> = input.split("\n\n").collect();
    let seed_ranges: Vec<usize> = sections[0]
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .map(|nstr| nstr.trim().parse().unwrap())
        .collect();
    let seed_ranges: Vec<_> = seed_ranges.chunks(2).map(|s| s.to_vec()).collect();
    let maps: Vec<_> = sections[1..]
        .iter()
        .map(|&lines| Map::from_lines(lines))
        .collect();
    let mut minimum = usize::MAX;
    for seed_range in seed_ranges.into_iter() {
        let start = seed_range[0];
        let end = start + seed_range[1];
        println!("in range {start}->{}", end - 1);
        for seed in start..end {
            let loc_num = find_location_number(seed, &maps, 0);
            minimum = cmp::min(minimum, loc_num);
        }
    }
    minimum
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example() {
        let input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(solve(input), 46);
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Got the minimum: {}", solve(input));
}
