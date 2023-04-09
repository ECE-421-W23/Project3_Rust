use common::TootOtto::{Piece, Player, TootOtto};
use common::TootOtto::Piece::T;
use common::TootOtto::Player::Toot;
use std::io::{self, Write};

fn main() {
    let mut game = TootOtto::new();

    loop {
        println!("Enter column and piece (e.g. '3 T'):");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut iter = input.trim().split_whitespace();
        let col = iter.next().unwrap().parse::<usize>().unwrap();
        let piece = match iter.next().unwrap() {
            "T" => Piece::T,
            "O" => Piece::O,
            _ => continue,
        };

        game.make_move_by_toot(col, piece);
        game.make_move_by_ai(1);

        game.print_board();
    }
    // let mut game = TootOtto::new();
    // // game.print_board();
    // // game.make_move_by_toot(0, Piece::T);
    // // game.make_move_by_otto(0, Piece::O);
    // // game.make_move_by_toot(0, Piece::O);
    // game.make_move_by_toot(0, Piece::T);
    // //game.make_move_by_otto(3, Piece::T);
    // game.make_move_by_toot(1, Piece::O);
    // //game.make_move_by_otto(2, Piece::T);
    // game.make_move_by_toot(2, Piece::O);
    // //game.make_move_by_toot(2, Piece::O);
    // // game.make_move_by_otto(2, Piece::T);
    //
    // // TOOT
    // println!("{}" , game.evaluate_board(true));
    // // OTTO
    // println!("{}" , game.evaluate_board(false));
    // println!("{:?}" , game.minimax(1,false).0);
    // game.make_move_by_ai(1);

    // game.print_board();
    // match game.winner() {
    //     None => {}
    //     Some(x) => {
    //         if x == Toot {
    //             println!("Toot Won!");
    //         } else {
    //             println!("Otto Won!");
    //         }
    //     }
    // }
}