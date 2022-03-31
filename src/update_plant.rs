use crate::Cell;
use crate::CellApi;
use crate::Species;
use crate::EMPTY_CELL;
use crate::PLANT_CELL;

pub fn update_plant(cell: Cell, mut api: CellApi) {
    let down_x = api.rand_dir_2();
    let below = api.get(0, 1);
    if below.species == Species::Empty || below.species == Species::Water {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
        return;
    }
}
