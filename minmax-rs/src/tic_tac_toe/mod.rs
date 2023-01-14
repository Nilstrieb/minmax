mod board;
mod game;
mod player;

pub use {board::TicTacToe, player::*};

#[cfg(test)]
mod tests {
    use crate::{minmax::PerfectPlayer, tic_tac_toe::board::TicTacToe, GamePlayer, Player};

    use crate::player::{GreedyPlayer, RandomPlayer};

    fn assert_win_ratio<X: GamePlayer<TicTacToe>, O: GamePlayer<TicTacToe>>(
        runs: u64,
        x_win_ratio: f64,
        x: impl Fn() -> X,
        o: impl Fn() -> O,
    ) {
        let mut results = [0u64, 0, 0];

        for _ in 0..runs {
            let result = TicTacToe::empty().play::<X, O>(&mut x(), &mut o());
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
        assert_win_ratio(1, 1.0, || PerfectPlayer::new(false), || GreedyPlayer);
    }

    #[test]
    fn perfect_beats_random() {
        assert_win_ratio(10, 0.95, || PerfectPlayer::new(false), || RandomPlayer);
    }
}
