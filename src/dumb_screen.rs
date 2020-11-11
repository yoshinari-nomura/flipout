use crate::board::Turn;
use crate::position::*;
use crate::ui_board::{Color, UiBoard};

pub struct DumbScreen {
    dumb: bool,
    reverse_video: bool,
}

impl DumbScreen {
    pub fn new(dumb: bool, reverse_video: bool) -> Self {
        DumbScreen {
            dumb,
            reverse_video,
        }
    }

    pub fn update_screen(&self, board: &UiBoard) {
        if !self.dumb {
            Self::clear_screen();
        }
        self.print_board(board);
    }

    fn clear_screen() {
        print!("\x1b[2J");
        Self::locate(1, 1);
    }

    fn locate(x: u32, y: u32) {
        print!("\x1b[{};{}H", x, y);
    }

    fn print_board(&self, board: &UiBoard) {
        let black = if self.reverse_video { "○" } else { "●" };
        let white = if self.reverse_video { "●" } else { "○" };
        let turn = match board.turn() {
            Some(Turn::White) => white,
            Some(Turn::Black) => black,
            None => "Game Over",
        };

        print!("  ａｂｃｄｅｆｇｈ");

        for i in 0..64 {
            if i % 8 == 0 {
                print!("\n{} ", i / 8 + 1);
            }
            let pos = (1 << 63) >> i;
            let grid_char = match board.color_at(Position::new(pos)) {
                Color::White => white,
                Color::Black => black,
                Color::Empty => {
                    if board.is_legal_move(Position::new(pos)) {
                        "＊"
                    } else {
                        "・"
                    }
                }
            };
            print!("{}", grid_char);
        }
        println!(
            "\n{}:{} {}:{} Turn:{}",
            black,
            board.count_black(),
            white,
            board.count_white(),
            turn
        );
    }
}
