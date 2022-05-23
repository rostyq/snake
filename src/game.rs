use crate::snake::Snake;

pub struct Game {
  snake: Snake,
  frame_count: u64,
}

impl Game {
  pub fn new() -> Self {
    Self {
      snake: Snake::new(),
      frame_count: 0
    }
  }

  pub fn update(&mut self) {
    self.frame_count += 1;
    if self.frame_count % 15 == 0 {
      self.snake.update();
    }

    self.snake.draw();
  }
}