use std::fmt::{Display, Write};

use crate::{Player, State};

#[derive(Clone)]
pub struct Board(u32);

impl Board {
    pub fn empty() -> Self {
        // A = 1010
        // 18 bits - 9 * 2 bits - 4.5 nibbles
        Self(0x0002AAAA)
    }

    fn validate(&self) {
        if cfg!(debug_assertions) {
            let board = self.0;
            for i in 0..16 {
                let next_step = board >> (i * 2);
                let mask = 0b11;
                let pos = next_step & mask;
                if pos >= 3 {
                    panic!("Invalid bits, self: {board:0X}, bits: {pos:0X}");
                }
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<Player> {
        debug_assert!(index < 9);

        let board = self.0;

        let shifted = board >> (index * 2);
        let masked = shifted & 0b11;

        // SAFETY: So uh, this is a bit unlucky.
        // You see, there are two entire bits of information at our disposal for each position.
        // This is really bad. We only have three valid states. So we need to do _something_ if it's invalid.
        // We just hope that it will never be invalid which it really shouldn't be and also have a debug assertion
        // here to make sure that it really is valid and then if it's not invalid we just mov it out and are happy.
        self.validate();
        unsafe { Player::from_u8(masked as u8).unwrap_unchecked() }
    }

    pub fn set(&mut self, index: usize, value: Option<Player>) {
        debug_assert!(index < 9);
        self.validate();

        let value = Player::as_u8(value) as u32;

        let value = value << (index * 2);
        let mask = 0b11 << (index * 2);

        let current_masked_off_new = self.0 & !mask;
        let result = value | current_masked_off_new;
        self.0 = result;

        self.validate();
    }

    pub fn iter(&self) -> impl Iterator<Item = Option<Player>> {
        let mut i = 0;
        let this = self.clone();
        std::iter::from_fn(move || {
            let result = (i < 9).then(|| this.get(i));
            i += 1;
            result
        })
    }

    pub fn result(&self) -> State {
        win_table::result(self)
    }
}

mod win_table {
    use super::Board;
    use crate::{Player, State};

    const WIN_TABLE_SIZE: usize = 2usize.pow(2 * 9);
    static WIN_TABLE: &[u8; WIN_TABLE_SIZE] =
        include_bytes!(concat!(env!("OUT_DIR"), "/win_table"));

    pub fn result(board: &Board) -> State {
        match WIN_TABLE[board.0 as usize] {
            0 => State::Winner(Player::X),
            1 => State::Winner(Player::X),
            2 => State::InProgress,
            3 => State::Draw,
            n => panic!("Invalid value {n} in table"),
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..3 {
            for j in 0..3 {
                let index = i * 3 + j;
                match self.get(index) {
                    Some(player) => {
                        write!(f, "\x1B[33m{player}\x1B[0m  ")?;
                    }
                    None => {
                        write!(f, "\x1B[35m{index}\x1B[0m  ")?;
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
    use super::{Board, Player};

    #[test]
    fn board_field() {
        let mut board = Board::empty();
        board.set(0, None);
        board.set(8, Some(Player::X));
        board.set(4, Some(Player::O));
        board.set(5, Some(Player::X));

        let expected = [
            None,
            None,
            None,
            None,
            Some(Player::O),
            Some(Player::X),
            None,
            None,
            Some(Player::X),
        ];

        board
            .iter()
            .zip(expected.into_iter())
            .enumerate()
            .for_each(|(idx, (actual, expected))| assert_eq!(actual, expected, "Position {idx}"));
    }
}