use crate::Cell;
use crate::CellApi;
use crate::Species;
use crate::EMPTY_CELL;

pub fn update_fire(cell: Cell, mut api: CellApi) {
    let above = api.get(0, -1);
    let below = api.get(0, 1);
    let down_x = api.rand_dir_2();

    if above.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, -1, cell);
    } else if above.species == Species::Wall {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, -1, EMPTY_CELL);
        api.set(0, -2, EMPTY_CELL);
        api.set(0, -3, EMPTY_CELL);
        api.set(0, -4, EMPTY_CELL);
    } else if api.get(down_x, -1).species == Species::Wall {
        api.set(0, 0, EMPTY_CELL);
        api.set(down_x, -1, EMPTY_CELL);
    }
}
