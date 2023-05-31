use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello() -> String {
    String::from("Hello World from wasm!")
}

#[wasm_bindgen]
pub struct GameBoard {
    cell_numbers: [u16; 16],
}

#[wasm_bindgen]
impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard {
            cell_numbers: [0; 16],
        }
    }
    pub fn cells(&self) -> *const u16 {
        self.cell_numbers.as_ptr()
    }

    pub fn testing() -> *const u16 {
        [0; 16].as_ptr()
    }
}
