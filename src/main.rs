use minmax::{Board, GreedyPlayer};

fn main() {
    let mut board = Board::empty();
    let result = board.play(&mut GreedyPlayer, &mut GreedyPlayer);
    println!("{board}");
    match result {
        Some(winner) => {
            println!("player {winner} won!");
        }
        None => println!("a draw..."),
    }
}
