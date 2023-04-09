use std::io;
use std::io::Write;

use common::Connect4::Connect4;
use common::TootOtto::{Piece, Player, TootOtto};
use common::TootOtto::Piece::T;
use common::TootOtto::Player::Toot;

fn main() {
    let mut game = Connect4::new();

    loop {
        println!("Enter column:");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let col = input.trim().parse().unwrap(); // Parse the user input to an integer


        game.user_move(col);
        game.ai_move(3);

        game.print_board();
    }
}