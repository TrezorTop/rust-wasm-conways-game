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

/// The `Universe` struct represents the state of the Game of Life simulation.
/// It contains the width and height of the grid, as well as the current and next
/// generations of cells.
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

#[wasm_bindgen]
impl Universe {
    /// Creates a new `Universe` instance with a default width and height of 128.
    ///
    /// This function sets up the initial state of the `Universe` by:
    /// - Creating a `FixedBitSet` to represent the current cells
    /// - Creating a `FixedBitSet` to represent the next generation of cells
    /// - Initializing all cells to the dead state
    ///
    /// The `Universe` struct contains the width, height, current cells, and next generation cells.
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

    /// Resets the state of the `Universe` by randomly setting each cell to either alive or dead.
    ///
    /// This function iterates over all the cells in the `Universe` and randomly sets each cell to
    /// either the alive or dead state based on a 50% probability.
    pub fn reset(&mut self) {
        let size = (self.width * self.height) as usize;

        for i in 0..size {
            self.cells.set(i, js_sys::Math::random() < 0.5);
        }
    }

    /// Clears all cells in the `Universe` to the dead state.
    ///
    /// This function iterates over all the cells in the `Universe` and sets each cell to the dead state.
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    /// Returns the width of the `Universe`.
    ///
    /// This function returns the current width of the `Universe` instance.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Sets the width of the `Universe`.
    ///
    /// If the provided `width` is the same as the current width, this function returns without making any changes.
    /// Otherwise, it updates the `width` field and calls the `update_cells_size()` function to resize the `cells` and `next_cells` fields accordingly.
    pub fn set_width(&mut self, width: u32) {
        if self.width == width {
            return;
        }

        self.width = width;

        self.update_cells_size()
    }

    /// Returns the height of the `Universe`.
    ///
    /// This function returns the current height of the `Universe` instance.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Sets the height of the `Universe`.
    ///
    /// If the provided `height` is the same as the current height, this function returns without making any changes.
    /// Otherwise, it updates the `height` field and calls the `update_cells_size()` function to resize the `cells` and `next_cells` fields accordingly.
    pub fn set_height(&mut self, height: u32) {
        if self.height == height {
            return;
        }

        self.height = height;

        self.update_cells_size()
    }
    
    /// Updates the size of the `cells` and `next_cells` fields to match the current `width` and `height` of the `Universe`.
    ///
    /// If the current size of `cells` is larger than the required size, the `cells` field is set to a new `FixedBitSet` with the required capacity.
    /// Otherwise, the `cells` field is grown to the required size.
    fn update_cells_size(&mut self) {
        let size = (self.width * self.height) as usize;

        if self.cells.len() > size {
            self.cells = FixedBitSet::with_capacity(size);
        } else {
            // self.cells.clear();
            self.cells.grow(size);
        }
    }

    /// Returns a raw pointer to the underlying `cells` bit set.
    ///
    /// This function provides low-level access to the `cells` bit set, 
    /// which represents the current state of the cells in the `Universe`. 
    /// The returned pointer can be used to efficiently access or manipulate the cell states, 
    /// but care must be taken to ensure that the memory layout and size of the `cells` bit set are not changed in a way that would invalidate the pointer.
    ///
    /// # Safety
    /// The caller must ensure that the returned pointer remains valid 
    /// and that any modifications to the `cells` bit set are performed in a way that preserves the invariants of the `Universe` struct.
    pub fn cells(&self) -> *const usize {
        self.cells.as_slice().as_ptr()
    }

    /// Toggles the state of the cell at the given row and column.
    ///
    /// This function updates the state of the cell at the specified row and column in the `cells` bit set.
    /// If the cell is currently alive, it is set to dead. If the cell is currently dead, it is set to alive.
    ///
    /// # Arguments
    /// * `row` - The row index of the cell to toggle.
    /// * `col` - The column index of the cell to toggle.
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let index = self.get_index(row, col);

        self.cells.toggle(index);
    }

    /// Advances the state of the `Universe` by one time step.
    ///
    /// This function updates the state of the `Universe` by applying the rules of the Game of Life to each cell in the `cells` bit set. 
    /// The new state is stored in the `next_cells` bit set, 
    /// and then the `cells` and `next_cells` bit sets are swapped to make the new state the current state.
    ///
    /// The function first iterates over all the cells in the `cells` bit set, 
    /// calculating the number of live neighbors for each cell. 
    /// It then updates the state of each cell in the `next_cells` bit set based on the number of live neighbors, 
    /// following the rules of the Game of Life:
    ///
    /// - Any live cell with fewer than two live neighbors dies, as if caused by underpopulation.
    /// - Any live cell with two or three live neighbors lives on to the next generation.
    /// - Any live cell with more than three live neighbors dies, as if by overpopulation.
    /// - Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
    ///
    /// After updating the `next_cells` bit set, the function swaps the `cells` and `next_cells` bit sets to make the new state the current state.
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

    /// Calculates the index of a cell in the `cells` bit set given its row and column coordinates.
    ///
    /// This function takes the row and column coordinates of a cell and calculates the corresponding index in the `cells` bit set.
    /// The index is calculated by multiplying the row by the width of the `Universe` and adding the column.
    /// This ensures that the cells are stored in row-major order in the bit set.
    ///
    /// # Arguments
    /// * `row` - The row coordinate of the cell.
    /// * `column` - The column coordinate of the cell.
    ///
    /// # Returns
    /// The index of the cell in the `cells` bit set.
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    /// Calculates the number of live neighbors for a given cell in the Game of Life.
    ///
    /// This function takes the row and column coordinates of a cell and calculates the number of live neighbors
    /// surrounding that cell. It does this by checking the eight adjacent cells and counting how many of them
    /// are alive. The function handles the edge cases where the cell is on the edge of the grid by wrapping
    /// around to the opposite side of the grid.
    ///
    /// # Arguments
    /// * `row` - The row coordinate of the cell.
    /// * `column` - The column coordinate of the cell.
    ///
    /// # Returns
    /// The number of live neighbors for the given cell.
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

// These methods are not opened for JavaScript
impl Universe {
    /// Returns a reference to the internal `FixedBitSet` that represents the cells in the Universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Sets the alive cells in the Universe to the given list of cell coordinates.
    ///
    /// This function takes a slice of `(u32, u32)` tuples, where each tuple represents the row and column
    /// of a cell in the Universe. It then iterates over the slice and sets the corresponding cells in
    /// the `FixedBitSet` to be alive.
    ///
    /// # Examples
    /// 
    /// use conway_game_of_life::Universe;
    ///
    /// let mut universe = Universe::new(10, 10);
    /// universe.set_alive_cells(&[(0, 0), (1, 1), (2, 2)]);
    /// 
    pub fn set_alive_cells(&mut self, cells: &[(u32, u32)]) {
        cells.iter().for_each(|(row, col)| {
            let index = self.get_index(*row, *col);

            self.cells.set(index, true);
        })
    }
}
