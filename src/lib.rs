mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

//enum to define the properties of Cell
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0, 
    Alive = 1,
}

//struct to create a universe
#[wasm_bindgen]
pub struct Universe {
    w: u32,
    h: u32,
    cells: Vec<Cell>,
}

//struct methods
#[wasm_bindgen]
impl Universe {
    //game flow
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.h {
            for col in 0..self.w {
                let id = self.get_index(row,col);
                let cell = self.cells[id];
                let live_neighbors = self.live_neighbor_count(row)

                //will change the cells to dead or alive
                let next_cell = match (cell, live_neighbors) {
                    // any live cell with < 2 live neighbours will die
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    //any cell with 2 or 3 neighbours live
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    //any live cell with more than three live neighbours will die
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    //any dead cell with 3 live neighbours will become a living cell
                    (Cell::Dead, 3) => Cell::Alive,
                    //all other cells stay the same
                    (otherwise, _) => otherwise,
                };
                next[id] = next_cell;
            }
        }
        self.cells = next 
    }
    
    //function to access the cells at a given row or column
    fn get_pos(&self, row: u32, column: u32) -> usize {
        (row * self.w + column) as usize
    }

    //counts the number of neighbours that are alive
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut c = 0;
        for rows in [self.h -1, 0, 1].iter().cloned() {
            for col in [self.w -1, 0, 1].iter().cloned(){
                if rows == 0 && col == 0 {
                    continue;
                }

                let n_row = (row + rows) % self.h;
                let n_col = (column + col) % self.w;
                let i = self.get_pos(n_row, n_col);
                c += self.cells[i] as u8;
            }
        }
        c
    }
 }