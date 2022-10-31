use std::fmt::{Display, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Winner(Player),
    InProgress,
    Draw,
}

impl Player {
    pub fn opponent(self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }

    fn from_u8(num: u8) -> Result<Option<Self>, ()> {
        Ok(match num {
            0 => Some(Player::X),
            1 => Some(Player::O),
            2 => None,
            _ => return Err(()),
        })
    }

    pub fn as_u8(this: Option<Player>) -> u8 {
        match this {
            Some(Player::X) => 0,
            Some(Player::O) => 1,
            None => 2,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::X => "X",
            Self::O => "O",
        })
    }
}

#[derive(Clone)]
pub struct Board(u32);

impl Board {
    pub fn empty() -> Self {
        // A = 1010
        // 18 bits - 9 * 2 bits - 4.5 nibbles
        Self(0x0002AAAA)
    }

    #[cfg(not(debug_assertions))]
    fn validate(&self) {}

    #[cfg(debug_assertions)]
    fn validate(&self) {
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

    pub fn get(&self, index: usize) -> Option<Player> {
        self.validate();
        debug_assert!(index < 9);

        let board = self.0;

        let shifted = board >> (index * 2);
        let masked = shifted & 0b11;

        Player::from_u8(masked as u8).unwrap()
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
            let result = (i < 8).then(|| this.get(i));
            i += 1;
            result
        })
    }

    pub fn result(&self) -> State {
        win_table::result(self)
    }
}

mod win_table {
    use super::{Board, Player, State};

    const WIN_TABLE_SIZE: usize = 2usize.pow(2 * 9);
    const WIN_TABLE: &[u8; WIN_TABLE_SIZE] = include_bytes!(concat!(env!("OUT_DIR"), "/win_table"));

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
