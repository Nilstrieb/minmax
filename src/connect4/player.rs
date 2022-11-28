use std::io::Write;

use crate::{Game, GamePlayer, Player};

use super::Connect4;

#[derive(Clone, Default)]
pub struct HumanPlayer;

impl GamePlayer<Connect4> for HumanPlayer {
    fn next_move(&mut self, board: &mut Connect4, this_player: Player) {
        loop {
            print!("{board}where to put the next {this_player}? (0-7): ");

            std::io::stdout().flush().unwrap();
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();

            match buf.trim().parse() {
                Ok(number) if number < 7 => match board[number] {
                    None => {
                        board.make_move(number, this_player);
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
