use std::io::Write;

use crate::{GamePlayer, Player};

use super::TicTacToe;

#[derive(Clone, Default)]
pub struct GreedyPlayer;

impl GamePlayer<TicTacToe> for GreedyPlayer {
    fn next_move(&mut self, board: &mut TicTacToe, this_player: Player) {
        let first_free = board.iter().position(|p| p.is_none()).unwrap();
        board.set(first_free, Some(this_player));
    }
}

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

#[derive(Clone, Default)]
pub struct RandomPlayer;

fn fun_random() -> u64 {
    use std::hash::{BuildHasher, Hasher};
    std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish()
}

impl GamePlayer<TicTacToe> for RandomPlayer {
    fn next_move(&mut self, board: &mut TicTacToe, this_player: Player) {
        loop {
            let next = (fun_random() % 9) as usize;
            match board.get(next) {
                Some(_) => {}
                None => {
                    board.set(next, Some(this_player));
                    return;
                }
            }
        }
    }
}
