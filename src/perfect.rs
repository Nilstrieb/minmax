use std::ops::Neg;

use crate::{Board, GamePlayer, Player, State};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Score {
    Lost = -1,
    Tie = 0,
    Won = 1,
}

impl Neg for Score {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Lost => Self::Won,
            Self::Tie => Self::Tie,
            Self::Won => Self::Lost,
        }
    }
}

#[derive(Clone)]
pub struct PerfectPlayer {
    best_move: usize,
}

impl Default for PerfectPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl PerfectPlayer {
    pub fn new() -> Self {
        Self {
            best_move: usize::MAX,
        }
    }

    fn minmax(&mut self, board: &mut Board, player: Player, depth: usize) -> Score {
        if depth < 2 {
            //print!("{board}{}| playing {player}: ", " ".repeat(depth));
        }
        match board.result() {
            State::Winner(winner) => {
                if depth < 2 {
                    //println!("   a winner {winner}");
                }
                if winner == player {
                    Score::Won
                } else {
                    Score::Lost
                }
            }
            State::Draw => {
                if depth < 2 {
                    //println!("this is gonna be a draw");
                }
                Score::Tie
            }
            State::InProgress => {
                if depth < 2 {
                    //println!("not done yet");
                }
                let mut max_value = Score::Lost;

                debug_assert!(
                    !board.iter().all(|x| x.is_some()),
                    "the board is full but state is InProgress"
                );

                for (i, pos) in board.iter().enumerate() {
                    if pos.is_some() {
                        continue;
                    }

                    board.set(i, Some(player));
                    let value = -self.minmax(board, player.opponent(), depth + 1);

                    if depth < 2 {
                        if i == 8 {
                            //println!("AAA\n{board}AAAA");
                        }
                        //println!("{}^| {i} {player} -> {:?}", " ".repeat(depth), value);
                    }

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
