use std::ops::Sub;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Pixel {
    Space,
    Galaxy,
}

impl Pixel {
    pub fn from_char(ch: char) -> Self {
        match ch {
            '.' => Self::Space,
            '#' => Self::Galaxy,
            _ => panic!("invalid pixel: {ch}"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
    pub data: Pixel,
}

impl Sub<Self> for Coordinate {
    type Output = usize;

    fn sub(self, other: Self) -> Self::Output {
        (self.x as isize - other.x as isize).abs() as usize
            + (self.y as isize - other.y as isize).abs() as usize
    }
}
