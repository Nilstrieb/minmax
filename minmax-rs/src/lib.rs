#![feature(
    never_type,
    try_trait_v2,
    return_position_impl_trait_in_trait,
    let_chains,
)]
#![allow(incomplete_features)]

pub mod connect4;
mod minmax;
pub mod player;
mod state;
pub mod tic_tac_toe;

use std::fmt::Display;

use state::IgnorePlayer;

pub use self::minmax::PerfectPlayer;
pub use self::state::{Player, Score, State};

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

    /// Returns an iterator of all possible moves. Should be ordered best to worst.
    fn possible_moves(&self) -> impl Iterator<Item = Self::Move>;

    fn result(&self) -> State;

    /// Only called if [`GameBoard::REASONABLE_SEARCH_DEPTH`] is `Some`.
    fn rate(&self, player: Player) -> Score<IgnorePlayer>;

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

#[cfg(test)]
fn assert_win_ratio<G: Game, X: GamePlayer<G>, O: GamePlayer<G>>(
    runs: u64,
    x_win_ratio: f64,
    x: impl Fn() -> X,
    o: impl Fn() -> O,
) {
    let mut results = [0u64, 0, 0];

    for _ in 0..runs {
        let result = G::empty().play::<X, O>(&mut x(), &mut o());
        let idx = Player::as_u8(result);
        results[idx as usize] += 1;
    }

    let total = results.iter().copied().sum::<u64>();

    let ratio = (total as f64) / (results[0] as f64);
    println!("{ratio} >= {x_win_ratio}");
    assert!(ratio >= x_win_ratio);
}
