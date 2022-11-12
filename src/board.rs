use crate::physics::{Direction, Point};
use crate::snake::Snake;
use rand::{thread_rng, Rng};

fn calc_pos(x: i32, y: i32) -> Point {
    let mut random = thread_rng();
    Point::new(random.gen_range(0..x), random.gen_range(0..y))
}
pub struct Board {
    pub width: i32,
    pub height: i32,
    pub snake: Snake,
    pub fruit: Point,
    pub game_over: bool,
    pub pause: bool,
}
impl Board {
    pub fn new(width: i32, height: i32) -> Board {
        Board {
            width: width,
            height: height,
            snake: Snake::new(calc_pos(width, height)),
            fruit: calc_pos(width, height),
            game_over: false,
            pause: false,
        }
    }
}
