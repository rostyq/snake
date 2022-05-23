use crate::palette::set_draw_color;
use crate::snake::{Point, Snake};
use crate::wasm4::{BUTTON_DOWN, BUTTON_LEFT, BUTTON_RIGHT, BUTTON_UP, GAMEPAD1, blit, BLIT_2BPP};
use fastrand::Rng;

const FRUIT_WIDTH: u32 = 8;
const FRUIT_HEIGHT: u32 = 8;
const FRUIT_FLAGS: u32 = BLIT_2BPP;
const FRUIT_SPRITE: [u8; 16] = [
    0x00, 0xa0, 0x02, 0x00, 0x0e, 0xf0, 0x36, 0x5c, 0xd6, 0x57, 0xd5, 0x57, 0x35, 0x5c, 0x0f, 0xf0,
];

pub struct Game {
    rng: Rng,
    snake: Snake,
    frame_count: u64,
    prev_gamepad: u8,
    fruit: Point,
}

impl Game {
    pub fn new() -> Self {
        let rng = Rng::with_seed(42);
        Self {
            snake: Snake::new(),
            frame_count: 0,
            prev_gamepad: 0,
            fruit: Point {
                x: rng.i32(0..20),
                y: rng.i32(0..20),
            },
            rng,
        }
    }

    fn draw_fruit(&self) {
        set_draw_color(0x4320);
        blit(&FRUIT_SPRITE, self.fruit.x * 8, self.fruit.x * 8, FRUIT_WIDTH, FRUIT_HEIGHT, FRUIT_FLAGS);
    }

    fn render(&self) {
        self.snake.draw();
        self.draw_fruit();
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        self.input();

        if self.frame_count % 15 == 0 {
            self.snake.update();
        }

        self.render();
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
