use serde::{Deserialize, Serialize};
use crate::Connect4::Piece::R;

#[derive(Clone, Debug)]
pub struct Connect4 {
    board: [[Option<Piece>; 7]; 6],
    current_player: Player,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Piece {
    R,
    Y,
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Player {
    Red,
    Yellow,
}

impl From<Piece> for Player {
    fn from(piece: Piece) -> Self {
        match piece {
            Piece::R => Player::Red,
            Piece::Y => Player::Yellow,
        }
    }
}

impl Connect4 {
    pub fn new() -> Connect4 {
        Connect4 {
            board: [[None; 7]; 6],
            current_player: Player::Red,
        }
    }
    fn check_bounds(&self, col: usize) -> bool {
        col < 7
    }

    pub fn user_move(&mut self, column: usize) -> bool {
        let mut done = false;
        let piece = match self.current_player {
            Player::Yellow => Piece::Y,
            Player::Red => Piece::R,
        };
        if !self.check_bounds(column) {
            return false;
        }
        if self.place_piece(column, piece).is_some() {
            done = true;
            self.current_player = match self.current_player {
                Player::Red => Player::Yellow,
                Player::Yellow => Player::Red,
            };
        }
        done
    }

    fn place_piece(&mut self, column: usize, piece: Piece) -> Option<usize>{
        // get the lowest available row in the selected column
        let mut row = None;
        for i in (0..6).rev() {
            if self.board[i][column].is_none() {
                row = Some(i);
                break;
            }
        }
        // if the column is full, return None
        if row.is_none() {
            return None;
        }
        // place the Red piece at the selected position
        self.board[row.unwrap()][column] = Some(piece);
        //return row where the data was added
        row
    }

    fn remove_piece(&mut self, row: usize, col: usize) {
        self.board[row][col] = None;
    }

    pub fn ai_move(&mut self, depth: usize) -> bool{
        let mut done = false;
        let (column, score) = self.minimax(depth as i32, true);
        if column < 7 {
            let piece = if self.current_player == Player::Red { Piece::R } else { Piece::Y };
            if self.place_piece(column, piece).is_some() {
                done = true;
                self.current_player = match self.current_player {
                    Player::Red => Player::Yellow,
                    Player::Yellow => Player::Red,
                };
            }
        }
        done
    }

    fn minimax(&mut self, depth: i32, maximizing_player: bool) -> (usize, i32) {
        if depth == 0 || self.is_draw() || self.is_over() {
            return (0, self.evaluate_board(maximizing_player));
        }

        let mut best_score = if maximizing_player {i32::MIN} else {i32::MAX};
        let mut best_col = 0;
        for column in 0..7 {
            if self.board[0][column].is_some() {
                continue;
            }
            let piece = if self.current_player == Player::Red { Piece::R } else { Piece::Y };
            let row = self.place_piece(column, piece);
            if row.is_some() {
                let (_, score) = self.minimax(depth - 1, !maximizing_player);
                self.remove_piece(row.unwrap(), column);
                // Update the best move and score if we found a better one
                if maximizing_player && score > best_score {
                    best_score = score;
                    best_col = column;
                } else if !maximizing_player && score < best_score {
                    best_score = score;
                    best_col = column;
                }
            }
        }
        (best_col, best_score)
    }

    fn evaluate_board(&self, maximizing_player: bool) -> i32 {
        let mut score = 0;
        // Evaluate horizontal lines
        for row in 0..6 {
            for col in 0..4 {
                if let Some(piece) = self.board[row][col] {
                    let mut length = 1;
                    for i in 1..4 {
                        if self.board[row][col + i] == Some(piece) {
                            length += 1;
                        } else {
                            break;
                        }
                    }
                    score += self.get_score(length, maximizing_player);
                }
            }
        }

        // Evaluate vertical lines
        for row in 0..3 {
            for col in 0..7 {
                if let Some(piece) = self.board[row][col] {
                    let mut length = 1;
                    for i in 1..4 {
                        if self.board[row + i][col] == Some(piece) {
                            length += 1;
                        } else {
                            break;
                        }
                    }
                    score += self.get_score(length, maximizing_player);
                }
            }
        }

        // Evaluate diagonal lines
        for row in 0..3 {
            for col in 0..4 {
                if let Some(piece) = self.board[row][col] {
                    let mut length = 1;
                    for i in 1..4 {
                        if self.board[row + i][col + i] == Some(piece) {
                            length += 1;
                        } else {
                            break;
                        }
                    }
                    score += self.get_score(length, maximizing_player);
                }
                if let Some(piece) = self.board[row][col + 3] {
                    let mut length = 1;
                    for i in 1..4 {
                        if self.board[row + i][col + 3 - i] == Some(piece) {
                            length += 1;
                        } else {
                            break;
                        }
                    }
                    score += self.get_score(length, maximizing_player);
                }
            }
        }
        score
    }

    fn get_score(&self, length: i32, maximizing_player: bool) -> i32 {
        let mut score = 0;
        //ai will prioritize blocking as much as possible
        if maximizing_player {
            score += match length {
                2 => 10,
                3 => 100,
                _ => 0,
            };
        } else {
            score += match length {
                2 => -10,
                3 => -100,
                _ => 0,
            };
        }
        score
    }

    pub fn winner(&self) -> Option<Player> {
        let mut left_to_right = [None; 4];
        let mut top_to_bottom = [None; 4];
        let mut backward_slash = [None; 4];
        let mut forward_slash = [None; 4];

        for i in 0..6 {
            for j in 0..7 {
                for k in 0..4 {
                    // from (i, j) to the right
                    if j + k < 7 {
                        left_to_right[k] = self.board[i][j+k];
                    } else {
                        left_to_right[k] = None;
                    }
                    // from (i, j) to the bottom
                    if i + k < 6 {
                        top_to_bottom[k] = self.board[i+k][j];
                    } else {
                        top_to_bottom[k] = None;
                    }
                    // from (i, j) to bottom right
                    if i+k < 6 && j+k < 7 {
                        backward_slash[k] = self.board[i+k][j+k];
                    } else {
                        backward_slash[k] = None;
                    }
                    // from (i, j) to top right
                    if i as i32-k as i32 >= 0 && j+k < 7 {
                        forward_slash[k] = self.board[i-k][j+k];
                    } else {
                        forward_slash[k] = None;
                    }
                }
                // check for a win
                if Self::check_win(&left_to_right) {
                    return left_to_right[0].map(|p| p.into());
                }
                if Self::check_win(&top_to_bottom) {
                    return top_to_bottom[0].map(|p| p.into());
                }
                if Self::check_win(&backward_slash) {
                    return backward_slash[0].map(|p| p.into());
                }
                if Self::check_win(&forward_slash) {
                    return forward_slash[0].map(|p| p.into());
                }
            }
        }
        // game is still running
        None
    }

    fn check_win(line: &[Option<Piece>; 4]) -> bool {
        let mut red_found = false;
        let mut yellow_found = false;

        for i in 0..line.len()-3 {
            if line[i] == Some(Piece::R) && line[i+1] == Some(Piece::R) && line[i+2] == Some(Piece::R) && line[i+3] == Some(Piece::R) {
                red_found = true;
            }
            if line[i] == Some(Piece::Y) && line[i+1] == Some(Piece::Y) && line[i+2] == Some(Piece::Y) && line[i+3] == Some(Piece::Y) {
                yellow_found = true;
            }
        }

        red_found || yellow_found
    }

    pub fn is_draw(&self) -> bool{
        if self.board.iter().all(|row| row.iter().all(|cell| cell.is_some())) {
            return true;
        }
        return false;
    }

    fn is_over(&self) -> bool{
        return match self.winner() {
            None => {
                false
            }
            Some(_) => {
                true
            }
        }
    }

    // Print the current state of the game board
    pub fn print_board(&self) {
        for row in &self.board {
            for cell in row {
                match cell {
                    Some(Piece::R) => print!("R "),
                    Some(Piece::Y) => print!("Y "),
                    None => print!("- "),
                }
            }
            println!();
        }
    }
}