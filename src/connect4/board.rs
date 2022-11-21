use std::ops::ControlFlow;

use crate::Player;

type Position = Option<Player>;

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 4;
const BOARD_POSITIONS: usize = BOARD_WIDTH * BOARD_HEIGHT;

struct Winner<T>(Option<T>);

impl<T> std::ops::Try for Winner<T> {
    // Always None
    type Output = Winner<!>;

    type Residual = Winner<T>;

    fn from_output(_: Self::Output) -> Self {
        Self(None)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self(Some(_)) => ControlFlow::Break(self),
            Self(None) => ControlFlow::Continue(self),
        }
    }
}

struct Board {
    positions: [Position; BOARD_POSITIONS],
}

impl Board {
    fn winner() -> Winner<Player> {
        todo!()
    }
}
