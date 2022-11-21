use crate::{Board, GamePlayer, Player, State};

impl Board {
    pub fn default_play<X: GamePlayer, O: GamePlayer>() -> Option<Player> {
        Self::empty().play(&mut X::default(), &mut O::default())
    }

    pub fn play<A: GamePlayer, B: GamePlayer>(&mut self, x: &mut A, o: &mut B) -> Option<Player> {
        let mut current_player = Player::X;

        for _ in 0..9 {
            if current_player == Player::X {
                x.next_move(self, current_player);
            } else {
                o.next_move(self, current_player);
            }

            match self.result() {
                State::Winner(player) => return Some(player),
                State::Draw => {
                    return None;
                }
                State::InProgress => {}
            }

            current_player = current_player.opponent();
        }

        None
    }
}
