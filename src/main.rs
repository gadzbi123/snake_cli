mod snake;
use crate::snake::{Point, Snake};
struct Board<'a> {
    width: usize,
    height: usize,
    border: &'a str,
    snake: Snake<'a>,
}
impl Board<'static> {
    fn new(width: usize, height: usize) -> Board<'static> {
        Board {
            width: width,
            height: height,
            border: "#",
            snake: Snake::new(width, height),
        }
    }

    fn print(&self) {
        let top_bot_line = self.border.repeat(self.width);
        println!("{}", top_bot_line);
        for i in 0..(self.height - 2) {
            let mut mid_line =
                (self.border.to_owned() + " ".repeat(self.width - 2).as_str() + self.border);

            if i == self.snake.head.y {
                mid_line.replace_range(self.snake.head.x..self.snake.head.x + 1, "X");
            }
            println!("{}", mid_line);
        }
        println!("{}", top_bot_line);
    }
}
fn main() {
    let mut my_board = Board::new(25, 15);
    my_board.print();
}
