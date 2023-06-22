use rand::seq::SliceRandom;
use std::collections::VecDeque;
use std::process;
use wasm_bindgen::prelude::*;

enum Movement {
    Left,
    Right,
    Up,
    Down,
}

struct Sequence {
    sequence: [usize; 16],
}

impl Sequence {
    fn get(movement: Movement) -> Sequence {
        match movement {
            Movement::Left => {
                let sequence: [usize; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
                Sequence { sequence }
            }

            Movement::Right => {
                let sequence: [usize; 16] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
                Sequence { sequence }
            }

            Movement::Up => {
                let sequence: [usize; 16] = [0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15];
                Sequence { sequence }
            }

            Movement::Down => {
                let sequence: [usize; 16] = [12, 8, 4, 0, 13, 9, 5, 1, 14, 10, 6, 2, 15, 11, 7, 3];
                Sequence { sequence }
            }
        }
    }

    fn get_cell_index(&self, index: usize) -> usize {
        self.sequence[index]
    }
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

    pub fn init_game(&mut self) {
        self.spawn();
        self.spawn();
    }

    pub fn cells(&self) -> *const u16 {
        self.cell_numbers.as_ptr()
    }

    pub fn slide_left(&mut self) {
        self.make_move(Movement::Left);
        self.spawn();
    }

    pub fn slide_right(&mut self) {
        self.make_move(Movement::Right);
        self.spawn();
    }

    pub fn slide_up(&mut self) {
        self.make_move(Movement::Up);
        self.spawn();
    }

    pub fn slide_down(&mut self) {
        self.make_move(Movement::Down);
        self.spawn();
    }
}

impl GameBoard {
    fn spawn(&mut self) {
        let mut empty_positions: Vec<usize> = Vec::with_capacity(16);
        for (index, element) in self.cell_numbers.iter().enumerate() {
            if *element == 0 {
                empty_positions.push(index);
            }
        }

        //get random position
        let index = match empty_positions.choose(&mut rand::thread_rng()) {
            Some(value) => *value,
            None => {
                println!("Game Over!");
                process::exit(0);
            }
        };

        self.cell_numbers[index] = if rand::random::<bool>() { 2 } else { 4 };
    }

    fn make_move(&mut self, movement: Movement) {
        let seq = Sequence::get(movement);

        for i in 0..4 {
            let mut tmp = 0;
            let mut decque: VecDeque<u16> = VecDeque::with_capacity(4);

            for j in 0..4 {
                let cell_index = seq.get_cell_index(4 * i + j);
                if self.cell_numbers[cell_index] == tmp && tmp != 0 {
                    decque.push_back(tmp * 2);
                    tmp = 0;
                } else {
                    if tmp != 0 && self.cell_numbers[cell_index] != 0 {
                        decque.push_back(tmp);
                        tmp = self.cell_numbers[cell_index];
                    } else if tmp == 0 && self.cell_numbers[cell_index] != 0 {
                        tmp = self.cell_numbers[cell_index];
                    }
                }
            }

            if tmp != 0 {
                decque.push_back(tmp);
            }

            for j in 0..4 {
                let cell_index = seq.get_cell_index(4 * i + j);
                self.cell_numbers[cell_index] = decque.pop_front().unwrap_or(0);
            }
        }
    }
}
