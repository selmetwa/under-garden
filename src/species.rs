use crate::update_sand;
use crate::update_water;
use crate::Cell;
use crate::SandApi;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Species {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
    Mud = 4,
}

impl Species {
    pub fn update(&self, cell: Cell, api: SandApi) {
        match self {
            Species::Empty => {}
            Species::Wall => {}
            Species::Sand => update_sand::update_sand(cell, api),
            Species::Water => update_water::update_water(cell, api),
            Species::Mud => {}
        }
    }
}

// fun dig(cell: Cell, mut api: SandApi) {
//   if ()
// }
