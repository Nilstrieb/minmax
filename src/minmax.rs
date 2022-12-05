use crate::{Game, GamePlayer, Player, Score, State};

#[derive(Clone)]
pub struct PerfectPlayer<G: Game> {
    best_move: Option<G::Move>,
    max_depth: Option<usize>,
}

impl<G: Game> Default for PerfectPlayer<G> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G: Game> PerfectPlayer<G> {
    pub fn new() -> Self {
        Self {
            best_move: None,
            max_depth: G::REASONABLE_SEARCH_DEPTH,
        }
    }

    pub fn with_max_depth(mut self, max_depth: Option<usize>) -> Self {
        self.max_depth = max_depth;
        self
    }

    fn minmax(&mut self, board: &mut G, player: Player, depth: usize) -> Score {
        if let Some(max_depth) = self.max_depth && depth >= max_depth {
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
                let mut max_value = Score::MIN;

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
        println!("{board}");

        self.best_move = None;
        self.minmax(board, this_player, 0);

        board.make_move(self.best_move.expect("could not make move"), this_player);
    }
}
