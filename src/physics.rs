pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(_x: i32, _y: i32) -> Point {
        Point { x: _x, y: _y }
    }
    pub fn move_in_dir(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
