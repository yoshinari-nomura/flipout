use crate::minimax;
use crate::position::Position;
use crate::ui_board::UiBoard;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

pub enum Action {
    Move(u64),
    Pass,
    GiveUp,
}

pub trait Player {
    fn action(&mut self, board: &UiBoard) -> Action;
}

pub struct HumanPlayer {
    tty: Box<dyn std::io::BufRead>,
}

pub struct RobotPlayer {}

pub struct CleverRobotPlayer {}

impl Default for RobotPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl RobotPlayer {
    pub fn new() -> Self {
        RobotPlayer {}
    }
}

impl Default for CleverRobotPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl CleverRobotPlayer {
    pub fn new() -> Self {
        CleverRobotPlayer {}
    }
}

impl HumanPlayer {
    pub fn new(tty: Box<dyn std::io::BufRead>) -> Self {
        HumanPlayer { tty }
    }
}

impl Player for RobotPlayer {
    fn action(&mut self, board: &UiBoard) -> Action {
        let vec = board.legal_moves_as_vec();

        if vec.is_empty() {
            Action::Pass
        } else {
            let rnd: usize = rand::random();
            let mov = vec[rnd % vec.len()];
            Action::Move(mov)
        }
    }
}

impl Player for CleverRobotPlayer {
    fn action(&mut self, board: &UiBoard) -> Action {
        let raw_board = board.raw_board();
        let mut moves = board.legal_moves();
        let turn = raw_board.turn;

        if moves == 0 {
            Action::Pass
        } else {
            let mut best_score = std::i32::MIN;
            let mut best_move = 0;
            let holes = raw_board.count_hole();
            let depth = if holes <= 14 { 14 } else { 5 };

            while moves != 0 {
                let mov = 1 << moves.trailing_zeros();
                let mut child = raw_board.clone();
                child.put_stone(mov);
                let score = minimax::minimax(&child, turn, depth);

                if score > best_score {
                    best_score = score;
                    best_move = mov;
                }
                moves &= !mov;
            }
            Action::Move(best_move)
        }
    }
}

impl Player for HumanPlayer {
    fn action(&mut self, _board: &UiBoard) -> Action {
        loop {
            print!("Move: ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            self.tty.read_line(&mut line).unwrap();
            line = line.trim_end().to_string();

            if &line == "pass" {
                return Action::Pass;
            } else if &line == "giveup" {
                return Action::GiveUp;
            } else if let Ok(pos) = Position::from_str(&line) {
                return Action::Move(pos.as_bitboard());
            } else {
                print!("Invalid '{}' ", &line);
            }
        }
    }
}
