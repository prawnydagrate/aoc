#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    pub ch: char,
    pub x: usize,
    pub y: usize,
    pub x_cost: usize,
    pub y_cost: usize,
}
