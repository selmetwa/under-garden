use wasm_bindgen::prelude::*;

pub struct Cell {
    species: Species,
    ra: u8,
    rb: u8,
    clock: u8,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Species {
    Empty = 0,
    Wall = 1,
    Sand = 2,
}

impl Species {
    pub fn update(&self, cell: Cell) {
        match self {
            Species::Empty => {}
            Species::Wall => {}
            Species::Sand => {}
        }
    }
}
