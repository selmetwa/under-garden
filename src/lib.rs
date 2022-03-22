extern crate js_sys;
extern crate web_sys;

mod species;
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

// impl Cell {
//     fn toggle(&mut self) {
//         *self = match *self {
//             Cell::Dead => Cell::Alive,
//             Cell::Alive => Cell::Dead,
//         }
//     }
// }

impl Cell {
    pub unsafe fn new(species: Species) -> Cell {
        Cell {
            species: species,
            ra: 1,
            // ra: 100 + (js_sys::Math::random() * 50.) as u8,
            rb: 0,
            clock: 0,
        }
    }
    // pub fn update(&self, api: SandApi) {
    //     self.species.update(*self, api);
    // }
}

static EMPTY_CELL: Cell = Cell {
    species: Species::Empty,
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
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = &self.cells[idx];
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 150;
        let height = 150;
        let cells = (0..width * height).map(|_i| EMPTY_CELL).collect();
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
        // let radius: f64 = (size as f64);

        let floor = (radius + 1.0) as i32;
        let ciel = (radius + 1.5) as i32;

        unsafe {
            log!("x {}: ", x);
            log!("y {}: ", y);
            log!("size {}: ", size);
            log!("floor {}: ", floor);
            log!("ciel {}: ", ciel);
            log!("species {:?}: ", species);
            log!("self.cells {:?}: ", self.cells);
        }

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

                // let test = self.get_cell(px, py).species;
                // unsafe {
                //     log!("test {:?}", test);
                //     log!("type {:?}", Species::Empty);
                // }
                if self.get_cell(px, py).species == Species::Empty || species == Species::Empty {
                    self.cells[i] = Cell {
                        species: species,
                        ra: 60
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
