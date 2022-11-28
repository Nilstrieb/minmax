use std::ops::Neg;

use crate::{Game, GamePlayer, Player, State};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score(i32);

impl Score {
    const LOST: Self = Self(i32::MIN);
    const TIE: Self = Self(0);
    const WON: Self = Self(i32::MAX);

    pub fn new(int: i32) -> Self {
        Self(int)
    }
}

impl Neg for Score {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

#[derive(Clone)]
pub struct PerfectPlayer<B: Game> {
    best_move: Option<B::Move>,
}

impl<B: Game> Default for PerfectPlayer<B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: Game> PerfectPlayer<B> {
    pub fn new() -> Self {
        Self { best_move: None }
    }

    fn minmax(&mut self, board: &mut B, player: Player, depth: usize) -> Score {
        if let Some(max_depth) = B::REASONABLE_SEARCH_DEPTH && depth >= max_depth {
            return board.rate(player);
        }

        match board.result() {
            State::Winner(winner) => {
                if winner == player {
                    Score::WON
                } else {
                    Score::LOST
                }
            }
            State::Draw => Score::TIE,
            State::InProgress => {
                let mut max_value = Score::LOST;

                for pos in board.possible_moves() {
                    board.make_move(pos, player);
                    let value = -self.minmax(board, player.opponent(), depth + 1);

                    board.undo_move(pos);

                    if value > max_value {
                        max_value = value;
                        if depth == 0 {
                            self.best_move = Some(pos);
                        }
                    }
                }

                max_value
            }
        }
    }
}

impl<G: Game> GamePlayer<G> for PerfectPlayer<G> {
    fn next_move(&mut self, board: &mut G, this_player: Player) {
        self.best_move = None;
        self.minmax(board, this_player, 0);

        board.make_move(self.best_move.expect("could not make move"), this_player);
    }
}
