use std::borrow::BorrowMut;
use std::cmp::{max, min};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct TootOtto {
    board: [[Option<Piece>; 7]; 6],
    current_player: Player,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    T,
    O,
}

#[derive(PartialEq, Clone)]
pub enum Player {
    Toot,
    Otto,
    AI,
}

impl From<Piece> for Player {
    fn from(piece: Piece) -> Self {
        match piece {
            Piece::T => Player::Toot,
            Piece::O => Player::Otto,
        }
    }
}

impl TootOtto {
    pub fn new() -> TootOtto {
        TootOtto {
            board: [[None; 7]; 6],
            current_player: Player::Toot,
        }
    }

    pub fn get_current_player(&self) -> Player{
        self.current_player.clone()
    }

    pub fn get_grid(&self) -> [[Option<Piece>; 7]; 6] {
        self.board.clone()
    }

    pub fn top_row(&self, col: usize) -> usize {
        for row in (0..6).rev() {
            if self.board[row][col].is_none() {
                return row;
            }
        }
        // If the column is full, return a large number.
        10
    }

    // function to make a move by Player Toot
    pub fn make_move_by_toot(&mut self, column: usize, piece: Piece) {
        // if self.current_player != Player::Toot {
        //     return;
        // }
        // check if the given column is valid
        if column >= 7 {
            return;
        }
        self.place_piece(column, piece);
    }

    pub fn make_move_by_otto(&mut self, column: usize, piece: Piece) {
        // if self.current_player != Player::Otto {
        //     return;
        // }
        if column >= 7 {
            return;
        }
        self.place_piece(column, piece);
    }

    fn place_piece(&mut self, column:usize, piece: Piece){
        let mut row = None;
        for i in (0..6).rev() {
            if self.board[i][column].is_none() {
                row = Some(i);
                break;
            }
        }
        // if the column is full, return
        if row.is_none() {
            return;
        }
        // place the piece at the selected position
        self.board[row.unwrap()][column] = Some(piece);
    }

    fn check_bounds(&self, col: usize) -> bool {
        col < 7
    }


    pub fn make_move_by_ai(&mut self, depth: usize) {
        let mut maximizing_player = true;
        let mut next_Toot = false;
        // Let Toot be max and Otto be min
        match self.current_player {
            Player::Toot => {
                maximizing_player = false;
                next_Toot = true;
            },
            _ => {},
        };

        self.current_player = Player::AI;
        // let mut current_state = self.board.clone();
        // let mut best_score = i32::MIN;
        let (column, score, piece) = self.minimax(depth as i32, i32::MIN, i32::MAX, maximizing_player);
        if score != 0 && column != 10{
            self.place_piece(column, piece);
        }

        // switch to the next player
        if next_Toot == true{
            self.current_player = Player::Toot;
        } else {
            self.current_player = Player::Otto;
        }
    }

    pub fn minimax(&mut self, depth: i32, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> (usize, i32, Piece) {
        let (mut best_col, mut best_score, mut best_piece) = (0, i32::MIN, Piece::T);

        if depth == 0 || self.is_draw()  || self.is_over() {
            return (0, self.evaluate_board( maximizing_player), best_piece);
        }

        if maximizing_player {
        // favour TOOT
            let piece = Piece::T;
            for j in 0..7 {
                if self.board[0][j].is_some() {
                    continue;
                }
                let mut game_copy = self.clone();
                game_copy.place_piece(j, piece);
                let temp_val = game_copy.minimax(depth - 1, alpha, beta, !maximizing_player);
                if temp_val.1 > best_score {
                    best_score = temp_val.1;
                    best_col = j as usize;
                    best_piece = piece;
                }
                alpha = alpha.max(best_score);
                if alpha >= beta {
                    break;
                }
            }
            let piece = Piece::O;
            for j in 0..7{
                if self.board[0][j].is_some() {
                    continue;
                }
                let mut game_copy = self.clone();
                game_copy.place_piece(j, piece);
                let temp_val = game_copy.minimax(depth - 1, alpha, beta, !maximizing_player);
                if temp_val.1 > best_score {
                    best_score = temp_val.1;
                    best_col = j as usize;
                    best_piece = piece;
                }
                alpha = alpha.max(best_score);
                if alpha >= beta {
                    break;
                }
            }
            return(best_col, best_score, best_piece);
        } else {
        // favour OTTO
            let piece = Piece::T;
            for j in 0..7 {
                if self.board[0][j].is_some() {
                    continue;
                }
                let mut game_copy = self.clone();
                game_copy.place_piece(j, piece);
                let temp_val = game_copy.minimax(depth - 1, alpha, beta, !maximizing_player);
                if temp_val.1 < best_score {
                    best_score = temp_val.1;
                    best_col = j as usize;
                    best_piece = piece;
                }
                beta = beta.min(best_score);
                if alpha >= beta {
                    break;
                }
            }
            let piece = Piece::O;
            for j in 0..7{
                if self.board[0][j].is_some() {
                    continue;
                }
                let mut game_copy = self.clone();
                game_copy.place_piece(j, piece);
                let temp_val = game_copy.minimax(depth - 1, alpha, beta, !maximizing_player);
                if temp_val.1 < best_score {
                    best_score = temp_val.1;
                    best_col = j as usize;
                    best_piece = piece;
                }
                beta = beta.min(best_score);
                if alpha >= beta {
                    break;
                }
            }
            return(best_col, best_score, best_piece);
        }

        (best_col, best_score, best_piece)
        // let mut new_board = self.clone();
        // let pieces = [Piece::T, Piece::O];
        // let mut max_eval = i32::MIN;
        // let mut min_eval = i32::MAX;
        // let mut is_break = false;
        // for piece in pieces.iter() {
        //     for col in 0..7 {
        //         if is_break {
        //             break;
        //         }
        //         if let Some(row) = self.get_valid_row(col) {
        //             self.board[row][col] = Some(*piece);
        //
        //             let score = self.minimax(depth - 1, alpha, beta, !maximizing_player).1;
        //
        //             if maximizing_player {
        //                 if score > max_eval{
        //                     max_eval = score;
        //                     best_score = score;
        //                     best_col = Some(col);
        //                     best_piece = *piece;
        //                 }
        //                 alpha = alpha.max(score);
        //                 if beta <= alpha {
        //                     is_break = true;
        //                     self.board[row][col] = None;
        //                     break;
        //                 }
        //             } else {
        //                 if score < min_eval{
        //                     min_eval = score;
        //                     best_score = score;
        //                     best_col = Some(col);
        //                     best_piece = *piece;
        //                 }
        //                 beta = beta.min(score);
        //                 if beta <= alpha {
        //                     is_break = true;
        //                     self.board[row][col] = None;
        //                     break;
        //                 }
        //             }
        //             self.board[row][col] = None;
        //         }
        //     }
        // }
        //
        // if best_col.is_none() {
        //     (10, best_score, best_piece)
        // } else {
        //     (best_col.unwrap(), best_score, best_piece)
        // }
    }

    pub fn evaluate_board(&self, maximizing: bool) -> i32 {
        let mut score = 0;
        let mut right = [None; 4];
        let mut bottom = [None; 4];
        let mut bottom_right = [None; 4];
        let mut top_right = [None; 4];

        for i in 0..6 {
            for j in 0..7 {
                for k in 0..4 {
                    // from (i, j) to the right
                    if j + k < 7 {
                        right[k] = self.board[i][j + k];
                    } else {
                        right[k] = None;
                    }
                    // from (i, j) to the bottom
                    if i + k < 6 {
                        bottom[k] = self.board[i + k][j];
                    } else {
                        bottom[k] = None;
                    }
                    // from (i, j) to bottom right
                    if i + k < 6 && j + k < 7 {
                        bottom_right[k] = self.board[i + k][j + k];
                    } else {
                        bottom_right[k] = None;
                    }
                    // from (i, j) to top right
                    if i as i32 - k as i32 >= 0 && j + k < 7 {
                        top_right[k] = self.board[i - k][j + k];
                    } else {
                        top_right[k] = None;
                    }
                }

                let score_1 = self.get_score(&right, maximizing);
                let score_2 = self.get_score(&bottom, maximizing);
                let score_3 = self.get_score(&bottom_right, maximizing);
                let score_4 = self.get_score(&top_right, maximizing);
                // calculate the total score for this position
                score += score_1 + score_2 + score_3 + score_4;
            }
        }
        score
    }

    fn get_score(&self, line: &[Option<Piece>; 4], maximizing: bool) -> i32 {
        let mut score = 0;
        // TOOT scores are positive, OTTO scores are negative
        // 10 for OO, -10 for TT
        for i in 0..line.len()-3 {
            if maximizing == true {
                // prioritize blocking win of OTTO over 2nd last win move
                if line[i] == Some(Piece::T) && line[i+1] == Some(Piece::O) && line[i+2] == Some(Piece::O) && line[i+3] == Some(Piece::T) {
                    return 1100
                }
                if line[i] == Some(Piece::O) && line[i+1] == Some(Piece::T) && line[i+2] == Some(Piece::T) && line[i+3] == Some(Piece::O) {
                    return -1000
                }
                if line[i] == Some(Piece::T) && line[i+1] == Some(Piece::O) && line[i+2] == Some(Piece::O) && line[i+3] == None{
                    return 100
                }
                if line[i] == Some(Piece::O) && line[i+1] == Some(Piece::T) && line[i+2] == Some(Piece::T) && line[i+3] == None{
                    return -110
                }
                if line[i] == None && line[i+1] == Some(Piece::O) && line[i+2] == Some(Piece::O) && line[i+3] == Some(Piece::T) {
                    return 100
                }
                if line[i] == None && line[i+1] == Some(Piece::T) && line[i+2] == Some(Piece::T) && line[i+3] == Some(Piece::O) {
                    return -110
                }
            } else {
                // prioritize blocking win of TOOT over 2nd last win move
                if line[i] == Some(Piece::T) && line[i+1] == Some(Piece::O) && line[i+2] == Some(Piece::O) && line[i+3] == Some(Piece::T) {
                    return 1000
                }
                if line[i] == Some(Piece::O) && line[i+1] == Some(Piece::T) && line[i+2] == Some(Piece::T) && line[i+3] == Some(Piece::O) {
                    return -1100
                }
                if line[i] == Some(Piece::T) && line[i+1] == Some(Piece::O) && line[i+2] == Some(Piece::O) && line[i+3] == None{
                    return 110
                }
                if line[i] == Some(Piece::O) && line[i+1] == Some(Piece::T) && line[i+2] == Some(Piece::T) && line[i+3] == None{
                    return -100
                }
                if line[i] == None && line[i+1] == Some(Piece::O) && line[i+2] == Some(Piece::O) && line[i+3] == Some(Piece::T) {
                    return 110
                }
                if line[i] == None && line[i+1] == Some(Piece::T) && line[i+2] == Some(Piece::T) && line[i+3] == Some(Piece::O) {
                    return -100
                }
            }
            if line[i] == None && line[i+1] == Some(Piece::T) && line[i+2] == Some(Piece::T) && line[i+3] == None {
                return -10
            }
            if line[i] == None && line[i+1] == Some(Piece::O) && line[i+2] == Some(Piece::O) && line[i+3] == None {
                return 10
            }
        }
        score
    }

    fn get_valid_row(&self, col: usize) -> Option<usize> {
        for row in 0..self.board.len() {
            if self.board[row][col].is_none() {
                return Some(row);
            }
        }
        None
    }

    fn undo_move(&mut self, col: usize) {
        for i in (0..6).rev() {
            if self.board[i][col].is_some() {
                self.board[i][col] = None;
                break;
            }
        }
    }

    pub fn winner(&self) -> Option<Player> {
        let mut right = [None; 4];
        let mut bottom = [None; 4];
        let mut bottom_right = [None; 4];
        let mut top_right = [None; 4];

        for i in 0..6 {
            for j in 0..7 {
                for k in 0..4 {
                    // from (i, j) to the right
                    if j + k < 7 {
                        right[k] = self.board[i][j + k];
                    } else {
                        right[k] = None;
                    }
                    // from (i, j) to the bottom
                    if i + k < 6 {
                        bottom[k] = self.board[i + k][j];
                    } else {
                        bottom[k] = None;
                    }
                    // from (i, j) to bottom right
                    if i + k < 6 && j + k < 7 {
                        bottom_right[k] = self.board[i + k][j + k];
                    } else {
                        bottom_right[k] = None;
                    }
                    // from (i, j) to top right
                    if i as i32 - k as i32 >= 0 && j + k < 7 {
                        top_right[k] = self.board[i - k][j + k];
                    } else {
                        top_right[k] = None;
                    }
                }

                // check for a win
                if Self::check_win(&right) {
                    return right[0].map(|p| p.into());
                }
                if Self::check_win(&bottom) {
                    return bottom[0].map(|p| p.into());
                }
                if Self::check_win(&bottom_right) {
                    return bottom_right[0].map(|p| p.into());
                }
                if Self::check_win(&top_right) {
                    return top_right[0].map(|p| p.into());
                }
            }
        }
        // game is still running
        None
    }

    pub fn is_draw(&self) -> bool{
        if self.board.iter().all(|row| row.iter().all(|cell| cell.is_some())) {
            return true;
        }
        return false;
    }

    pub fn is_over(&self) -> bool{
        return match self.winner() {
            None => {
                false
            }
            Some(_) => {
                true
            }
        }
    }

    fn check_win(line: &[Option<Piece>; 4]) -> bool {
        let mut toot_found = false;
        let mut otto_found = false;

        for i in 0..line.len()-3 {
            if line[i] == Some(Piece::T) && line[i+1] == Some(Piece::O) && line[i+2] == Some(Piece::O) && line[i+3] == Some(Piece::T) {
                toot_found = true;
            }
            if line[i] == Some(Piece::O) && line[i+1] == Some(Piece::T) && line[i+2] == Some(Piece::T) && line[i+3] == Some(Piece::O) {
                otto_found = true;
            }
        }

        toot_found || otto_found
    }

    // Print the current state of the game board
    pub fn print_board(&self) {
        for row in &self.board {
            for cell in row {
                match cell {
                    Some(Piece::T) => print!("T "),
                    Some(Piece::O) => print!("O "),
                    None => print!("- "),
                }
            }
            println!();
        }
    }
}


