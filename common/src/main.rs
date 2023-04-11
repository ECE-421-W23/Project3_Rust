use std::io;
use std::io::Write;

use common::Connect4::Connect4;

fn main() {
    /*
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

     */
    let mut game = Connect4::new();

    loop {
        println!("Enter column:");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let col = input.trim().parse().unwrap(); // Parse the user input to an integer
        game.user_move(col);
        game.ai_move(4);
        game.print_board();
    }
}