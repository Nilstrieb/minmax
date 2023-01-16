use std::{
    fmt::{Display, Debug},
    ops::{ControlFlow, Try, Neg},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X = 1,
    O = 16,
}

pub type Position = Option<Player>;

pub fn position_as_int(pos: Position) -> u8 {
    // we make sure that this is branchless by setting the repr of Player
    match pos {
        None => 0,
        Some(Player::X) => 1,
        Some(Player::O) => 16,
    }
}

impl PartialEq<Option<Player>> for Player {
    fn eq(&self, other: &Option<Player>) -> bool {
        match (self, other) {
            (Player::X, Some(Player::X)) => true,
            (Player::O, Some(Player::O)) => true,
            _ => false,
        }
    }
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

    pub fn from_u8(num: u8) -> Result<Option<Self>, ()> {
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

impl std::ops::FromResidual for State {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        residual
    }
}

impl Try for State {
    // InProgress
    type Output = Self;

    type Residual = Self;

    fn from_output(_: Self::Output) -> Self {
        Self::InProgress
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::InProgress => ControlFlow::Continue(self),
            Self::Winner(_) | Self::Draw => ControlFlow::Break(self),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score(pub i32);

impl Score {
    // Due to the nature of two's completement, we can't actually negate this properly, so add 1.
    pub const LOST: Self = Self(i32::MIN + 1);
    pub const TIE: Self = Self(0);
    pub const WON: Self = Self(i32::MAX);

    pub fn new(int: i32) -> Self {
        Self(int)
    }

    #[allow(unused)]
    fn randomize(self) -> Self {
        let score = self.0 as f32;
        let rand = rand::thread_rng();
        self
    }
}

impl Neg for Score {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Debug for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::WON => f.write_str("WON"),
            Self::LOST => f.write_str("LOST"),
            Self(other) => Debug::fmt(&other, f),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use crate::Player;

    use super::Position;

    #[test]
    fn position_size_and_repr() {
        assert_eq!(mem::size_of::<Position>(), 1);
        // this is uh maybe a tiny little not fully sound but pretty much sound
        unsafe {
            assert_eq!(0, mem::transmute::<Position, u8>(Position::None));
            assert_eq!(1, mem::transmute::<Position, u8>(Position::Some(Player::X)));
            assert_eq!(
                16,
                mem::transmute::<Position, u8>(Position::Some(Player::O))
            );
        }
    }
}
