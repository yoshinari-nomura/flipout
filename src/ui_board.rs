use crate::board::{Board, Turn};
use crate::position::*;
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
    whatnow: Option<Turn>,
}

impl Default for UiBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl UiBoard {
    pub fn new() -> Self {
        let board = Board::new();
        UiBoard {
            board,
            whatnow: Some(Turn::Black),
        }
    }

    ////////////////////////////////////////////////////////////////
    // Mutable functions

    pub fn put_stone(&mut self, pos: Position) -> Result<&Self, &str> {
        if self.is_game_over() {
            return Err("Game over");
        }
        if self.is_legal_move(pos) {
            self.board.put_stone(pos.as_bits());
            self.update_satus();
            Ok(self)
        } else {
            Err("Invalid move")
        }
    }

    pub fn pass(&mut self) -> Result<&Self, &str> {
        if self.is_game_over() {
            return Err("Game over");
        }
        if self.legal_moves() == 0 {
            self.board.pass();
            self.update_satus();
            Ok(self)
        } else {
            Err("Can't pass")
        }
    }

    fn update_satus(&mut self) {
        if self.is_game_over() {
            self.whatnow = None;
            return;
        }
        if self.legal_moves() == 0 {
            self.board.pass();
        }
        self.whatnow = self.turn();
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

    pub fn color_at(&self, pos: Position) -> Color {
        if self.is_black_at(pos.as_bits()) {
            Color::Black
        } else if self.is_white_at(pos.as_bits()) {
            Color::White
        } else {
            Color::Empty
        }
    }

    ////////////////////////////////////////////////////////////////
    // Game rules

    pub fn whatnow(&self) -> Option<Turn> {
        self.whatnow
    }

    pub fn is_game_over(&self) -> bool {
        self.board.is_game_over()
    }

    pub fn is_legal_move(&self, pos: Position) -> bool {
        self.board.is_legal_move(pos.as_bits())
    }

    fn legal_moves(&self) -> Moves {
        self.board.legal_moves()
    }

    pub fn legal_moves_as_vec(&self) -> Vec<Move> {
        self.board.legal_moves_as_vec()
    }

    pub fn reversible_stones(&self, pos: Position) -> Positions {
        Positions::new(self.board.reversible_stones(pos.as_bits()))
    }

    ////////////////////////////////////////////////////////////////
    // Current status

    pub fn turn(&self) -> Option<Turn> {
        if self.is_game_over() {
            None
        } else {
            Some(self.board.turn)
        }
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
        let (black, white) = ("●", "○");
        let turn = match self.turn() {
            Some(Turn::White) => white,
            Some(Turn::Black) => black,
            None => "None",
        };

        write!(f, "  ａｂｃｄｅｆｇｈ")?;

        for i in 0..64 {
            if i % 8 == 0 {
                write!(f, "\n{} ", i / 8 + 1)?;
            }
            let pos = (1 << 63) >> i;
            let grid_char = match self.color_at(Position::new(pos)) {
                Color::White => white,
                Color::Black => black,
                Color::Empty => {
                    if self.is_legal_move(Position::new(pos)) {
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
