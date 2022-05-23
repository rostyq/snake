use crate::wasm4::{
  PALETTE,
  DRAW_COLORS,
};

pub fn set_palette(value: [u32; 4]) {
  unsafe { *PALETTE = value; }
}

pub fn set_draw_color(idx: u16) {
  unsafe { *DRAW_COLORS = idx.into() }
}