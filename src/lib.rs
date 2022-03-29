extern crate js_sys;
extern crate web_sys;

mod species;
mod update_dynamite;
mod update_sand;
mod update_seed;
mod update_smoke;
mod update_steam;
mod update_water;
mod update_worm;
mod utils;

use rand::{Rng, SeedableRng};
use rand_xoshiro::SplitMix64;
use species::Species;
use wasm_bindgen::prelude::*;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    species: Species,
    ra: u8,
    rb: u8,
    clock: u8,
}

#[wasm_bindgen]
pub struct Universe {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    rng: SplitMix64,
    generation: u8,
}

pub struct CellApi<'a> {
    x: i32,
    y: i32,
    universe: &'a mut Universe,
}

impl<'a> CellApi<'a> {
    pub fn get(&mut self, dx: i32, dy: i32) -> Cell {
        if dx > 5 || dx < -5 || dy > 5 || dy < -5 {
            panic!("oob set");
        }
        let nx = self.x + dx;
        let ny = self.y + dy;
        if nx < 0 || nx > self.universe.width - 1 || ny < 0 || ny > self.universe.height - 1 {
            return Cell {
                species: Species::Wall,
                ra: 0,
                rb: 0,
                clock: self.universe.generation,
            };
        }
        self.universe.get_cell(nx, ny)
    }
    pub fn set(&mut self, dx: i32, dy: i32, v: Cell) {
        if dx > 5 || dx < -5 || dy > 5 || dy < -5 {
            panic!("oob set");
        }
        let nx = self.x + dx;
        let ny = self.y + dy;

        if nx < 0 || nx > self.universe.width - 1 || ny < 0 || ny > self.universe.height - 1 {
            return;
        }
        let i = self.universe.get_index(nx, ny);
        self.universe.cells[i] = v;
        self.universe.cells[i].clock = self.universe.generation.wrapping_add(1);
    }

    pub fn rand_int(&mut self, n: i32) -> i32 {
        self.universe.rng.gen_range(0..n)
    }

    pub fn once_in(&mut self, n: i32) -> bool {
        self.rand_int(n) == 0
    }
    pub fn rand_dir(&mut self) -> i32 {
        let i = self.rand_int(1000);
        (i % 3) - 1
    }
    pub fn rand_dir_2(&mut self) -> i32 {
        let i = self.rand_int(1000);
        if (i % 2) == 0 {
            -1
        } else {
            1
        }
    }

    pub fn rand_vec(&mut self) -> (i32, i32) {
        let i = self.rand_int(2000);
        match i % 9 {
            0 => (1, 1),
            1 => (1, 0),
            2 => (1, -1),
            3 => (0, -1),
            4 => (-1, -1),
            5 => (-1, 0),
            6 => (-1, 1),
            7 => (0, 1),
            _ => (0, 0),
        }
    }

    pub fn rand_vec_8(&mut self) -> (i32, i32) {
        let i = self.rand_int(8);
        match i {
            0 => (1, 1),
            1 => (1, 0),
            2 => (1, -1),
            3 => (0, -1),
            4 => (-1, -1),
            5 => (-1, 0),
            6 => (-1, 1),
            _ => (0, 1),
        }
    }
}

impl Cell {
    pub unsafe fn new(species: Species) -> Cell {
        Cell {
            species: species,
            ra: 0,
            rb: 0,
            clock: 0,
        }
    }
    pub fn update(&self, api: CellApi) {
        self.species.update(*self, api);
    }
}

static EMPTY_CELL: Cell = Cell {
    species: Species::Empty,
    ra: 0,
    rb: 0,
    clock: 0,
};

static STEAM_CELL: Cell = Cell {
    species: Species::Steam,
    ra: 0,
    rb: 0,
    clock: 0,
};

static SMOKE_CELL: Cell = Cell {
    species: Species::Smoke,
    ra: 0,
    rb: 0,
    clock: 0,
};

static PLANT_CELL: Cell = Cell {
    species: Species::Plant,
    ra: 0,
    rb: 0,
    clock: 0,
};

static MUD_CELL: Cell = Cell {
    species: Species::Mud,
    ra: 0,
    rb: 0,
    clock: 0,
};

static SAND_CELL: Cell = Cell {
    species: Species::Sand,
    ra: 0,
    rb: 0,
    clock: 0,
};

static WALL_CELL: Cell = Cell {
    species: Species::Wall,
    ra: 0,
    rb: 0,
    clock: 0,
};

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: i32, column: i32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_cell(&self, x: i32, y: i32) -> Cell {
        let i = self.get_index(x, y);
        return self.cells[i];
    }

    pub fn tick(&mut self) {
        self.generation = self.generation.wrapping_add(1);
        for x in 0..self.width {
            let scanx = if self.generation % 2 == 0 {
                self.width - (1 + x)
            } else {
                x
            };

            for y in 0..self.height {
                let cell = self.get_cell(scanx, y);

                Universe::update_cell(
                    cell,
                    CellApi {
                        universe: self,
                        x: scanx,
                        y,
                    },
                );
            }
        }

        self.generation = self.generation.wrapping_add(1);
    }

    pub fn new(width: i32, height: i32) -> Universe {
        let cells = (0..width * height).map(|_i| WALL_CELL).collect();
        let rng: SplitMix64 = SeedableRng::seed_from_u64(0x734f6b89de5f83cc);
        Universe {
            width,
            height,
            cells,
            rng,
            generation: 0,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn paint(&mut self, x: i32, y: i32, size: i32, species: Species) {
        let size = size;
        let radius: f64 = (size as f64) / 2.0;

        let floor = (radius + 1.0) as i32;
        let ciel = (radius + 1.5) as i32;

        for dx in -floor..ciel {
            for dy in -floor..ciel {
                if (((dx * dx) + (dy * dy)) as f64) > (radius * radius) {
                    continue;
                };
                let px = x + dx;
                let py = y + dy;
                let i = self.get_index(px, py);

                if px < 0 || px > self.width - 1 || py < 0 || py > self.height - 1 {
                    continue;
                }

                if self.get_cell(px, py).species == Species::Empty || species == Species::Empty {
                    self.cells[i] = Cell {
                        species: species,
                        ra: 20
                            + (size as u8)
                            + (self.rng.gen::<f32>() * 30.) as u8
                            + ((self.generation % 127) as i8 - 60).abs() as u8,
                        rb: 0,
                        clock: self.generation,
                    };
                }
            }
        }
    }
}

//private methods
impl Universe {
    fn update_cell(cell: Cell, api: CellApi) {
        if cell.clock - api.universe.generation == 1 {
            return;
        }

        cell.update(api);
    }
}
