use crate::{
    board::{Player, State},
    Board, GamePlayer,
};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Score(isize);

#[derive(Clone)]
pub struct PerfectPlayer {
    best_move: usize,
}

impl PerfectPlayer {
    pub fn new() -> Self {
        Self {
            best_move: usize::MAX,
        }
    }

    fn minmax(&mut self, board: &mut Board, player: Player, depth: usize) -> Score {
        match board.result() {
            State::Winner(winner) => {
                if winner == player {
                    Score(1)
                } else {
                    Score(-1)
                }
            }
            State::Draw => Score(0),
            State::InProgress => {
                let mut max_value = Score(isize::MIN);

                debug_assert!(
                    !board.iter().all(|x| x.is_some()),
                    "the board is full but state is InProgress"
                );

                for (i, pos) in board.iter().enumerate() {
                    if pos.is_some() {
                        continue;
                    }

                    board.set(i, Some(player));
                    let value = self.minmax(board, player.opponent(), depth + 1);
                    board.set(i, None);

                    if value > max_value {
                        max_value = value;
                        if depth == 0 {
                            self.best_move = i;
                        }
                    }
                }

                max_value
            }
        }
    }
}

impl GamePlayer for PerfectPlayer {
    fn next_move(&mut self, board: &mut Board, this_player: Player) {
        self.best_move = usize::MAX;
        self.minmax(board, this_player, 0);

        if self.best_move == usize::MAX {
            panic!("could not make a move");
        }

        board.set(self.best_move, Some(this_player));
    }
}
