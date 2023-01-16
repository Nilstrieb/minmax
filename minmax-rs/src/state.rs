use std::{
    fmt::Display,
    ops::{ControlFlow, Try},
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
