#![allow(unused_imports)]

use std::time::SystemTime;

use minmax::{Board, GamePlayer, GreedyPlayer, HumanPlayer, PerfectPlayer, Player, RandomPlayer};

fn main() {
    let mut results = [0, 0, 0];

    let start = SystemTime::now();

    for _ in 0..10_000 {
        let result = play_round(PerfectPlayer::new(), PerfectPlayer::new(), false);
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

fn play_round<A: GamePlayer, B: GamePlayer>(mut a: A, mut b: B, print: bool) -> Option<Player> {
    let mut board = Board::empty();
    let result = board.play(&mut a, &mut b);
    if print {
        println!("{board}");
    }
    match result {
        Some(winner) => {
            if print {
                println!("player {winner} won!");
            }
        }
        None => {
            if print {
                println!("a draw...")
            }
        }
    }
    result
}
