mod board;
mod game;

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
