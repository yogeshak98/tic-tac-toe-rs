use std::collections::HashSet;
use wasm_bindgen::prelude::*;

extern crate js_sys;

use js_sys::Math::{floor, random};

pub const WINNING_COMBINATIONS: [[usize; 3]; 8] = [
    [0,1,2], 
    [3,4,5],
    [6,7,8],
    [0,3,6],
    [1,4,7],
    [2,5,8],
    [0,4,8],
    [2,4,6]
];

fn get_random_int(max: f64) -> usize{
    return floor(random() * max) as usize;
}

pub trait IDifficulty{
    fn get_next_move(&self, player1_pos: &HashSet<usize>, player2_pos: &HashSet<usize>, empty: &HashSet<usize>, size: u8) -> usize;
}

pub struct Easy;

impl IDifficulty for Easy{
    fn get_next_move(&self, _player1_pos: &HashSet<usize>, _player2_pos: &HashSet<usize>, empty: &HashSet<usize>, _size: u8) -> usize {
        let empty_vec: Vec<&usize> = empty.iter().collect();
        *empty_vec[get_random_int(empty_vec.len() as f64)]
    }
}

pub struct Moderate;

impl Moderate {
    fn get_winning_combination_for_player(&self, positions: &HashSet<usize>, empty_set: &HashSet<usize>, comb: &[usize]) -> Option<usize>{
        let mut partial_comb = vec![];
        let mut missing_num = vec![];
        for i in comb{
            if positions.contains(&i) {
                partial_comb.push(*i);
            }
            else if empty_set.contains(&i){
                missing_num.push(*i);
            }
        }
        if partial_comb.len() == 2 && missing_num.len() == 1 {
            Some(missing_num[0])
        }
        else{
            None
        }
    }
}

impl IDifficulty for Moderate{

    fn get_next_move(&self, player1_pos: &HashSet<usize>, player2_pos: &HashSet<usize>, empty: &HashSet<usize>, _size: u8) -> usize {
        let mut player1_set = HashSet::<usize>::new();
        let mut player2_set = HashSet::<usize>::new();
        let mut empty_set = HashSet::<usize>::new();
        player1_set.extend(player1_pos.iter());
        player2_set.extend(player2_pos.iter());
        empty_set.extend(empty.iter());

        for comb in WINNING_COMBINATIONS {
            let player2_res = self.get_winning_combination_for_player(&player2_set, &empty_set, &comb);
            match player2_res {
                Some(x) => {
                    return x;
                },
                None => {}
            }
        }

        for comb in WINNING_COMBINATIONS {
            let player1_res = self.get_winning_combination_for_player(&player1_set, &empty_set, &comb);
            match player1_res {
                Some(x) => {
                    return x;
                },
                None => {}
            }
        }
        let empty_vec: Vec<&usize> = empty.iter().collect();
        *empty_vec[get_random_int(empty_vec.len() as f64)]
    }
}

pub struct Hard;

impl Hard {
    fn score(&self, opposition: &HashSet<usize>, player: &HashSet<usize>) -> i32{
        for comb in WINNING_COMBINATIONS {
            if player.intersection(&HashSet::from(comb)).map(|_| 1).sum::<i32>() == 3 {
                return 10;
            }
        }
        for comb in WINNING_COMBINATIONS {
            if opposition.intersection(&HashSet::from(comb)).map(|_| 1).sum::<i32>() == 3 {
                return -10;
            }
        }
        0
    }

    fn min_max(&self, depth: i32, is_max: bool, player1_pos: &mut HashSet<usize>, player2_pos: &mut HashSet<usize>, empty: &mut HashSet<usize>) -> i32{
        let score = self.score(player1_pos, player2_pos);
        if depth == 0 && score == 10 || score == -10 {
            return score;
        }
        if empty.len() == 0 {
            return 0;
        }
        let empty_cells: Vec<usize> = empty.iter().map(|x| *x).collect();
        if is_max {
            let mut best_score = -1000;
            for cell in empty_cells {
                player2_pos.insert(cell);
                empty.remove(&cell);
                let score = self.min_max(depth + 1, !is_max, player1_pos, player2_pos, empty);
                best_score = best_score.max(score);
                empty.insert(cell);
                player2_pos.remove(&cell);
            }
            best_score
        }
        else{
            let mut best_score = 1000;
            for cell in empty_cells {
                player1_pos.insert(cell);
                empty.remove(&cell);
                let score = self.min_max(depth + 1, !is_max, player1_pos, player2_pos, empty);
                best_score = best_score.min(score);
                empty.insert(cell);
                player1_pos.remove(&cell);
            }
            best_score
        }
    }

    fn find_best_move(&self, player1_pos: &mut HashSet<usize>, player2_pos: &mut HashSet<usize>, empty: &mut HashSet<usize>) -> i32 {
        let mut best_score = -1000;
        let mut best_move = -1;
        let empty_cells = empty.iter().map(|x| *x).collect::<Vec<usize>>();
        
        for cell in empty_cells {
            empty.remove(&cell);
            player2_pos.insert(cell);

            let score = self.min_max(0, false, player1_pos, player2_pos, empty);

            empty.insert(cell);
            player2_pos.remove(&cell);
            if score > best_score {
                best_score = score;
                best_move = cell as i32;
            }
        }
        best_move
    }
}

impl IDifficulty for Hard{
    fn get_next_move(&self, player1_pos: &HashSet<usize>, player2_pos: &HashSet<usize>, empty: &HashSet<usize>, _size: u8) -> usize {
        if empty.len() == 9 {
            let empty_vec: Vec<&usize> = empty.iter().collect();
            return *empty_vec[get_random_int(empty_vec.len() as f64)];
        }
        let mut empty_mut_clone = empty.clone();
        let mut player1_mut_clone = player1_pos.clone();
        let mut player2_mut_clone = player2_pos.clone();

        let next_move = self.find_best_move(&mut player1_mut_clone, &mut player2_mut_clone, &mut empty_mut_clone);
        if next_move >= 0 {
            next_move as usize
        }
        else{
            let empty_vec: Vec<&usize> = empty.iter().collect();
            *empty_vec[get_random_int(empty_vec.len() as f64)]
        }
    }
}

#[wasm_bindgen]
pub enum DifficultyLevel {
    EASY,
    MODERATE,
    HARD
}

pub enum Difficulty{
    EASY(Easy),
    MODERATE(Moderate),
    HARD(Hard)
}


impl IDifficulty for Difficulty {
    fn get_next_move(&self, player1_pos: &HashSet<usize>, player2_pos: &HashSet<usize>, empty: &HashSet<usize>, size: u8) -> usize {
        match self {
            Difficulty::EASY(lvl) => lvl.get_next_move(player1_pos, player2_pos, empty, size),
            Difficulty::MODERATE(lvl) => lvl.get_next_move(player1_pos, player2_pos, empty, size),
            Difficulty::HARD(lvl) => lvl.get_next_move(player1_pos, player2_pos, empty, size)
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_moderate(){
        let difficulty = Difficulty::MODERATE(Moderate);
        let mut a: HashSet<usize> = HashSet::new();
        a.extend(vec![0, 3, 4].iter());

        let mut b: HashSet<usize> = HashSet::new();
        b.extend(vec![1, 5].iter());

        let mut empty: HashSet<usize> = HashSet::new();
        empty.extend(vec![2, 6, 7, 8].iter());
        let next_move = difficulty.get_next_move(&a, &b, &empty, 9);
        assert_eq!(next_move, 6);
    }

    #[test]
    fn test_hard_1(){
        let difficulty = Difficulty::HARD(Hard);
        let mut a: HashSet<usize> = HashSet::new();
        a.extend(vec![1, 3, 4].iter());

        let mut b: HashSet<usize> = HashSet::new();
        b.extend(vec![0, 2, 5].iter());
        /* 
        input 
        player 1 - O
        player 2 - X - ai
        | X | O | X |
        | O | O | X |
        |   |   |   |
        */
        let mut empty: HashSet<usize> = HashSet::new();
        empty.extend(vec![6, 7, 8].iter());
        let next_move = difficulty.get_next_move(&a, &b, &empty, 9);
        assert_eq!(next_move, 8);
    }

    #[test]
    fn test_hard_2(){
        let difficulty = Difficulty::HARD(Hard);
        let mut a: HashSet<usize> = HashSet::new();
        a.extend(vec![3, 4].iter());

        let mut b: HashSet<usize> = HashSet::new();
        b.extend(vec![2].iter());
        /* 
        input 
        player 1 - O
        player 2 - X - ai
        |   |   | X |
        | O | O |   |
        |   |   |   |
        */
        let mut empty: HashSet<usize> = HashSet::new();
        empty.extend(vec![0, 1, 5, 6, 7, 8].iter());
        let next_move = difficulty.get_next_move(&a, &b, &empty, 9);
        assert_eq!(next_move, 5);
    }

    #[test]
    fn test_hard_3(){
        let difficulty = Difficulty::HARD(Hard);
        let mut a: HashSet<usize> = HashSet::new();
        a.extend(vec![3, 4].iter());

        let mut b: HashSet<usize> = HashSet::new();
        b.extend(vec![2, 5].iter());
        /* 
        input 
        player 1 - O
        player 2 - X - ai
        |   |   | X |
        | O | O | X |
        |   |   |   |
        */
        let mut empty: HashSet<usize> = HashSet::new();
        empty.extend(vec![0, 1, 6, 7, 8].iter());
        let next_move = difficulty.get_next_move(&b, &a, &empty, 9);
        assert_eq!(next_move, 8);
    }
}