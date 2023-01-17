//! The core logic of the minmax algorithm.
//! It's generic over [`Game`] and works with every game.

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
        grandparents_favourite_child_alpha: Score,
        parents_favourite_child_beta: Score,
        depth: usize,
    ) -> Score {
        // FIXME: Make depth decrease not increase.
        if let Some(max_depth) = self.max_depth && depth >= max_depth {
            return board.rate(maximizing_player);
        }

        match board.result() {
            State::Winner(winner) => {
                if winner == maximizing_player {
                    // Our maximizing player wins the game, so this node is a win for it.
                    Score::WON
                } else {
                    // The maximizing player lost the board here, so the node is a loss.
                    Score::LOST
                }
            }
            State::Draw => Score::TIE,
            State::InProgress => {
                // The board isn't done yet, go deeper!
                // The alpha is the favourite (highest reward) child of our grandparent (who's on our side!).
                let mut max_value = grandparents_favourite_child_alpha;

                for pos in board.possible_moves() {
                    board.make_move(pos, maximizing_player);

                    // Calculate the move for the nested call. This goes one layer deeper.
                    // The number represents the *return* value of the node.
                    // A is a very bad child for P (-10). B seems promising for now as 11 is a lot bigger than -10.
                    //
                    // X    P(  ) max_value: -10
                    //       /  \
                    // O A(10) B(  ) <- we are here in the loop and about to call D
                    //          /  \
                    // X    C(11) D(  )
                    let value = -self.minmax(
                        board,
                        // The player that will maximize this round is now the opponent. This layer it was our original
                        // opponent, O, but in the nested round it's X's turn again so they will try to maximize their score.
                        maximizing_player.opponent(),
                        // Our childs grandparent is out parent. We use negative to normalize the value into our child's
                        // layer again. Every time a score is moved between a layer it has to be normalized like this.
                        -parents_favourite_child_beta,
                        // We are the parent of our child. Normalize the value with the negative sign.
                        -max_value,
                        depth + 1,
                    );

                    board.undo_move(pos);

                    if value > max_value {
                        max_value = value;
                        if depth == 0 {
                            self.best_move = Some(pos);
                        }

                        // Imagine a game tree like this
                        // The goal of this entire recursion is to find the best play for P (X).
                        //
                        // X     P(  ) max_value: 10
                        //        /  \
                        // O  A(-10) B(  ) <- we are here in the loop for the first child that returned -10 (looks like 10 for us).
                        //            /  \
                        // X       C(-10) D(  )
                        //
                        // Our beta parameter is -10, because that's our best (for P) currently known sibling.
                        // If P plays B, we know that B will pick something _at least_ as good as C. This means
                        // that B will pick something that's -10 or lower for P. -10 is definitly worse than 10, the current
                        // max_value for P, so playing B is definitly a very bad idea, no matter the horrors behind D.
                        // So don't even bother calculating the value of D and just break out.
                        if max_value >= parents_favourite_child_beta {
                            break;
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
        let start = Instant::now();
        self.best_move = None;

        // Get the rating for the one move we will make.
        self.minmax(board, this_player, Score::LOST, Score::WON, 0);

        board.make_move(
            self.best_move
                .unwrap_or_else(|| board.possible_moves().next().expect("cannot make move")),
            this_player,
        );

        if self.print_time {
            let duration = start.elapsed();
            println!("Move took {duration:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_win_ratio;
    use crate::connect4::board::Connect4;
    use crate::minmax::PerfectPlayer;

    use crate::player::{GreedyPlayer, RandomPlayer};
    use crate::tic_tac_toe::TicTacToe;

    #[test]
    fn perfect_always_beats_greedy() {
        assert_win_ratio::<TicTacToe, _, _>(1, 1.0, || PerfectPlayer::new(false), || GreedyPlayer);
        assert_win_ratio::<Connect4, _, _>(
            1,
            1.0,
            || PerfectPlayer::new(false).with_max_depth(Some(8)),
            || GreedyPlayer,
        );
    }

    #[test]
    fn perfect_beats_random() {
        assert_win_ratio::<TicTacToe, _, _>(
            10,
            0.95,
            || PerfectPlayer::new(false),
            || RandomPlayer,
        );
        assert_win_ratio::<Connect4, _, _>(
            5,
            0.95,
            || PerfectPlayer::new(false).with_max_depth(Some(7)),
            || RandomPlayer,
        );
    }

    #[test]
    fn good_beat_bad() {
        assert_win_ratio::<TicTacToe, _, _>(
            1,
            1.0,
            || PerfectPlayer::new(false).with_max_depth(Some(7)),
            || PerfectPlayer::new(false).with_max_depth(Some(5)),
        );
        assert_win_ratio::<Connect4, _, _>(
            1,
            1.0,
            || PerfectPlayer::new(false).with_max_depth(Some(7)),
            || PerfectPlayer::new(false).with_max_depth(Some(5)),
        );
    }
}
