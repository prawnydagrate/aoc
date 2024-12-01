use p10::*;
use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> usize {
    println!();
    let mut positions = HashMap::new();
    let mut animal_position = Position {
        from: None,
        row: 0,
        col: 0,
    };
    let nrows = input.lines().count();
    let ncols = input.lines().next().unwrap().len();
    let is_valid_position = |Position { row, col, from: _ }| row < nrows && col < ncols;
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let position = Position {
                from: None,
                row,
                col,
            };
            if ch == 'S' {
                animal_position = position;
            }
            positions.insert(position, Tile::from_char(ch));
        }
    }
    let directions = [
        Direction::North,
        Direction::East,
        Direction::West,
        Direction::South,
    ];
    let mut relevant_adj_nodes = Vec::new();
    let mut start_directions = Vec::new();
    for direction in directions {
        let position_adj_to_animal = match animal_position + direction {
            Some(pos) => pos,
            _ => continue,
        };
        if !is_valid_position(position_adj_to_animal) {
            continue;
        }
        let tile = positions.get(&position_adj_to_animal).unwrap();
        if let Tile::Connector(pipe) = tile {
            if pipe.get_directions().contains(&direction.get_opposite()) {
                start_directions.push(direction);
                relevant_adj_nodes.push(position_adj_to_animal);
            }
        }
    }
    assert_eq!(relevant_adj_nodes.len(), 2);
    let mut nodes = relevant_adj_nodes.clone();
    let mut pipe_nodes = vec![(nodes[0], nodes[1])];
    while nodes[0] != nodes[1] {
        let mut new_nodes = Vec::new();
        for node in nodes {
            let pipe = match positions.get(&node).unwrap() {
                Tile::Connector(p) => p,
                _ => panic!("a non-pipe made its way into the current nodes"),
            };
            for direction in pipe.get_directions() {
                if direction == node.from.unwrap() {
                    continue;
                }
                let adj = match node + direction {
                    Some(pos) => pos,
                    _ => continue,
                };
                new_nodes.push(adj);
            }
        }
        assert_eq!(new_nodes.len(), 2);
        nodes = new_nodes;
        pipe_nodes.push((nodes[0], nodes[1]));
    }
    let (start, finish_rev): (Vec<_>, Vec<_>) = pipe_nodes.into_iter().unzip();
    let start_to_finish: Vec<_> = std::iter::once(animal_position)
        .chain(start.into_iter().chain(finish_rev.into_iter().rev()))
        .collect();
    let start_node = Tile::Connector(Pipe::from_directions(&start_directions));
    positions.remove(&animal_position);
    positions.insert(animal_position, start_node);
    let mut inside;
    let mut inside_count = 0;
    for row in 0..nrows {
        inside = false;
        for col in 0..ncols {
            let pos = Position {
                row,
                col,
                from: None,
            };
            if start_to_finish.contains(&pos) {
                println!("start to finish contains row {row} col {col}");
                let tile = positions.get(&pos).unwrap();
                if let Tile::Connector(pipe) = tile {
                    if pipe.get_directions().contains(&Direction::North) {
                        println!("flipping inside at row {row} col {col}");
                        inside = !inside;
                    }
                }
            } else if inside {
                println!("incrementing inside count at row {row} col {col}");
                inside_count += 1;
            }
        }
    }
    inside_count
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example1() {
        let input = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(solve(input), 4);
    }

    #[test]
    fn example2() {
        let input = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(solve(input), 8);
    }

    #[test]
    fn example3() {
        let input = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(solve(input), 10);
    }

    #[test]
    fn example4() {
        let input = "\
.........
...F-7...
...S.|...
...L-J...
.........";
        assert_eq!(solve(input), 1);
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Got the answer: {}", solve(input));
}
