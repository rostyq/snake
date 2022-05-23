use crate::snake::Snake;
use crate::wasm4::{BUTTON_DOWN, BUTTON_LEFT, BUTTON_RIGHT, BUTTON_UP, GAMEPAD1};

pub struct Game {
    snake: Snake,
    frame_count: u64,
    prev_gamepad: u8,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
            frame_count: 0,
            prev_gamepad: 0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        self.input();
        if self.frame_count % 15 == 0 {
            self.snake.update();
        }

        self.snake.draw();
    }

    fn input(&mut self) {
        let gamepad = unsafe { *GAMEPAD1 };
        let pressed = gamepad & (gamepad ^ self.prev_gamepad);

        if pressed & BUTTON_UP != 0 {
            self.snake.up();
        }
        if pressed & BUTTON_DOWN != 0 {
            self.snake.down();
        }
        if pressed & BUTTON_LEFT != 0 {
            self.snake.left();
        }
        if pressed & BUTTON_RIGHT != 0 {
            self.snake.right();
        }

        self.prev_gamepad = gamepad;
    }
}
