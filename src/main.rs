#[derive(Debug)]
struct Snake {
    head: (f32, f32),
    tail: Vec<(f32, f32)>,
    chr: char,
    pub dir: Direction,
}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let mut my_snake = Snake {
        head: (1.0, 1.0),
        tail: Vec::new(),
        chr: 'X',
        dir: Direction::Up,
    };
    println!("{:?}", my_snake);
    my_snake.dir = Direction::Down;
    println!("{:?}", my_snake);
}
