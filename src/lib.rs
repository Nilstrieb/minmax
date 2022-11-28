#![feature(
    never_type,
    try_trait_v2,
    return_position_impl_trait_in_trait,
    let_chains
)]
#![allow(incomplete_features)]

pub mod connect4;
mod minmax;
pub mod tic_tac_toe;

mod player;

use std::fmt::Display;

use minmax::Score;
pub use player::{Player, State};

pub trait GamePlayer<G: ?Sized + Game>: Default {
    fn next_move(&mut self, board: &mut G, this_player: Player);
}

pub trait Game: Display {
    type Move: Copy;

    const REASONABLE_SEARCH_DEPTH: Option<usize>;

    fn empty() -> Self;

    fn possible_moves(&self) -> impl Iterator<Item = Self::Move>;

    fn result(&self) -> State;

    /// Only called if [`GameBoard::REASONABLE_SEARCH_DEPTH`] is `Some`.
    fn rate(&self, player: Player) -> Score;

    fn make_move(&mut self, position: Self::Move, player: Player);

    fn undo_move(&mut self, position: Self::Move);

    fn play<A: GamePlayer<Self>, B: GamePlayer<Self>>(
        &mut self,
        x: &mut A,
        o: &mut B,
    ) -> Option<Player> {
        let mut current_player = Player::X;

        loop {
            if current_player == Player::X {
                x.next_move(self, current_player);
            } else {
                o.next_move(self, current_player);
            }

            match self.result() {
                State::Winner(player) => return Some(player),
                State::Draw => {
                    return None;
                }
                State::InProgress => {}
            }

            current_player = current_player.opponent();
        }
    }
}
