use crate::{GamePlayer, Player, State};

use super::TicTacToe;

impl TicTacToe {
    pub fn play<A: GamePlayer<TicTacToe>, B: GamePlayer<TicTacToe>>(
        &mut self,
        x: &mut A,
        o: &mut B,
    ) -> Option<Player> {
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
