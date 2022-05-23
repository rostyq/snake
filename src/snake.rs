use crate::wasm4::rect;
use crate::palette::set_draw_color;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Snake {
    pub body: Vec<Point>,
    pub direction: Point,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![
                Point { x: 2, y: 0},
                Point { x: 1, y: 0 },
                Point { x: 0, y: 0 },
            ],
            direction: Point { x: 1, y: 0 },
        }
    }

    pub fn draw(&self) {
        set_draw_color(0x43);
        for &Point { x, y } in self.body.iter() {
            rect(x * 8, y * 8, 8, 8);
        }

        let head = &self.body[0];

        set_draw_color(0x4);
        rect(head.x * 8, head.y * 8, 8, 8);
    }

}