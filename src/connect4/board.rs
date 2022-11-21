use std::ops::Index;

use crate::{Player, State};

type Position = Option<Player>;

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 4;
const BOARD_POSITIONS: usize = BOARD_WIDTH * BOARD_HEIGHT;

pub struct Board {
    positions: [Position; BOARD_POSITIONS],
}

impl Board {
    pub fn new() -> Self {
        Self {
            positions: [None; BOARD_POSITIONS],
        }
    }

    pub fn result(&self) -> State {
        self.check_board()
    }

    fn check_board(&self) -> State {
        self.check_columns()?;
        self.check_rows()?;
        self.check_diagonals()
    }

    fn check_columns(&self) -> State {
        for i in 0..BOARD_WIDTH {
            self.check_four(i, i + BOARD_WIDTH, i + 2 * BOARD_WIDTH, i + 3 * BOARD_WIDTH)?;
        }

        State::InProgress
    }

    fn check_rows(&self) -> State {
        for row_start in 0..BOARD_HEIGHT {
            for offset in 0..4 {
                let start = row_start + offset;
                self.check_four(start, start + 1, start + 2, start + 3)?;
            }
        }

        State::InProgress
    }

    fn check_diagonals(&self) -> State {
        State::InProgress
    }

    fn check_four(&self, a: usize, b: usize, c: usize, d: usize) -> State {
        self[a]
            .map(|player| {
                if player == self[a] && player == self[b] && player == self[c] && player == self[d]
                {
                    State::Winner(player)
                } else {
                    State::InProgress
                }
            })
            .unwrap_or(State::InProgress)
    }
}

impl Index<usize> for Board {
    type Output = Position;

    fn index(&self, index: usize) -> &Self::Output {
        &self.positions[index]
    }
}
