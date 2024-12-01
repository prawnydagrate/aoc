use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Add;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Pipe {
    pub fn from_char(ch: char) -> Self {
        use Pipe::*;
        match ch {
            '|' => NorthSouth,
            '-' => EastWest,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            _ => panic!("invalid pipe character: {ch}"),
        }
    }

    pub fn from_directions(directions: &[Direction]) -> Self {
        use Direction::*;
        use Pipe::*;
        match directions {
            [North, South] | [South, North] => NorthSouth,
            [East, West] | [West, East] => EastWest,
            [North, East] | [East, North] => NorthEast,
            [North, West] | [West, North] => NorthWest,
            [South, West] | [West, South] => SouthWest,
            [South, East] | [East, South] => SouthEast,
            _ => panic!("invalid directions"),
        }
    }

    pub fn get_directions(&self) -> [Direction; 2] {
        use Direction::*;
        use Pipe::*;
        match self {
            NorthSouth => [North, South],
            EastWest => [East, West],
            NorthEast => [North, East],
            NorthWest => [North, West],
            SouthWest => [South, West],
            SouthEast => [South, East],
        }
    }
}

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Pipe::*;
        write!(
            f,
            "{}",
            match self {
                NorthSouth => '|',
                EastWest => '—',
                NorthEast => 'L',
                NorthWest => 'J',
                SouthWest => '7',
                SouthEast => 'F',
            }
        )
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Tile {
    Animal,
    Connector(Pipe),
    Ground,
}

impl Tile {
    pub fn from_char(ch: char) -> Self {
        use Tile::*;
        match ch {
            'S' => Animal,
            '.' => Ground,
            _ => Connector(Pipe::from_char(ch)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub from: Option<Direction>,
    pub row: usize,
    pub col: usize,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        (self.row, self.col) == (other.row, other.col)
    }
}

impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
    }
}

impl Add<Direction> for Position {
    type Output = Option<Self>;

    fn add(self, other: Direction) -> Self::Output {
        let mut ret = self.clone();
        use Direction::*;
        match other {
            North => {
                ret.from = Some(South);
                if ret.row == 0 {
                    return None;
                }
                ret.row -= 1;
            }
            East => {
                ret.from = Some(West);
                ret.col += 1;
            }
            South => {
                ret.from = Some(North);
                ret.row += 1;
            }
            West => {
                ret.from = Some(East);
                if ret.col == 0 {
                    return None;
                }
                ret.col -= 1;
            }
        }
        Some(ret)
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn get_opposite(&self) -> Self {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}
