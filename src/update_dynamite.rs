use crate::Cell;
use crate::CellApi;
use crate::Species;
use crate::EMPTY_CELL;
use crate::SMOKE_CELL;
use crate::STEAM_CELL;
use rand::Rng;

pub fn update_dynamite(cell: Cell, mut api: CellApi) {
    let down_x = api.rand_dir_2();
    let below = api.get(0, 1);

    if below.species == Species::Stone {
        let num = rand::thread_rng().gen_range(0..500);
        if num >= 475 {
            api.set(0, -1, SMOKE_CELL);
            api.set(0, 0, EMPTY_CELL);
            api.set(1, 1, EMPTY_CELL);
            return;
        }
    }

    if api.get(down_x, 1).species == Species::Stone {
        let num = rand::thread_rng().gen_range(0..500);
        if num >= 475 {
            api.set(0, -1, SMOKE_CELL);
            api.set(0, 0, EMPTY_CELL);
            api.set(down_x, 1, EMPTY_CELL);
            api.set(down_x + 1, 2, EMPTY_CELL);
        }
    }
    if below.species == Species::Empty || below.species == Species::Smoke {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(down_x, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(down_x, 1, cell);
    } else {
        api.set(0, 0, cell);
    }

    if below.species == Species::Water {
        api.set(0, -1, STEAM_CELL);
        api.set(0, 0, STEAM_CELL);
        api.set(0, 1, EMPTY_CELL);
    } else if api.get(down_x, 1).species == Species::Water {
        api.set(-1, -1, STEAM_CELL);
        api.set(0, 0, STEAM_CELL);
    }

    if below.species == Species::Wall
        || below.species == Species::Seed
        || below.species == Species::Plant
        || below.species == Species::Sand
    {
        api.set(0, -1, SMOKE_CELL);
        api.set(0, 0, EMPTY_CELL);
        api.set(1, 1, EMPTY_CELL);
        api.set(2, 2, EMPTY_CELL);
    } else if api.get(down_x, 1).species == Species::Wall
        || api.get(down_x, 1).species == Species::Sand
        || api.get(down_x, 1).species == Species::Plant
        || api.get(down_x, 1).species == Species::Seed
    {
        api.set(0, -1, SMOKE_CELL);
        api.set(0, 0, EMPTY_CELL);
        api.set(down_x, 1, EMPTY_CELL);
        api.set(down_x + 1, 2, EMPTY_CELL);
    }
}
