use std::sync::Mutex;

use lazy_static::lazy_static;

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;

mod palette;
mod snake;
mod game;

use game::Game;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}


#[no_mangle]
fn start() {
    palette::set_palette([0xfbf7f3, 0xe5b083, 0x426e5d, 0x20283d]);
}

#[no_mangle]
fn update() {
    GAME.lock().unwrap().update();
}
