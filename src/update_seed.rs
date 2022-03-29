use crate::Cell;
use crate::CellApi;
use crate::Species;
use crate::EMPTY_CELL;
use crate::PLANT_CELL;

pub fn update_seed(cell: Cell, mut api: CellApi) {
    let down_x = api.rand_dir_2();
    let below = api.get(0, 1);
    if below.species == Species::Sand || below.species == Species::Plant {
        api.set(0, -3, PLANT_CELL);
        api.set(-1, -2, PLANT_CELL);
        api.set(-2, -2, PLANT_CELL);
        api.set(-3, -2, PLANT_CELL);
        api.set(3, -2, PLANT_CELL);
        api.set(2, -2, PLANT_CELL);
        api.set(1, -1, PLANT_CELL);
        api.set(-1, -2, PLANT_CELL);
        api.set(-1, -1, PLANT_CELL);
        api.set(-3, -1, PLANT_CELL);
        api.set(-3, -2, PLANT_CELL);
        api.set(0, -1, PLANT_CELL);
        api.set(0, 0, PLANT_CELL);
        api.set(0, 1, PLANT_CELL);
    } else if below.species == Species::Empty || below.species == Species::Water {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(down_x, 1).species == Species::Water {
        api.set(0, 0, EMPTY_CELL);
        api.set(down_x, 1, cell);
    } else if api.get(down_x, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(down_x, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}
