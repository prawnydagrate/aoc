use std::collections::HashSet;

#[derive(Debug)]
struct Guard {
    pos: isize,
    direction: isize,
}

impl Guard {
    fn next_pos(&self) -> isize {
        self.pos + self.direction
    }

    fn turn_right(&mut self, linelen: isize) {
        let d = self.direction;
        self.direction = if d == -linelen {
            1
        } else if d == 1 {
            linelen
        } else if d == linelen {
            -1
        } else if d == -1 {
            -linelen
        } else {
            panic!("bro is walking towards z78 🙏")
        }
    }
}

#[derive(Debug)]
struct Map {
    guard: Guard,
    positions: HashSet<usize>,
    mapdata: Vec<bool>,
    linelen: isize,
}

impl Map {
    fn advance(&mut self) -> bool {
        let next = self.guard.next_pos();
        let (l, d) = (self.linelen, self.guard.direction);
        // guard left the map
        if !(0..self.mapdata.len() as isize).contains(&next)
            || d == 1 && next % l == 0
            || d == -1 && self.guard.pos % l == 0
        {
            return true;
        }
        // check if obstacle
        if self.mapdata[next as usize] {
            self.guard.turn_right(l);
            return false;
        }
        self.guard.pos = next;
        self.positions.insert(next as usize);
        false
    }
}

fn parse(input: &str) -> Map {
    let mut guard = Guard {
        pos: 0,
        direction: 0,
    };
    let mut mapdata = Vec::new();
    let mut linelen = 0_isize;
    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        if !line.is_empty() {
            linelen = line.len() as isize;
        }
        for (j, c) in line.chars().enumerate() {
            let pos = i as isize * linelen + j as isize;
            match c {
                '.' => mapdata.push(false),
                '#' => mapdata.push(true),
                g => {
                    mapdata.push(false);
                    match g {
                        '^' => {
                            guard.pos = pos;
                            guard.direction = -linelen;
                        }
                        '>' => {
                            guard.pos = pos;
                            guard.direction = 1;
                        }
                        'v' => {
                            guard.pos = pos;
                            guard.direction = linelen;
                        }
                        '<' => {
                            guard.pos = pos;
                            guard.direction = -1;
                        }
                        _ => panic!("got character {c} at position ({i}, {j})"),
                    }
                }
            }
        }
    }
    assert!(linelen > 0);
    let first_pos = guard.pos as usize;
    Map {
        guard,
        positions: HashSet::from([first_pos]),
        mapdata,
        linelen,
    }
}

pub fn solve(input: &str) -> usize {
    let mut map = parse(input);
    while !map.advance() {}
    return map.positions.len();
}
