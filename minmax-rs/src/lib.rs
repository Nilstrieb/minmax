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

pub mod player;

use std::{fmt::Display, ops::Neg};

pub use self::minmax::PerfectPlayer;
pub use player::{Player, State};

pub trait GamePlayer<G: ?Sized + Game> {
    fn next_move(&mut self, board: &mut G, this_player: Player);
}

impl<G: Game, P: GamePlayer<G> + ?Sized> GamePlayer<G> for &mut P {
    fn next_move(&mut self, board: &mut G, this_player: Player) {
        P::next_move(self, board, this_player)
    }
}

impl<G: Game, P: GamePlayer<G> + ?Sized> GamePlayer<G> for Box<P> {
    fn next_move(&mut self, board: &mut G, this_player: Player) {
        P::next_move(self, board, this_player)
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score(i32);

impl Score {
    // Due to the nature of two's completement, we can't actually negate this properly, so add 1.
    const LOST: Self = Self(i32::MIN + 1);
    const TIE: Self = Self(0);
    const WON: Self = Self(i32::MAX);

    pub fn new(int: i32) -> Self {
        Self(int)
    }

    #[allow(unused)]
    fn randomize(self) -> Self {
        let score = self.0 as f32;
        let rand = rand::thread_rng();
        self
    }
}

impl Neg for Score {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
