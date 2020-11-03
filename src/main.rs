//! flipout is a reversi (Othello) implementation in Rust
//!

use flipout::player::*;
use flipout::position::Position;
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

struct Config {
    pub auto_demo: bool,
    pub reverse_video: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            auto_demo: false,
            reverse_video: false,
        }
    }
}

fn play(mut board: UiBoard, mut black: Box<dyn Player>, mut white: Box<dyn Player>) {
    loop {
        let player = if board.is_black_turn() {
            &mut black
        } else {
            &mut white
        };

        let action = player.action(&board);

        print!("{}", board);
        print!(
            "Player {}: ",
            if board.is_black_turn() {
                "black"
            } else {
                "white"
            },
        );

        match action {
            Action::GiveUp => break,
            Action::Pass => {
                if board.pass().is_ok() {
                    println!("pass");
                }
            }
            Action::Move(mov) => {
                println!("move {}", Position::from_u64(mov).unwrap());
                if board.put_stone(mov).is_ok() {}
            }
        }

        // player.update(&board);

        if board.is_game_over() {
            break;
        }
    }
    print!("{}", board);
}

fn main() {
    let mut cnf = Config::new();
    let tty = Box::new(BufReader::new(io::stdin()));

    for arg in env::args().skip(1) {
        match &*arg {
            "-a" => cnf.auto_demo = true,
            "-r" => cnf.reverse_video = true,
            _ => {
                usage_and_exit();
            }
        }
    }

    let board = UiBoard::new(cnf.reverse_video);

    let black: Box<dyn Player> = if cnf.auto_demo {
        Box::new(RobotPlayer::new())
    } else {
        Box::new(HumanPlayer::new(tty))
    };
    let white: Box<dyn Player> = Box::new(CleverRobotPlayer::new());

    play(board, black, white);
}
