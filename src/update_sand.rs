use crate::Cell;
use crate::SandApi;
use crate::Species;
use crate::EMPTY_CELL;

pub fn update_sand(cell: Cell, mut api: SandApi) {
    let downX = api.rand_dir_2();
    let nbr = api.get(0, 1);
    if nbr.species == Species::Empty || nbr.species == Species::Water {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(downX, 1).species == Species::Water {
        api.set(0, 0, EMPTY_CELL);
        api.set(downX, 1, cell);
    } else if api.get(downX, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(downX, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}
