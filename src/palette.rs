use crate::wasm4::PALETTE;

pub fn set_palette(value: [u32; 4]) {
  unsafe { *PALETTE = value; }
}