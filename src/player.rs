use std::{
    fmt::Display,
    ops::{ControlFlow, Try},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
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
