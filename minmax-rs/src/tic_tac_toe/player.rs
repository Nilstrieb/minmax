use std::io::Write;

use crate::{GamePlayer, Player};

use super::TicTacToe;

#[derive(Clone, Default)]
pub struct HumanPlayer;

impl GamePlayer<TicTacToe> for HumanPlayer {
    fn next_move(&mut self, board: &mut TicTacToe, this_player: Player) {
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
