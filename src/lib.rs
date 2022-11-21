#![feature(never_type, try_trait_v2, return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

pub mod connect4;
mod minmax;
pub mod tic_tac_toe;

mod player;

use self::minmax::GameBoard;
pub use player::{Player, State};

pub trait Game {
    type Board: GameBoard;
}

pub trait GamePlayer<G: Game>: Default {
    fn next_move(&mut self, board: &mut G::Board, this_player: Player);
}
