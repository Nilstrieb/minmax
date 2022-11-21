#![feature(never_type, try_trait_v2)]

mod board;
pub mod connect4;
mod game;
mod perfect;
mod player;

use std::io::Write;

pub use board::Board;
pub use perfect::PerfectPlayer;
pub use player::{Player, State};

pub trait GamePlayer: Default {
    fn next_move(&mut self, board: &mut Board, this_player: Player);
}

#[derive(Clone, Default)]
pub struct GreedyPlayer;

impl GamePlayer for GreedyPlayer {
    fn next_move(&mut self, board: &mut Board, this_player: Player) {
        let first_free = board.iter().position(|p| p.is_none()).unwrap();
        board.set(first_free, Some(this_player));
    }
}

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
pub struct RandomPlayer;

fn fun_random() -> u64 {
    use std::hash::{BuildHasher, Hasher};
    std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish()
}

impl GamePlayer for RandomPlayer {
    fn next_move(&mut self, board: &mut Board, this_player: Player) {
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

#[cfg(test)]
mod tests {
    use crate::{Board, GamePlayer, GreedyPlayer, PerfectPlayer, Player, RandomPlayer};

    fn assert_win_ratio<X: GamePlayer, O: GamePlayer>(runs: u64, x_win_ratio: f64) {
        let mut results = [0u64, 0, 0];

        for _ in 0..runs {
            let result = Board::default_play::<X, O>();
            let idx = Player::as_u8(result);
            results[idx as usize] += 1;
        }

        let total = results.iter().copied().sum::<u64>();

        let ratio = (total as f64) / (results[0] as f64);
        println!("{ratio} >= {x_win_ratio}");
        assert!(ratio >= x_win_ratio);
    }

    #[test]
    fn perfect_always_beats_greedy() {
        assert_win_ratio::<PerfectPlayer, GreedyPlayer>(20, 1.0);
    }

    #[test]
    fn perfect_beats_random() {
        assert_win_ratio::<PerfectPlayer, RandomPlayer>(10, 0.95);
    }
}
