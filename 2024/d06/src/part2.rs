use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Eq, PartialEq)]
enum GuardStatus {
    Left,
    Loop,
    Going,
}

#[derive(Debug, Clone)]
struct Map {
    guard: Guard,
    positions: HashSet<(isize, isize)>,
    mapdata: Vec<bool>,
    linelen: isize,
}

impl Map {
    fn advance(&mut self) -> GuardStatus {
        let next = self.guard.next_pos();
        let (l, d) = (self.linelen, self.guard.direction);
        // guard left the map
        if !(0..self.mapdata.len() as isize).contains(&next)
            || d == 1 && next % l == 0
            || d == -1 && self.guard.pos % l == 0
        {
            return GuardStatus::Left;
        }
        // check if obstacle
        if self.mapdata[next as usize] {
            self.guard.turn_right(l);
            return GuardStatus::Going;
        }
        self.guard.pos = next;
        if self.positions.insert((next, self.guard.direction)) {
            GuardStatus::Going
        } else {
            GuardStatus::Loop
        }
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
    let first_pos = (guard.pos, guard.direction);
    Map {
        guard,
        positions: HashSet::from([first_pos]),
        mapdata,
        linelen,
    }
}

pub fn solve(input: &str) -> usize {
    let orig_map = parse(input);
    (0..orig_map.mapdata.len())
        .map(|i| {
            if i == orig_map.guard.pos as usize {
                return 0;
            }
            let mut new_map = orig_map.clone();
            new_map.mapdata[i] = true;
            loop {
                let status = new_map.advance();
                match status {
                    GuardStatus::Going => (),
                    GuardStatus::Left => return 0,
                    GuardStatus::Loop => return 1,
                }
            }
        })
        .sum()
}
