use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::{ControlFlow, Neg, Try},
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

// This fun generic setup ensures that we never compare two scores from different layers.
pub struct Score<P>(pub i32, PhantomData<P>);

pub trait MinmaxPlayer {
    type Enemy: MinmaxPlayer<Enemy = Self>;
}
pub struct GoodPlayer;
pub struct EvilPlayer;
pub struct IgnorePlayer;

impl MinmaxPlayer for GoodPlayer {
    type Enemy = EvilPlayer;
}
impl MinmaxPlayer for EvilPlayer {
    type Enemy = GoodPlayer;
}
impl MinmaxPlayer for IgnorePlayer {
    type Enemy = Self;
}

impl Score<IgnorePlayer> {
    // Due to the nature of two's completement, we can't actually negate this properly, so add 1.
    pub const LOST: Self = Self(i32::MIN + 1, PhantomData);
    pub const TIE: Self = Self(0, PhantomData);
    pub const WON: Self = Self(i32::MAX, PhantomData);

    pub fn for_player<P>(self) -> Score<P> {
        Score(self.0, PhantomData)
    }
}

impl<P> Score<P> {
    pub fn new(int: i32) -> Self {
        Self(int, PhantomData)
    }

    pub fn ignore_side(self) -> Score<IgnorePlayer> {
        Score(self.0, PhantomData)
    }

    #[allow(unused)]
    fn randomize(self) -> Self {
        let score = self.0 as f32;
        let rand = rand::thread_rng();
        self
    }
}

impl<P: MinmaxPlayer> Neg for Score<P> {
    type Output = Score<P::Enemy>;

    fn neg(self) -> Self::Output {
        Score(-self.0, PhantomData)
    }
}

impl<P> PartialEq for Score<P> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<P> Eq for Score<P> {}

impl<P> PartialOrd for Score<P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<P> Ord for Score<P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<P> Clone for Score<P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P> Copy for Score<P> {}

impl<P> Debug for Score<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // this should be a match but StructuralEq does not like me
        if self.ignore_side() == Score::WON {
            f.write_str("WON")
        } else if Score::LOST == self.ignore_side() {
            f.write_str("LOST")
        } else {
            Debug::fmt(&self.0, f)
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
