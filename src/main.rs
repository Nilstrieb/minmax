#![feature(let_chains)]

use std::{fmt::Display, str::FromStr, time::SystemTime};

use clap::{Parser, ValueEnum};
use minmax::{
    connect4::{self, board::Connect4},
    tic_tac_toe::{self, TicTacToe},
    Game, GamePlayer, PerfectPlayer, Player,
};

#[derive(Debug, Clone)]
enum PlayerConfig {
    Human,
    Perfect { depth: Option<usize> },
}

impl FromStr for PlayerConfig {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");
        let mut player = match parts
            .next()
            .ok_or_else(|| "No player name provided".to_owned())?
        {
            "human" | "h" => Self::Human,
            "perfect" | "p" | "ai" | "minmax" => Self::Perfect { depth: None },
            string => {
                return Err(format!(
                    "Invalid player: {string}. Available players: human,perfect"
                ))
            }
        };

        if let Some(depth) = parts.next()
            && let Self::Perfect { depth: player_depth } = &mut player
        {
            match depth.parse() {
                Ok(depth) => *player_depth = Some(depth),
                Err(err) => return Err(format!("Invalid depth: {depth}. {err}")),
            }
        }

        Ok(player)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum GameType {
    TicTacToe,
    Connect4,
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    game: GameType,
    #[arg(short)]
    x: PlayerConfig,
    #[arg(short)]
    o: PlayerConfig,
    #[arg(long)]
    no_print_time: bool,
}

fn main() {
    let args = Args::parse();

    match args.game {
        GameType::Connect4 => {
            let get_player = |player| -> Box<dyn GamePlayer<Connect4>> {
                match player {
                    PlayerConfig::Human => Box::new(connect4::HumanPlayer),
                    PlayerConfig::Perfect { depth } => {
                        Box::new(PerfectPlayer::new(!args.no_print_time).with_max_depth(depth))
                    }
                }
            };

            let player_a = get_player(args.o);
            let player_b = get_player(args.x);

            play_with_players(player_a, player_b);
        }
        GameType::TicTacToe => {
            let get_player = |player| -> Box<dyn GamePlayer<TicTacToe>> {
                match player {
                    PlayerConfig::Human => Box::new(tic_tac_toe::HumanPlayer),
                    PlayerConfig::Perfect { depth } => {
                        Box::new(PerfectPlayer::new(!args.no_print_time).with_max_depth(depth))
                    }
                }
            };

            let player_a = get_player(args.o);
            let player_b = get_player(args.x);

            play_with_players(player_a, player_b);
        }
    }
}

#[allow(dead_code)]
fn tic_tac_toe_stats() {
    let mut results = [0, 0, 0];

    let start = SystemTime::now();

    for _ in 0..100 {
        let result = play::<PerfectPlayer<TicTacToe>, tic_tac_toe::GreedyPlayer, _>(false);
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

fn play_with_players<G: Game, X: GamePlayer<G>, O: GamePlayer<G>>(mut x: X, mut o: O) {
    let mut board = G::empty();
    let result = board.play(&mut x, &mut o);

    print_result(result, board);
}

fn play<X: GamePlayer<G> + Default, O: GamePlayer<G> + Default, G: Game>(
    print: bool,
) -> Option<Player> {
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
