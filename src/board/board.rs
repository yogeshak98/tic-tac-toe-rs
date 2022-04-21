use std::collections::HashSet;

use wasm_bindgen::prelude::*;
use crate::difficulty::difficulty::{Difficulty, Easy, Moderate, Hard, DifficultyLevel, IDifficulty, WINNING_COMBINATIONS};

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell{
    Empty = 0,
    Player1 = 1,
    Player2 = 2
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Draw,
    Human,
    Computer,
    InProgess
}

#[wasm_bindgen]
pub struct Board{
    size: u8,
    difficulty: Difficulty,
    cells: Vec<Cell>,
    status: Status
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Self {
        Board{
            size: 3,
            difficulty: Difficulty::MODERATE(Moderate),
            cells: vec![Cell::Empty; 3 * 3],
            status: Status::InProgess
        }
    }

    pub fn reset(&mut self) {
        self.cells = vec![Cell::Empty; 3 * 3];
        self.status = Status::InProgess;
    }

    pub fn set_difficulty(&mut self, difficulty: DifficultyLevel){
        self.difficulty = match difficulty {
            DifficultyLevel::EASY => Difficulty::EASY(Easy),
            DifficultyLevel::HARD => Difficulty::HARD(Hard),
            DifficultyLevel::MODERATE => Difficulty::MODERATE(Moderate)
        };
    }

    pub fn get_difficulty(&self) -> DifficultyLevel{
        match self.difficulty {
            Difficulty::MODERATE(_) => DifficultyLevel::MODERATE,
            Difficulty::HARD(_) => DifficultyLevel::HARD,
            _ => DifficultyLevel::EASY,
        }
    }

    pub fn get_size(&self) -> u8 {
        self.size
    }

    pub fn make_next_valid_move(&mut self, player: Cell) {

        // Check if other player is victor
        let (a, b, c) = self.get_cells_info(&player);
        let victory_status = if !Cell::Player1.eq(&player) { Status::Human } else { Status::Computer };
        let player1_status = self.get_status(&a, victory_status);

        if player1_status == Status::Human || player1_status == Status::Computer {
            self.status = player1_status;
            return;
        }

        // check if no empty cells => draw
        if c.len() == 0 {
            self.status = Status::Draw;
            return;
        }

        // get next move on difficulty level
        let next_move = self.difficulty.get_next_move(&a, &b, &c, self.size);
        self.cells[next_move] = player;
        let (a, b, c) = self.get_cells_info(&player);

        let status = match player {
            Cell::Player1 => self.get_status(&a, Status::Human),
            Cell::Player2 => self.get_status(&b, Status::Computer),
            _ => Status::InProgess
        };

        // check if victor after recent move
        if status == Status::Computer || status == Status::Human {
            self.status = status;
            return;
        }
        // check for draw
        if c.len() == 0 {
            self.status = Status::Draw;
            return;
        }
    }

    pub fn get_cells_ptr(&self) -> *const Cell{
        self.cells.as_ptr()
    }

    pub fn set_cell(&mut self, idx: usize, player: Cell){
        self.cells[idx] = player;
    }

    pub fn is_empty_cell(&self, idx: usize) -> bool {
        self.cells[idx].eq(&Cell::Empty)
    }

    pub fn get_game_status(&self) -> Status{
        self.status
    }
}



impl Board {

    fn get_cells_info(&self, player: &Cell) -> (HashSet<usize>, HashSet<usize>, HashSet<usize>) {
        let mut player1 = HashSet::new();
        let mut player2 = HashSet::new();
        let mut empty = HashSet::new();
        self.cells.iter().enumerate()
        .for_each(|(x, y)| 
            if y.eq(&Cell::Empty){
                empty.insert(x);
            }
            else if y.eq(player) {
                player2.insert(x);
            }
            else {
                player1.insert(x);
            }
        );
        
        (player1, player2, empty)
    }

    fn get_status(&self, player_pos_set: &HashSet<usize>, status: Status) -> Status {
        for comb in WINNING_COMBINATIONS {
            let mut matching_count: u8 = 0;
            for i in comb{
                if player_pos_set.contains(&i) {
                    matching_count += 1;
                }
            }
            if matching_count == 3 {
                return status;
            }
        }
        Status::InProgess
    }
}
