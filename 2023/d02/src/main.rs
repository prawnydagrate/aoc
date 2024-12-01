use std::cmp;

#[derive(Debug)]
struct Reveal {
    red: usize,
    green: usize,
    blue: usize,
}

impl Reveal {
    fn from(pairs: &[(&str, usize)]) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for (name, balls) in pairs {
            match *name {
                "red" => red = *balls,
                "green" => green = *balls,
                "blue" => blue = *balls,
                _ => (),
            }
        }
        Self { red, green, blue }
    }
}

struct Game {
    id: usize,
    reveals: Vec<Reveal>,
}

impl Game {
    fn from_line(line: &str) -> Self {
        let game_id_and_reveals_part: Vec<_> = line.split(": ").collect();
        let game_id_part = game_id_and_reveals_part[0];
        let reveals_part = game_id_and_reveals_part[1];
        let game_id = game_id_part
            .chars()
            .skip(5)
            .collect::<String>()
            .parse()
            .unwrap();
        let reveals_info = reveals_part.split("; ");
        let mut reveals = Vec::new();
        for reveal_info in reveals_info {
            let balls_colors = reveal_info.split(", ");
            let mut pairs = Vec::new();
            for ball_color in balls_colors {
                let balls_and_color: Vec<_> = ball_color.split(' ').collect();
                let color = balls_and_color[1];
                let balls: usize = balls_and_color[0].parse().unwrap();
                pairs.push((color, balls));
            }
            reveals.push(Reveal::from(&pairs));
        }
        Self {
            id: game_id,
            reveals,
        }
    }
}

fn solve(input: &str) -> usize {
    let mut powers = Vec::new();
    for line in input.lines() {
        let game = Game::from_line(line);
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for reveal in game.reveals {
            max_red = cmp::max(max_red, reveal.red);
            max_green = cmp::max(max_green, reveal.green);
            max_blue = cmp::max(max_blue, reveal.blue);
        }
        powers.push(max_red * max_green * max_blue);
    }
    powers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example() {
        let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\
";
        assert_eq!(solve(input), 2286);
    }
}

fn main() {
    let input = include_str!("input.txt").trim();
    println!("Got the sum {}", solve(input));
}
