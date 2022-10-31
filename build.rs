use std::{fs::File, io::Write, path::PathBuf};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Player {
    X,
    O,
}

impl Player {
    fn from_u8(num: u8) -> Option<Self> {
        match num {
            0 => Some(Player::X),
            1 => Some(Player::O),
            2 => None,
            _ => panic!("Invalid value {num}"),
        }
    }

    fn as_u8(this: Option<Player>) -> u8 {
        match this {
            Some(Player::X) => 0,
            Some(Player::O) => 1,
            None => 2,
        }
    }
}

#[derive(Clone, Copy)]
struct Board(u32);

impl Board {
    fn new(num: u32) -> Option<Board> {
        for i in 0..16 {
            let next_step = num >> (i * 2);
            let mask = 0b11;
            let pos = next_step & mask;
            if pos == 3 {
                return None;
            }
        }

        Some(Self(num))
    }

    fn get(&self, index: usize) -> Option<Player> {
        debug_assert!(index < 9);

        let shifted = self.0 >> (index * 2);
        let masked = shifted & 0b11;
        Player::from_u8(masked as u8)
    }
}

fn winner(board: Board) -> Option<Player> {
    fn won_row(a: Option<Player>, b: Option<Player>, c: Option<Player>) -> Option<Player> {
        if a == Some(Player::X) && b == Some(Player::X) && c == Some(Player::X) {
            Some(Player::X)
        } else if a == Some(Player::O) && b == Some(Player::O) && c == Some(Player::O) {
            Some(Player::O)
        } else {
            None
        }
    }

    macro_rules! test_row {
        ($a:literal, $b:literal, $c:literal) => {
            match won_row(board.get($a), board.get($b), board.get($c)) {
                Some(player) => return Some(player),
                None => {}
            }
        };
    }

    test_row!(0, 1, 2);
    test_row!(3, 4, 5);
    test_row!(6, 7, 8);

    test_row!(0, 3, 6);
    test_row!(1, 4, 7);
    test_row!(2, 5, 8);

    test_row!(0, 4, 8);
    test_row!(2, 4, 6);
    None
}

fn calculate_win_table(file: &mut impl Write) {
    for board in 0..(2u32.pow(18)) {
        let byte = match Board::new(board) {
            Some(board) => {
                let winner = winner(board);
                Player::as_u8(winner)
            }
            None => 0,
        };
        file.write_all(&[byte]).expect("write file");
    }
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
    let win_table_path = PathBuf::from(out_dir).join("win_table");
    let mut win_table_file = File::create(win_table_path).expect("create win table file");

    calculate_win_table(&mut win_table_file);

    win_table_file.flush().expect("flushing file");
}
