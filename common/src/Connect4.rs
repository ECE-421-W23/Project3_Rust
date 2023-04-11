use serde::{Deserialize, Serialize};
use rand::Rng;

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

    pub fn get_grid(&self) -> [[Option<Piece>; 7]; 6] {
        self.board.clone()
    }

    pub fn top_row(&self, col: usize) -> usize {
        for row in (0..6).rev() {
            if self.board[row][col].is_none() {
                return row;
            }
        }
        //if row is empty return anything greater than 6
        10
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

    fn place_piece(&mut self, column: usize, piece: Piece) -> Option<usize> {
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

    pub fn ai_move(&mut self, depth: usize) -> bool {
        let mut done = false;
        let piece = if self.current_player == Player::Red { Piece::R } else { Piece::Y };
        //if difficulty is easy, randomize ai input
        if depth.clone() == 1 {
            let mut rng = rand::thread_rng();
            loop {
                let random_col = rng.gen_range(0..7);
                if self.place_piece(random_col, piece).is_some() {
                    done = true;
                    break;
                }
            }
        }
        else {
            //reduce depth as we dont want to waste resources
            let new_depth = match depth {
                2 => 1,
                4 => 2,
                _ => 1,
            };
            let (column, _score) = self.minimax(new_depth as i32, piece.clone());
            if column < 7 && self.place_piece(column, piece).is_some() {
                done = true;
            }
        }
        self.current_player = match self.current_player {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        };
        done
    }

    fn minimax(&mut self, depth: i32, piece: Piece) -> (usize, i32) {
        if depth == 0 || self.is_draw() || self.is_over() {
            return (0, self.evaluate_board());
        }
        //AI is always minimising the score, so start with max
        let mut best_score = i32::MAX;
        let mut best_col = 0;
        for column in 0..7 {
            if self.board[0][column].is_some() {
                continue;
            }
            let row = self.place_piece(column, piece.clone());
            if row.is_some() {
                let next_piece = match piece {
                    Piece::R => Piece::Y,
                    Piece::Y => Piece::R,
                };
                let (_, score) = self.minimax(depth - 1, next_piece);
                self.remove_piece(row.unwrap(), column);
                // Update the best move and score if we found a better one
                if score < best_score {
                    best_score = score;
                    best_col = column;
                }
            }
        }
        (best_col, best_score)
    }

    fn evaluate_board(&self) -> i32 {
        let mut score = 0;
        let mut left_to_right = [None; 4];
        let mut top_to_bottom = [None; 4];
        let mut backward_slash = [None; 4];
        let mut forward_slash = [None; 4];

        for i in 0..6 {
            for j in 0..7 {
                for k in 0..4 {
                    // from (i, j) to the right
                    if j + k < 7 {
                        left_to_right[k] = self.board[i][j + k];
                    } else {
                        left_to_right[k] = None;
                    }
                    // from (i, j) to the bottom
                    if i + k < 6 {
                        top_to_bottom[k] = self.board[i + k][j];
                    } else {
                        top_to_bottom[k] = None;
                    }
                    // from (i, j) to bottom right
                    if i + k < 6 && j + k < 7 {
                        backward_slash[k] = self.board[i + k][j + k];
                    } else {
                        backward_slash[k] = None;
                    }
                    // from (i, j) to top right
                    if i as i32 - k as i32 >= 0 && j + k < 7 {
                        forward_slash[k] = self.board[i - k][j + k];
                    } else {
                        forward_slash[k] = None;
                    }
                }
                let score_1 = self.get_score(&left_to_right);
                let score_2 = self.get_score(&top_to_bottom);
                let score_3 = self.get_score(&forward_slash);
                let score_4 = self.get_score(&backward_slash);
                // calculate the total score for this position
                score += score_1 + score_2 + score_3 + score_4;
            }
        }
        score
    }

    fn get_score(&self, line: &[Option<Piece>; 4]) -> i32 {
        //red is always human and ai is always the minimising player
        let mut score = 0;
        let mut user_win = 0;
        let mut ai_win = 0;
        let mut empty_cell = 0;
        for i in 0..line.len(){
            if line[i] == Some(Piece::R) {
                user_win += 1; // Increment user_win count
                ai_win = 0; // Reset ai_win count
            } else if line[i] == Some(Piece::Y) {
                ai_win += 1; // Increment ai_win count
                user_win = 0; // Reset user_win count
            } else {
                empty_cell += 1;
            }
        }
        //if ai can win then prioritize winning
        if ai_win == 4 {
            score = -10000;
        }
        else if ai_win == 3 && empty_cell == 1 {
            score = -1000;
        }
        else if ai_win == 2 && empty_cell == 2 {
            score = -100;
        }
        else if ai_win == 1 && empty_cell == 3 {
            score = -10;
        }
        //block the move
        else if user_win == 3 && empty_cell == 1 {
            score = 10000;
        }
        else if user_win == 2 && empty_cell == 2 {
            score = 1000;
        }
        else if user_win == 1 && empty_cell == 3 {
            score = 100;
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
                        left_to_right[k] = self.board[i][j + k];
                    } else {
                        left_to_right[k] = None;
                    }
                    // from (i, j) to the bottom
                    if i + k < 6 {
                        top_to_bottom[k] = self.board[i + k][j];
                    } else {
                        top_to_bottom[k] = None;
                    }
                    // from (i, j) to bottom right
                    if i + k < 6 && j + k < 7 {
                        backward_slash[k] = self.board[i + k][j + k];
                    } else {
                        backward_slash[k] = None;
                    }
                    // from (i, j) to top right
                    if i as i32 - k as i32 >= 0 && j + k < 7 {
                        forward_slash[k] = self.board[i - k][j + k];
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

        for i in 0..line.len() - 3 {
            if line[i] == Some(Piece::R) && line[i + 1] == Some(Piece::R) && line[i + 2] == Some(Piece::R) && line[i + 3] == Some(Piece::R) {
                red_found = true;
            }
            if line[i] == Some(Piece::Y) && line[i + 1] == Some(Piece::Y) && line[i + 2] == Some(Piece::Y) && line[i + 3] == Some(Piece::Y) {
                yellow_found = true;
            }
        }
        red_found || yellow_found
    }

    pub fn is_draw(&self) -> bool {
        if self.board.iter().all(|row| row.iter().all(|cell| cell.is_some())) {
            return true;
        }
        return false;
    }

    fn is_over(&self) -> bool {
        return match self.winner() {
            None => {
                false
            }
            Some(_) => {
                true
            }
        };
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