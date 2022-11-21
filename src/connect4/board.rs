use std::ops::Index;

use crate::{Player, State};

type Position = Option<Player>;

const WIDTH: usize = 7;
const HEIGTH: usize = 4;
const BOARD_POSITIONS: usize = WIDTH * HEIGTH;

///  0  1  2  3  4  5  6
///  7  8  9 10 11 12 13
/// 14 15 16 17 18 19 20
/// 21 22 23 24 25 26 27
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
        match self.check_board() {
            State::Winner(winner) => State::Winner(winner),
            State::InProgress if self.positions.iter().all(|position| position.is_some()) => {
                State::Draw
            }
            State::InProgress => State::InProgress,
            State::Draw => unreachable!("check_board cannot tell a draw"),
        }
    }

    fn check_board(&self) -> State {
        self.check_columns()?;
        self.check_rows()?;
        self.check_diagonals()
    }

    fn check_columns(&self) -> State {
        for i in 0..WIDTH {
            self.check_four(i, i + WIDTH, i + 2 * WIDTH, i + 3 * WIDTH)?;
        }

        State::InProgress
    }

    fn check_rows(&self) -> State {
        for row_start in 0..HEIGTH {
            for offset in 0..4 {
                let start = row_start + offset;
                self.check_four(start, start + 1, start + 2, start + 3)?;
            }
        }

        State::InProgress
    }

    fn check_diagonals(&self) -> State {
        // */*
        for start in 3..WIDTH {
            const DIFF: usize = WIDTH - 1;
            self.check_four(start, start + DIFF, start + 2 * DIFF, start + 3 * DIFF)?;
        }

        // *\*
        for start in 0..4 {
            const DIFF: usize = WIDTH + 1;
            self.check_four(start, start + DIFF, start + 2 * DIFF, start + 3 * DIFF)?;
        }
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

#[cfg(test)]
mod tests {
    use crate::{Player, State};

    use super::Board;

    fn parse_board(board: &str) -> Board {
        let positions = board
            .chars()
            .filter(|char| !char.is_whitespace())
            .map(|char| match char {
                'X' => Some(Player::X),
                'O' => Some(Player::O),
                '_' => None,
                char => panic!("Invalid char in board: `{char}`"),
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect(&format!(
                "not enough positions provided: {}",
                board.chars().filter(|c| !c.is_whitespace()).count()
            ));

        Board { positions }
    }

    fn test(board: &str, state: State) {
        let board = parse_board(board);
        assert_eq!(board.result(), state);
    }

    #[test]
    fn draw() {
        test(
            "
        XOOOXOX
        XOOOXOX
        OXXXOXO
        XOOOXXX
        ",
            State::Draw,
        );
    }

    #[test]
    fn full_winner() {
        test(
            "
        XOOOXOX
        XOOOXOX
        OXXXOXO
        XOOOXOX
        ",
            State::Winner(Player::O),
        );
    }

    #[test]
    fn three_rows() {
        test(
            "
        XXX_OOO
        _XXX___
        X_OOO__
        OOO____
        ",
            State::InProgress,
        );
    }
}
