use common::TootOtto::{Piece, Player, TootOtto};
use common::TootOtto::Piece::T;
use common::TootOtto::Player::Toot;

use common::Connect4::{Connect4};
use common::Connect4::Piece::{R, Y};
use common::Connect4::Player::Red;

fn main() {
    /*
    let mut game = TootOtto::new();
    // game.print_board();
    game.make_move_by_toot(0, Piece::O);
    game.make_move_by_otto(1, Piece::T);
    game.make_move_by_toot(2, Piece::T);
    game.make_move_by_otto(3, Piece::T);
    game.make_move_by_toot(1, Piece::T);
    game.make_move_by_otto(2, Piece::T);
    game.make_move_by_toot(2, Piece::T);
    game.make_move_by_otto(3, Piece::T);
    game.make_move_by_toot(3, Piece::T);
    game.make_move_by_otto(3, Piece::O);
     */
    let mut game = Connect4::new(true);
    game.red_move(0, R);
    game.ai_move();
    game.red_move(3, R);
    game.ai_move();
    game.red_move(4, R);
    game.ai_move();
    game.red_move(5, R);
    game.ai_move();

    game.print_board();
    match game.winner() {
        None => {}
        Some(x) => {
            if x == Red {
                println!("Red Won!");
            } else {
                println!("Yellow Won!");
            }
        }
    }
}