mod board;
mod game;

use std::io::Write;

use board::Player;

pub use board::Board;

pub trait GamePlayer {
    fn next_move(&mut self, board: &mut Board, this_player: Player);
}

pub struct GreedyPlayer;

impl GamePlayer for GreedyPlayer {
    fn next_move(&mut self, board: &mut Board, this_player: Player) {
        let first_free = board.iter().position(|p| p.is_none()).unwrap();
        board.set(first_free, Some(this_player));
    }
}

pub struct HumanPlayer;

impl GamePlayer for HumanPlayer {
    fn next_move(&mut self, board: &mut Board, this_player: Player) {
        loop {
            print!("{board}where to put the next {this_player}? (0-8): ");

            std::io::stdout().flush().unwrap();
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();

            match buf.trim().parse() {
                Ok(number) if number < 9 => match board.get(number) {
                    None => {
                        board.set(number, Some(this_player));
                        return;
                    }
                    Some(_) => {
                        println!("Field is occupied already.")
                    }
                },
                Ok(_) | Err(_) => {
                    println!("Invalid input.")
                }
            }
        }
    }
}
