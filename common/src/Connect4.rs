use std::arch::x86_64::CpuidResult;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Connect4 {
    board: [[Option<Piece>; 7]; 6],
    current_player: Player,
    against_ai: bool,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    R,
    Y,
}

#[derive(PartialEq, Clone)]
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
    pub fn new(against_ai: bool) -> Connect4 {
        Connect4 {
            board: [[None; 7]; 6],
            current_player: Player::Red,
            against_ai,
        }
    }
    fn check_bounds(&self, col: usize) -> bool {
        col < 7
    }
    fn get_current_player(&self) -> Player {
        self.current_player.clone()
    }
    fn get_grid(&self) -> [[Option<Piece>; 7]; 6] {
        self.board.clone()
    }

    pub fn red_move(&mut self, column: usize, piece: Piece) {
        if self.current_player != Player::Red {
            return;
        }
        // check if the given column is valid
        if column >= 7 {
            return;
        }
        // get the lowest available row in the selected column
        let mut row = None;
        for i in (0..6).rev() {
            if self.board[i][column].is_none() {
                row = Some(i);
                break;
            }
        }
        // if the column is full, return an error
        if row.is_none() {
            return;
        }
        // place the Red piece at the selected position
        self.board[row.unwrap()][column] = Some(piece);
        // switch to the next player (Yellow)
        self.current_player = Player::Yellow;
    }

    pub fn yellow_move(&mut self, column: usize, piece: Piece) {
        if self.current_player != Player::Yellow {
            return;
        }
        if column >= 7 {
            return;
        }
        // get the lowest available row in the selected column
        let mut row = None;
        for i in (0..6).rev() {
            if self.board[i][column].is_none() {
                row = Some(i);
                break;
            }
        }
        // if the column is full, return an error
        if row.is_none() {
            return;
        }
        // place the Yellow piece at the selected position
        self.board[row.unwrap()][column] = Some(piece);
        // switch to the next player (Red)
        self.current_player = Player::Red;
    }

    //ai is always yellow (no option provided in the current implementation)
    pub fn ai_move(&mut self) {
        let mut piece = Piece::Y;
        let mut column_to_choose = 0;
        if self.against_ai {
            column_to_choose = self.choose();
        }
       self.yellow_move(column_to_choose, piece);
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

    pub fn is_draw(&self) -> bool{
        if self.board.iter().all(|row| row.iter().all(|cell| cell.is_some())) {
            return true;
        }
        return false;
    }
    fn choose(&self) -> usize {
        let mut best_score = i32::MIN;
        let mut best_column = 0;

        for column in 0..7 {
            let mut cloned_board = self.clone();
            cloned_board.red_move(column, Piece::R);

            let score = cloned_board.get_score(Player::Yellow);

            if score > best_score {
                best_score = score;
                best_column = column;
            }
        }
        best_column
    }

    fn get_score(&self, player: Player) -> i32 {
        let pieces = match player {
            Player::Red => Piece::R,
            Player::Yellow => Piece::Y,
        };
        let mut score = 0;
        // check horizontal
        for row in 0..6 {
            for col in 0..4 {
                let mut found = true;
                for i in 0..4 {
                    if self.board[row][col+i] != Some(pieces) {
                        found = false;
                        break;
                    }
                }
                if found {
                    score += 1;
                }
            }
        }
        // check vertical
        for row in 0..3 {
            for col in 0..7 {
                let mut found = true;
                for i in 0..4 {
                    if self.board[row+i][col] != Some(pieces) {
                        found = false;
                        break;
                    }
                }
                if found {
                    score += 1;
                }
            }
        }
        // check diagonal \
        for row in 0..3 {
            for col in 0..4 {
                let mut found = true;
                for i in 0..4 {
                    if self.board[row+i][col+i] != Some(pieces) {
                        found = false;
                        break;
                    }
                }
                if found {
                    score += 1;
                }
            }
        }
        // check diagonal /
        for row in 0..3 {
            for col in 3..7 {
                let mut found = true;
                for i in 0..4 {
                    if self.board[row+i][col-i] != Some(pieces) {
                        found = false;
                        break;
                    }
                }
                if found {
                    score += 1;
                }
            }
        }
        score
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

