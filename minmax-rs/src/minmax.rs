use std::time::Instant;

use crate::{Game, GamePlayer, Player, Score, State};

#[derive(Clone)]
pub struct PerfectPlayer<G: Game> {
    best_move: Option<G::Move>,
    max_depth: Option<usize>,
    print_time: bool,
}

impl<G: Game> Default for PerfectPlayer<G> {
    fn default() -> Self {
        Self::new(true)
    }
}

impl<G: Game> PerfectPlayer<G> {
    pub fn new(print_time: bool) -> Self {
        Self {
            best_move: None,
            max_depth: G::REASONABLE_SEARCH_DEPTH,
            print_time,
        }
    }

    pub fn with_max_depth(mut self, max_depth: Option<usize>) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn best_move(&self) -> G::Move {
        self.best_move.expect("no move made yet")
    }

    fn minmax(
        &mut self,
        board: &mut G,
        maximizing_player: Player,
        alpha: Score,
        beta: Score,
        depth: usize,
    ) -> Score {
        // FIXME: Make depth decrease not increase.
        if let Some(max_depth) = self.max_depth && depth >= max_depth {
            // FIXME: Why do we have rate and result?
            return board.rate(maximizing_player);
        }

        match board.result() {
            State::Winner(winner) => {
                if winner == maximizing_player {
                    Score::WON
                } else {
                    Score::LOST
                }
            }
            State::Draw => Score::TIE,
            State::InProgress => {
                let mut max_value = alpha;

                for pos in board.possible_moves() {
                    board.make_move(pos, maximizing_player);
                    let value =
                        -self.minmax(board, maximizing_player.opponent(), -beta, -max_value, depth + 1);

                    board.undo_move(pos);

                    if value > max_value {
                        max_value = value;
                        if depth == 0 {
                            self.best_move = Some(pos);
                        }
                    }
                    // Imagine a game tree like this
                    //    P(  )
                    //     /  \
                    // A(10) B(  ) <- we are here in the loop for the first child that returned 11.
                    //        /  \
                    //     C(11) D(  )
                    //
                    // Our beta parameter is 10, because that's the current max_value of our parent.
                    // If P plays B, we know that B will pick something _at least_ as good as C. This means
                    // that B will be -11 or worse. -11 is definitly worse than -10, so playing B is definitly
                    // a very bad idea, no matter the value of D. So don't even bother calculating the value of D
                    // and just break out. 
                    if max_value >= beta {
                        break;
                    }
                }

                max_value
            }
        }
    }
}

impl<G: Game> GamePlayer<G> for PerfectPlayer<G> {
    fn next_move(&mut self, board: &mut G, this_player: Player) {
        let start = Instant::now();
        self.best_move = None;
        self.minmax(board, this_player, Score::LOST, Score::WON, 0);

        board.make_move(self.best_move.expect("could not make move"), this_player);

        if self.print_time {
            let duration = start.elapsed();
            println!("Move took {duration:?}");
        }
    }
}
