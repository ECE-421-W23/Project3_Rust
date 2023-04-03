use serde::{Deserialize, Serialize};

pub struct TootOtto {
    board: [[Option<Piece>; 7]; 6],
    current_player: Player,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    T,
    O,
}

#[derive(PartialEq)]
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

    fn check_bounds(&self, col: usize) -> bool {
        col < 7
    }

    // function to make a move by Player Toot
    pub fn make_move_by_toot(&mut self, column: usize, piece: Piece) -> Result<(), String> {
        if self.current_player != Player::Toot {
            return Err(String::from("Not Toot's turn"));
        }
        // check if the given column is valid
        if column >= 7 {
            return Err(String::from("Invalid column index"));
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
            return Err(String::from("Selected column is full"));
        }
        // place the Toot piece at the selected position
        self.board[row.unwrap()][column] = Some(piece);
        // switch to the next player (Otto)
        self.current_player = Player::Otto;
        Ok(())
    }

    pub fn make_move_by_otto(&mut self, column: usize, piece: Piece) -> Result<(), String> {
        if self.current_player != Player::Otto {
            return Err(String::from("Not Otto's turn"));
        }
        if column >= 7 {
            return Err(String::from("Invalid column index"));
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
            return Err(String::from("Selected column is full"));
        }
        // place the Toot piece at the selected position
        self.board[row.unwrap()][column] = Some(piece);
        // switch to the next player (Toot)
        self.current_player = Player::Toot;
        Ok(())
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

        // check for a draw
        if self.board.iter().all(|row| row.iter().all(|cell| cell.is_some())) {
            return Some(Player::Toot);
        }

        // game is still running
        None
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

