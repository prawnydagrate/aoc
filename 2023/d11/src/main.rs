use p11::*;

fn flip_axis(list: Vec<String>) -> Vec<String> {
    println!("starting flip");
    let mut flipped = Vec::new();
    for idx in 0..list[0].len() {
        flipped.push(
            list.iter()
                .map(|s| s.chars().skip(idx).next().unwrap())
                .collect::<String>(),
        );
    }
    println!("finished flip");
    flipped
}

const EXPANSION_FACTOR: usize = 100;

fn solve(input: &str) -> usize {
    // expansion of universe
    let mut rows = Vec::new();
    for line in input.lines() {
        let line = line.trim().to_owned();
        let empty = ".".repeat(line.len());
        rows.push(line.clone());
        if line == empty {
            for _ in 1..EXPANSION_FACTOR {
                rows.push(line.clone());
            }
        }
    }
    println!("finished row expansion");
    let nrows = rows.len();
    let mut cols = Vec::new();
    for col in flip_axis(rows) {
        let empty = ".".repeat(col.len());
        cols.push(col.clone());
        if col == empty {
            for _ in 1..EXPANSION_FACTOR {
                cols.push(col.clone());
            }
        }
    }
    println!("finished column expansion");
    let ncols = cols.len();
    let string_grid = flip_axis(cols);
    // convert into list of coordinates
    let mut galaxies = Vec::new();
    for y in 0..nrows {
        for x in 0..ncols {
            let data = Pixel::from_char(string_grid[y].chars().skip(x).next().unwrap());
            if let Pixel::Galaxy = data {
                let coordinate = Coordinate { x, y, data };
                galaxies.push(coordinate);
            }
        }
    }
    println!("got galaxies");
    let mut total_distance = 0;
    for (idx, a) in galaxies.iter().enumerate() {
        for b in galaxies.iter().skip(idx + 1) {
            total_distance += b.clone() - a.clone();
        }
    }
    total_distance
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
