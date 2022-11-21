#![feature(never_type, try_trait_v2)]

pub mod connect4;
pub mod tic_tac_toe;

mod player;

pub use player::{Player, State};

pub trait Game {
    type Board;
}

pub trait GamePlayer<G: Game>: Default {
    fn next_move(&mut self, board: &mut G::Board, this_player: Player);
}
