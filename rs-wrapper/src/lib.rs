use jni::objects::{JClass, JObject, ReleaseMode};
use jni::sys::{jbyte, jint};
use jni::JNIEnv;
use minmax::{connect4::board::Connect4, GamePlayer};
use minmax::{Game, PerfectPlayer, Player};

/// We need to map the board.
/// Rust:
/// ```text
///  0  1  2  3  4  5  6
///  7  8  9 10 11 12 13
/// 14 15 16 17 18 19 20
/// 21 22 23 24 25 26 27
/// ```
/// Java:
/// ```text
/// 21 22 23 24 25 26 27
/// 14 15 16 17 18 19 20
///  7  8  9 10 11 12 13
///  0  1  2  3  4  5  6
/// ```
fn map_idx(i: usize) -> usize {
    match () {
        () if i < 7 => i + 21,
        () if i < 14 => i + 7,
        () if i < 21 => i - 7,
        () => i - 21,
    }
}

fn crate_board(java_board: &[i8]) -> Connect4 {
    let mut board = Connect4::new();

    for i in 0..28 {
        let java_int = java_board[i];
        let rust_value = match java_int {
            0 => {
                dbg!("x player {i}", i, map_idx(i));
                Some(Player::X)
            },
            1 => Some(Player::O),
            2 => None,
            _ => unreachable!(),
        };

        let rust_index = map_idx(i);

        board.set_pos(rust_index, rust_value);
    }

    board
}

// 0 -> BLUE -> X
// 1 -> RED -> O
// 2 -> empty
pub fn wrap_player(env: JNIEnv<'_>, current_player: i8, board: JObject<'_>) -> i32 {
    let board_size = env.get_array_length(board.into_raw()).unwrap();
    assert_eq!(board_size, 28);

    let byte_array = env
        .get_byte_array_elements(board.into_raw(), ReleaseMode::NoCopyBack)
        .unwrap();

    let slice = unsafe { std::slice::from_raw_parts(byte_array.as_ptr() as *const _, 28) };

    dbg!(slice);

    let mut board = crate_board(slice);

    println!("{board}");

    let mut player = PerfectPlayer::new(false);

    let current_player_rust = match current_player {
        0 => Player::X,
        1 => Player::O,
        _ => unreachable!(),
    };

    player.next_move(&mut board, current_player_rust);
    let result_move = player.best_move();
    board.undo_move(result_move);

    let result_move = board.drop_player(result_move);

    let java_idx = map_idx(result_move) as i32;

    dbg!(result_move, java_idx);

    java_idx
}

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_ch_bbw_m411_connect4_RustPlayer_rustPlay(
    env: JNIEnv<'_>,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    _: JClass<'_>,
    player: jbyte,
    board: JObject<'_>,
) -> jint {
    std::panic::catch_unwind(|| wrap_player(env, player, board))
        .unwrap_or_else(|_| std::process::abort())
}
