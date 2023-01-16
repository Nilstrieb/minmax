use crate::{Game, GamePlayer, Player};
use rand::Rng;

#[derive(Clone, Default)]
pub struct GreedyPlayer;

impl<G: Game> GamePlayer<G> for GreedyPlayer {
    fn next_move(&mut self, board: &mut G, this_player: Player) {
        let first_free = board.possible_moves().next().expect("cannot make move");
        board.make_move(first_free, this_player);
    }
}

#[derive(Clone, Default)]
pub struct RandomPlayer;

impl<G: Game> GamePlayer<G> for RandomPlayer {
    fn next_move(&mut self, board: &mut G, this_player: Player) {
        let moves = board.possible_moves().collect::<Vec<_>>();

        let selected = rand::thread_rng().gen_range(0..moves.len());
        board.make_move(moves[selected], this_player);
    }
}
