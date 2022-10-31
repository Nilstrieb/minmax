use crate::{
    board::{Player, State},
    Board, GamePlayer,
};

impl Board {
    pub fn play<A: GamePlayer, B: GamePlayer>(&mut self, a: &mut A, b: &mut B) -> Option<Player> {
        let mut current_player = Player::X;

        for _ in 0..9 {
            if current_player == Player::X {
                a.next_move(self, current_player);
            } else {
                b.next_move(self, current_player);
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
