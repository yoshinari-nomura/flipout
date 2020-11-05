use crate::board::{Board, Turn};
use std::fmt;

pub type Move = u64;
pub type Moves = u64;
pub type BitBoard = u64;

pub enum Color {
    Black,
    White,
    Empty,
}

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

    ////////////////////////////////////////////////////////////////
    // Mutable functions

    pub fn set_reverse_video(&mut self) {
        self.reverse_video = true;
    }

    pub fn put_stone(&mut self, mov: Move) -> Result<&Self, &str> {
        if self.board.is_legal_move(mov) {
            self.board.put_stone(mov);
            Ok(self)
        } else {
            Err("Invalid move")
        }
    }

    pub fn pass(&mut self) -> Result<&Self, &str> {
        if self.board.legal_moves() == 0 {
            self.board.pass();
            Ok(self)
        } else {
            Err("Cannot pass")
        }
    }

    ////////////////////////////////////////////////////////////////
    // Expose primitive interface

    pub fn raw_board(&self) -> &Board {
        &self.board
    }

    ////////////////////////////////////////////////////////////////
    // Count and examine stones

    pub fn count_black(&self) -> u32 {
        self.board.count_black()
    }

    pub fn count_white(&self) -> u32 {
        self.board.count_white()
    }

    pub fn count_hole(&self) -> u32 {
        self.board.count_hole()
    }

    pub fn color_at(&self, pos: u64) -> Color {
        if self.is_black_at(pos) {
            Color::Black
        } else if self.is_white_at(pos) {
            Color::White
        } else {
            Color::Empty
        }
    }

    ////////////////////////////////////////////////////////////////
    // Game rules

    pub fn is_game_over(&self) -> bool {
        self.board.is_game_over()
    }

    pub fn is_legal_move(&self, mov: u64) -> bool {
        self.board.is_legal_move(mov)
    }

    pub fn legal_moves(&self) -> Moves {
        self.board.legal_moves()
    }

    pub fn legal_moves_as_vec(&self) -> Vec<Move> {
        self.board.legal_moves_as_vec()
    }

    pub fn reversible_stones(&self, mov: Move) -> BitBoard {
        self.board.reversible_stones(mov)
    }

    ////////////////////////////////////////////////////////////////
    // Current status

    pub fn turn(&self) -> Turn {
        self.board.turn
    }

    pub fn is_black_turn(&self) -> bool {
        self.board.is_black_turn()
    }

    ////////////////////////////////////////////////////////////////
    // Private

    fn is_black_at(&self, pos: u64) -> bool {
        self.board.black & pos != 0
    }

    fn is_white_at(&self, pos: u64) -> bool {
        self.board.white & pos != 0
    }
}

impl fmt::Display for UiBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let black = if self.reverse_video { "○" } else { "●" };
        let white = if self.reverse_video { "●" } else { "○" };
        let turn = if self.is_black_turn() { black } else { white };

        write!(f, "  ＡＢＣＤＥＦＧＨ")?;

        for i in 0..64 {
            if i % 8 == 0 {
                write!(f, "\n{} ", i / 8 + 1)?;
            }
            let pos = (1 << 63) >> i;
            let grid_char = match self.color_at(pos) {
                Color::White => white,
                Color::Black => black,
                Color::Empty => {
                    if self.is_legal_move(pos) {
                        "＊"
                    } else {
                        "・"
                    }
                }
            };
            write!(f, "{}", grid_char)?;
        }
        writeln!(
            f,
            "\n{}:{} {}:{} Turn:{}",
            black,
            self.count_black(),
            white,
            self.count_white(),
            turn
        )?;
        Ok(())
    }
}
