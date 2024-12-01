use p11_again::*;

const EXPANSION_FACTOR: usize = 2;

fn solve(input: &str) -> usize {
    let mut grid = Vec::new();
    let mut galaxies = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut positions = Vec::new();
        let mut is_empty = false;
        for (x, ch) in line.chars().enumerate() {
            let pos = Position {
                ch,
                x,
                y,
                x_cost: 1,
                y_cost: 1,
            };
            positions.push(pos);
            if ch == '#' {
                galaxies.push(pos);
                is_empty = false;
            }
        }
        if is_empty {
            for i in 0..positions.len() {
                positions[i].y_cost = EXPANSION_FACTOR;
            }
        }
        grid.append(&mut positions);
    }
    for x in 0..input.lines().next().unwrap().len() {
        let mut is_empty = true;
        let col: Vec<_> = grid
            .iter()
            .enumerate()
            .filter(|(_, pos)| x == pos.x)
            .map(|(idx, pos)| {
                if pos.ch == '#' {
                    is_empty = false;
                }
                idx
            })
            .collect();
        if is_empty {
            for idx in col {
                grid[idx].x_cost = EXPANSION_FACTOR;
            }
        }
    }
    let mut total = 0;
    println!("number of galaxies: {}", galaxies.len());
    for (i, a) in galaxies.iter().enumerate() {
        for b in galaxies.iter().skip(i + 1) {
            println!("pair {a:?}, {b:?}");
            let x_range = std::cmp::min(a.x, b.x)..std::cmp::max(a.x, b.x);
            let y_range = std::cmp::min(a.y, b.y)..std::cmp::max(a.y, b.y);
            for x in x_range {
                for pos in grid.iter().filter(|pos| x == pos.x) {
                    total += pos.x_cost;
                }
            }
            for y in y_range {
                for pos in grid.iter().filter(|pos| y == pos.y) {
                    total += pos.y_cost;
                }
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
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        println!("{}", solve(input));
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Got the answer: {}", solve(input));
}

// I DID PART 2 WITH THESE MATHEMATICAL OBSERVATIONS (not a single extra line of code lol):

/*
[example]
1 -> 292
10 -> 1,030 (+738)
100 -> 8,410 (+7,380)
1,000 -> 82,210 (+73,800)
10,000 -> 820,210 (+738,000)
100,000 -> 8,200,210 (+7,380,000)
1,000,000 -> 82,000,210 (+73,800,000)

[my input]
1 -> 9,146,944
10 -> 13,270,591 (+4,123,647)
100 -> 54,507,061 (+41,236,470)
1,000 -> 54,507,061 + 412,364,700 = 466,871,761
10,000 -> 466,871,761 + 4,123,647,000 = 4,590,518,761
100,000 -> 4,590,518,761 + 41,236,470,000 = 45,826,988,761
1,000,000 -> 45,826,988,761 + 412,364,700,000 = 458,191,688,761 (correct answer)
*/
