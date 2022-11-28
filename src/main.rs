#![allow(unused_imports)]

use std::{fmt::Display, time::SystemTime};

use minmax::{
    connect4::{self, board::Connect4},
    tic_tac_toe::{GreedyPlayer, HumanPlayer, PerfectPlayer, TicTacToe},
    Game, GamePlayer, Player,
};

fn main() {
    play::<connect4::HumanPlayer, connect4::HumanPlayer, _>(true);
}

#[allow(dead_code)]
fn tic_tac_toe_stats() {
    let mut results = [0, 0, 0];

    let start = SystemTime::now();

    for _ in 0..100 {
        let result = play::<PerfectPlayer, GreedyPlayer, _>(false);
        let idx = Player::as_u8(result);
        results[idx as usize] += 1;
    }

    println!("Winner counts");
    println!(" X: {}", results[0]);
    println!(" O: {}", results[1]);
    println!(" Draw: {}", results[2]);

    let time = start.elapsed().unwrap();

    println!("Completed in {}ms", time.as_millis());
}

fn play<X: GamePlayer<G>, O: GamePlayer<G>, G: Game>(print: bool) -> Option<Player> {
    let mut board = G::empty();
    let result = board.play(&mut X::default(), &mut O::default());
    if print {
        print_result(result, board);
    }
    result
}

fn print_result(result: Option<Player>, board: impl Display) {
    println!("{board}");

    match result {
        Some(winner) => {
            println!("player {winner} won!");
        }
        None => {
            println!("a draw...")
        }
    }
}
