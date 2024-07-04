use std::mem;

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

use crate::utils::Timer;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
    // buffering the next generation for cells
    next_cells: FixedBitSet,
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 128;
        let height = 128;

        let size = (width * height) as usize;

        let mut cells = FixedBitSet::with_capacity(size);
        let next_cells = cells.clone();

        for i in 0..size {
            cells.set(i, false)
        }

        Universe {
            width,
            height,
            cells,
            next_cells,
        }
    }

    pub fn reset(&mut self) {
        let size = (self.width * self.height) as usize;

        for i in 0..size {
            self.cells.set(i, js_sys::Math::random() < 0.5);
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        if self.width == width {
            return;
        }

        self.width = width;

        self.update_cells_size()
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        if self.height == height {
            return;
        }

        self.height = height;

        self.update_cells_size()
    }

    // Updates size of the universe.
    // Resets all cells to the dead state.
    fn update_cells_size(&mut self) {
        let size = (self.width * self.height) as usize;

        if self.cells.len() > size {
            self.cells = FixedBitSet::with_capacity(size);
        } else {
            // self.cells.clear();
            self.cells.grow(size);
        }
    }

    pub fn cells(&self) -> *const usize {
        self.cells.as_slice().as_ptr()
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let index = self.get_index(row, col);

        self.cells.toggle(index);
    }

    pub fn tick(&mut self) {
        Timer::new("Universe::tick");

        let size = (self.width * self.height) as usize;
        for i in 0..size {
            let row = (i as u32) / self.width;
            let col = (i as u32) % self.width;
            let cell = self.cells[i];
            let live_neighbors = self.live_neighbor_count(row, col);

            self.next_cells.set(
                i,
                match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise
                }
            );
        }

        // Swap current cells with next cells
        mem::swap(&mut self.cells, &mut self.next_cells);
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }
}

impl Universe {
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_alive_cells(&mut self, cells: &[(u32, u32)]) {
        cells.iter().for_each(|(row, col)| {
            let index = self.get_index(*row, *col);

            self.cells.set(index, true);
        })
    }
}
