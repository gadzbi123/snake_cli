pub struct Point<X, Y> {
    x: X,
    y: Y,
}

pub struct Snake<'a> {
    head: Point<usize, usize>,
    tail: Vec<Point<usize, usize>>,
    chr: &'a str,
    pub dir: Direction,
}
impl Snake<'static> {
    pub fn new(width: usize, height: usize) -> Snake<'static> {
        Snake {
            head: Point {
                x: width / 2,
                y: height / 2,
            },
            tail: Vec::new(),
            chr: "X",
            dir: Direction::Up,
        }
    }
    fn tick_move(&mut self) {
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
    fn change_dir(&mut self, dir: Direction) {}
}
#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
