use crate::board::{Board, Turn};
use std::fmt;

pub type Move = u64;

pub struct UiBoard {
    board: Board,
    reverse_video: bool,
}

impl UiBoard {
    pub fn new(reverse_video: bool) -> Self {
        let board = Board::new();
        UiBoard {
            board,
            reverse_video,
        }
    }

    pub fn set_reverse_video(&mut self) {
        self.reverse_video = true;
    }

    pub fn put_stone(&mut self, mov: Move) -> Result<&mut Self, &str> {
        if self.board.is_legal_move(mov) {
            self.board.put_stone(mov);
            Ok(self)
        } else {
            Err("Invalid move")
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn turn(&self) -> Turn {
        self.board.turn
    }

    pub fn pass(&mut self) -> Result<&mut Self, &str> {
        if self.board.legal_moves() == 0 {
            self.board.pass();
            Ok(self)
        } else {
            Err("Cannot pass")
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.board.is_game_over()
    }

    pub fn is_black_turn(&self) -> bool {
        self.board.is_black_turn()
    }
}

impl fmt::Display for UiBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut black = self.board.black;
        let mut white = self.board.white;
        let mut hint = self.board.legal_moves();
        let black_char = if self.reverse_video { "○" } else { "●" };
        let white_char = if self.reverse_video { "●" } else { "○" };

        let mask: u64 = 1 << 63;

        writeln!(f, "  ａｂｃｄｅｆｇｈ")?;

        for row in 0..8 {
            write!(f, "{} ", row + 1)?;

            for _col in 0..8 {
                let stone = if black & mask != 0 {
                    black_char
                } else if white & mask != 0 {
                    white_char
                } else if hint & mask != 0 {
                    "＊"
                } else {
                    "・"
                };
                write!(f, "{}", stone)?;
                black <<= 1;
                white <<= 1;
                hint <<= 1;
            }
            writeln!(f)?;
        }
        writeln!(
            f,
            "Black: {} White: {}",
            self.board.count_black(),
            self.board.count_white()
        )?;
        Ok(())
    }
}
