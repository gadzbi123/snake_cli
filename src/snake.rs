use crate::physics::{Direction, Point};
pub struct Snake {
    pub head: Point,
    pub tail: Vec<Point>,
    pub dir: Direction,
}
impl Snake {
    pub fn new(point: Point) -> Snake {
        Snake {
            head: point,
            tail: Vec::new(),
            dir: Direction::Up,
        }
    }
    pub fn tick_move(&mut self) {
        if self.dir == Direction::Up {
            self.head.y += 1;
        }
        if self.dir == Direction::Down {
            self.head.y -= 1;
        }
        if self.dir == Direction::Left {
            self.head.x -= 1;
        }
        if self.dir == Direction::Down {
            self.head.y -= 1;
        }
    }

    pub fn change_dir(&mut self, dir_new: Direction) {
        if dir_new == self.dir.opposite() {
            return;
        };
        self.dir = dir_new;
    }
}
