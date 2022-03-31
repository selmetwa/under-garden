use crate::Cell;
use crate::CellApi;
use crate::Species;
use crate::EMPTY_CELL;
use crate::PLANT_CELL;
use crate::STONE_CELL;
use crate::WALL_CELL;

pub fn update_smoke(cell: Cell, mut api: CellApi) {
    let above = api.get(0, -1);
    let below = api.get(0, 1);

    if above.species == Species::Wall {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, -1, WALL_CELL);
        return;
    }

    if above.species == Species::Plant {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, -1, PLANT_CELL);
        return;
    }

    if above.species == Species::Stone {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, -1, STONE_CELL);
        return;
    }

    if above.species == Species::Empty || below.species == Species::Steam {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, -1, cell);
        return;
    }
}
