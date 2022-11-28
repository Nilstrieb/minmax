mod board;
mod game;
mod perfect;
mod player;

pub use {board::TicTacToe, perfect::PerfectPlayer, player::*};

#[cfg(test)]
mod tests {
    use crate::{tic_tac_toe::board::TicTacToe, GamePlayer, Player};

    use super::{
        perfect::PerfectPlayer,
        player::{GreedyPlayer, RandomPlayer},
    };

    fn assert_win_ratio<X: GamePlayer<TicTacToe>, O: GamePlayer<TicTacToe>>(
        runs: u64,
        x_win_ratio: f64,
    ) {
        let mut results = [0u64, 0, 0];

        for _ in 0..runs {
            let result = TicTacToe::default_play::<X, O>();
            let idx = Player::as_u8(result);
            results[idx as usize] += 1;
        }

        let total = results.iter().copied().sum::<u64>();

        let ratio = (total as f64) / (results[0] as f64);
        println!("{ratio} >= {x_win_ratio}");
        assert!(ratio >= x_win_ratio);
    }

    #[test]
    fn perfect_always_beats_greedy() {
        assert_win_ratio::<PerfectPlayer, GreedyPlayer>(20, 1.0);
    }

    #[test]
    fn perfect_beats_random() {
        assert_win_ratio::<PerfectPlayer, RandomPlayer>(10, 0.95);
    }
}
