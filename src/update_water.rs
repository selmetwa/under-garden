use crate::Cell;
use crate::CellApi;
use crate::Species;
use crate::EMPTY_CELL;
use crate::PLANT_CELL;

pub fn update_water(cell: Cell, mut api: CellApi) {
    let mut dx = api.rand_dir();
    let below = api.get(0, 1);
    let dx1 = api.get(dx, 1);

    if below.species == Species::Empty {
        api.set(0, 0, below);
        api.set(0, 1, cell);

        return;
    } else if dx1.species == Species::Empty {
        api.set(0, 0, dx1);
        api.set(dx, 1, cell);
        return;
    } else if api.get(-dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 1, cell);
        return;
    }

    if below.species == Species::Plant {
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
        return;
    }
    let down_x = api.rand_dir_2();
    if api.get(down_x, 1).species == Species::Plant {
        let left = cell.ra % 2 == 0;
        let stem = if left { PLANT_CELL } else { EMPTY_CELL };
        api.set(down_x, -2, PLANT_CELL);
        api.set(down_x, -1, PLANT_CELL);
        api.set(2, 3, PLANT_CELL);
        api.set(3, 1, PLANT_CELL);

        api.set(-2, -1, PLANT_CELL);
        api.set(-3, -2, PLANT_CELL);
        api.set(0, 0, PLANT_CELL);
        api.set(down_x, 1, PLANT_CELL);
    }

    let left = cell.ra % 2 == 0;
    dx = if left { 1 } else { -1 };
    let dx0 = api.get(dx, 0);
    let dxd = api.get(dx * 2, 0);

    if dx0.species == Species::Empty && dxd.species == Species::Empty {
        api.set(0, 0, dxd);
        api.set(2 * dx, 0, Cell { rb: 6, ..cell });
    } else if dx0.species == Species::Empty {
        api.set(0, 0, dx0);
        api.set(dx, 0, Cell { rb: 3, ..cell });
        let (dx, dy) = api.rand_vec_8();
        let nbr = api.get(dx, dy);
        if nbr.species == Species::Water {
            if nbr.ra % 2 != cell.ra % 2 {
                api.set(
                    dx,
                    dy,
                    Cell {
                        ra: cell.ra,
                        ..cell
                    },
                )
            }
        }
    } else if cell.rb == 0 {
        if api.get(-dx, 0).species == Species::Empty {
            // bump
            api.set(
                0,
                0,
                Cell {
                    ra: ((cell.ra as i32) + dx) as u8,
                    ..cell
                },
            );
        }
    } else {
        api.set(
            0,
            0,
            Cell {
                rb: cell.rb - 1,
                ..cell
            },
        );
    }
}
