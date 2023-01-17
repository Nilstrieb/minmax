use std::{
    fmt::{Display, Write},
    ops::{Index, IndexMut},
};

use crate::{
    state::{position_as_int, IgnorePlayer, Position},
    Game, Player, Score, State,
};

const WIDTH: usize = 7;
const HEIGTH: usize = 4;
const BOARD_POSITIONS: usize = WIDTH * HEIGTH;

///  0  1  2  3  4  5  6
///  7  8  9 10 11 12 13
/// 14 15 16 17 18 19 20
/// 21 22 23 24 25 26 27
#[derive(Clone)]
pub struct Connect4 {
    positions: [Position; BOARD_POSITIONS],
}

impl Connect4 {
    pub fn new() -> Self {
        Self {
            positions: [None; BOARD_POSITIONS],
        }
    }

    pub fn set_pos(&mut self, position: usize, value: Position) {
        self.positions[position] = value;
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
                let start = (row_start * WIDTH) + offset;
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
        // Instead of doing a branch after each field (slow) we just check all fields.
        // On each field, we get the integer value of the field (empty -> 0, X -> 1, O -> 16).
        // We sum them all. If all of them are X, the sum is 4. If all of them are O, the sum is 64.
        let sum = position_as_int(self[a])
            + position_as_int(self[b])
            + position_as_int(self[c])
            + position_as_int(self[d]);

        match sum {
            4 => State::Winner(Player::X),
            64 => State::Winner(Player::O),
            _ => State::InProgress,
        }
    }

    fn rate(&self, player: Player) -> Score<IgnorePlayer> {
        #[rustfmt::skip]
        const WIN_COUNT_TABLE: [i16; BOARD_POSITIONS] = [
            3, 4, 6, 7, 6, 4, 3,
            2, 4, 6, 7, 6, 4, 2,
            2, 4, 6, 7, 6, 4, 2,
            3, 4, 6, 7, 6, 4, 3,
        ];

        let score_player = |player: Player| {
            self.positions
                .iter()
                .enumerate()
                .filter(|(_, state)| **state == Some(player))
                .map(|(pos, _)| WIN_COUNT_TABLE[pos])
                .sum::<i16>()
        };

        Score::new(i32::from(
            score_player(player) - score_player(player.opponent()),
        ))
    }

    pub fn drop_player(&self, position: usize) -> usize {
        for i in 0..3 {
            let prev = position + (i * WIDTH);
            let next = position + ((i + 1) * WIDTH);

            if self[next].is_some() {
                return prev;
            }
        }

        let bottom = position + (3 * WIDTH);
        bottom
    }
}

impl Index<usize> for Connect4 {
    type Output = Position;

    fn index(&self, index: usize) -> &Self::Output {
        &self.positions[index]
    }
}

impl IndexMut<usize> for Connect4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.positions[index]
    }
}

impl Game for Connect4 {
    type Move = usize;

    const REASONABLE_SEARCH_DEPTH: Option<usize> = Some(11);

    fn empty() -> Self {
        Self::new()
    }

    fn possible_moves(&self) -> impl Iterator<Item = Self::Move> {
        let board = self.clone();
        [3, 2, 4, 1, 5, 0, 6]
            .into_iter()
            .filter(move |col| board[*col].is_none())
    }

    fn result(&self) -> State {
        Connect4::result(&self)
    }

    fn make_move(&mut self, position: Self::Move, player: Player) {
        let pos = self.drop_player(position);
        self[pos] = Some(player);
    }

    fn undo_move(&mut self, position: Self::Move) {
        for i in 0..4 {
            let pos = position + (i * WIDTH);

            if self[pos].is_some() {
                self[pos] = None;
                return;
            }
        }
    }

    fn rate(&self, player: Player) -> Score<IgnorePlayer> {
        Connect4::rate(&self, player)
    }
}

impl Display for Connect4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..HEIGTH {
            for j in 0..WIDTH {
                let index = (i * WIDTH) + j;
                match self[index] {
                    Some(Player::X) => {
                        write!(f, "\x1B[31m  X\x1B[0m  ")?;
                    }
                    Some(Player::O) => {
                        write!(f, "\x1B[34m  O\x1B[0m  ")?;
                    }
                    None => {
                        write!(f, "\x1B[35m{index:3 }\x1B[0m  ")?;
                    }
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Player, Score, State, state::IgnorePlayer};

    use super::Connect4;

    fn parse_board(board: &str) -> Connect4 {
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

        Connect4 { positions }
    }

    fn test_result(board: &str, state: State) {
        let board = parse_board(board);
        assert_eq!(board.result(), state);
    }

    fn test_rate(board: &str, player: Player, score: Score<IgnorePlayer>) {
        let board = parse_board(board);
        assert_eq!(board.rate(player), score);
    }

    #[test]
    fn draw() {
        test_result(
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
        test_result(
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
        test_result(
            "
        XXX_OOO
        _XXX___
        X_OOO__
        OOO____
        ",
            State::InProgress,
        );
    }

    #[test]
    fn rate_alone_edge() {
        test_rate(
            "
        _______
        _______
        _______
        ______X
        ",
            Player::X,
            Score::new(3),
        )
    }

    #[test]
    fn rate_alone_center() {
        test_rate(
            "
        _______
        _______
        _______
        ___X___
        ",
            Player::X,
            Score::new(7),
        )
    }

    #[test]
    fn rate_pair_center() {
        test_rate(
            "
        _______
        _______
        _______
        O__X___
        ",
            Player::X,
            Score::new(4),
        )
    }

    #[test]
    fn rate_pair_edge() {
        test_rate(
            "
        _______
        _______
        _______
        O_____X
        ",
            Player::X,
            Score::new(0),
        )
    }
}
