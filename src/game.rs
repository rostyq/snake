use crate::snake::Snake;

pub struct Game {
  snake: Snake,
}

impl Game {
  pub fn new() -> Self {
    Self { snake: Snake::new() }
  }

  pub fn update(&self) {
    self.snake.draw();
  }
}