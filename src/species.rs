use crate::update_dynamite;
use crate::update_sand;
use crate::update_smoke;
use crate::update_steam;
use crate::update_water;
use crate::update_worm;
use crate::Cell;
use crate::CellApi;
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
    Dynamite = 5,
    Steam = 6,
    Smoke = 7,
    Worm = 8,
}

impl Species {
    pub fn update(&self, cell: Cell, api: CellApi) {
        match self {
            Species::Empty => {}
            Species::Wall => {}
            Species::Sand => update_sand::update_sand(cell, api),
            Species::Water => update_water::update_water(cell, api),
            Species::Mud => {}
            Species::Dynamite => update_dynamite::update_dynamite(cell, api),
            Species::Steam => update_steam::update_steam(cell, api),
            Species::Smoke => update_smoke::update_smoke(cell, api),
            Species::Worm => update_worm::update_worm(cell, api),
        }
    }
}
