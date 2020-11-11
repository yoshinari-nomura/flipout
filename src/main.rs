//! flipout is a reversi (Othello) implementation in Rust
//!

use flipout::dumb_screen::*;
use flipout::player::*;
use flipout::ui_board::UiBoard;
use std::io::{self, BufReader};
use std::{env, process};

fn usage_and_exit() {
    eprint!(
        r#"flipout - a reversi (Othello) implementation in Rust.
usage: flipout [-a] [-r]
  -a: Automatic demo mode
  -r: Reverse color for dark terminals
"#
    );
    process::exit(-1);
}

fn play(mut board: UiBoard, screen: DumbScreen, mut black: impl Player, mut white: impl Player) {
    screen.update_screen(&board);

    loop {
        let action = if board.is_black_turn() {
            black.action(&board)
        } else {
            white.action(&board)
        };

        match action {
            Action::GiveUp => break,
            Action::Pass => board.pass().is_ok(),
            Action::Move(mov) => board.put_stone(mov.as_bits()).is_ok(),
        };

        screen.update_screen(&board);

        if board.is_game_over() {
            break;
        }
    }
}

fn main() {
    let mut opt_auto_demo = false;
    let mut opt_reverse_video = false;

    let tty = Box::new(BufReader::new(io::stdin()));

    for arg in env::args().skip(1) {
        match &*arg {
            "-a" => opt_auto_demo = true,
            "-r" => opt_reverse_video = true,
            _ => {
                usage_and_exit();
            }
        }
    }

    let board = UiBoard::new();
    let screen = DumbScreen::new(opt_auto_demo, opt_reverse_video);
    let white = CleverRobotPlayer::new();

    if opt_auto_demo {
        play(board, screen, RobotPlayer::new(), white);
    } else {
        play(board, screen, HumanPlayer::new(tty), white);
    }
}
