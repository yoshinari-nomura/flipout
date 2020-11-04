//! Structures for expressing state of game.
//!
//! Board is used to store:
//! * positions of black and white stones
//! * the current player
//!
//! and is used as a node of the game search tree, it is necessary to
//! keep it compact.
//!

use crate::bitboard::{self, BitBoard, Move, Moves};
use std::fmt;
use wasm_bindgen::prelude::*;

/// Used to express the current player.
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Turn {
    Black,
    White,
}

impl Turn {
    pub fn is_black(&self) -> bool {
        match *self {
            Turn::Black => true,
            Turn::White => false,
        }
    }

    pub fn opposit(&self) -> Turn {
        match *self {
            Turn::Black => Turn::White,
            Turn::White => Turn::Black,
        }
    }
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == Turn::Black {
            write!(f, "black")?;
        } else {
            write!(f, "white")?;
        }
        Ok(())
    }
}

/// Board is used to store:
/// * positions of black and white stones
/// * the current player
///
/// and is used as a node of the game search tree, it is necessary to
/// keep it compact.
#[derive(Clone, PartialEq)]
pub struct Board {
    pub black: BitBoard,
    pub white: BitBoard,
    pub turn: Turn,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    /// Create new reversi board
    pub fn new() -> Self {
        Board {
            black: 0x00_00_00_08_10_00_00_00,
            white: 0x00_00_00_10_08_00_00_00,
            turn: Turn::Black,
        }
    }

    pub fn count_black(&self) -> u32 {
        self.black.count_ones()
    }

    pub fn count_white(&self) -> u32 {
        self.white.count_ones()
    }

    pub fn count_stone(&self) -> u32 {
        (self.black | self.white).count_ones()
    }

    pub fn count_hole(&self) -> u32 {
        (self.black | self.white).count_zeros()
    }

    pub fn reversible_stones(&self, mov: Move) -> BitBoard {
        if self.is_black_turn() {
            bitboard::reversible_stones(self.black, self.white, mov)
        } else {
            bitboard::reversible_stones(self.white, self.black, mov)
        }
    }

    pub fn count_reversible_stones(&self, mov: Move) -> u32 {
        self.reversible_stones(mov).count_ones()
    }

    pub fn pass(&mut self) -> &Self {
        self.turn = self.turn.opposit();
        self
    }

    pub fn put_stone(&mut self, mov: Move) -> &Self {
        if self.is_black_turn() {
            let rev = bitboard::reversible_stones(self.black, self.white, mov);
            self.black ^= mov | rev;
            self.white ^= rev;
            self.turn = Turn::White;
            self
        } else {
            let rev = bitboard::reversible_stones(self.white, self.black, mov);
            self.white ^= mov | rev;
            self.black ^= rev;
            self.turn = Turn::Black;
            self
        }
    }

    pub fn legal_moves(&self) -> Moves {
        self.legal_moves_for_player(self.turn)
    }

    fn legal_moves_for_player(&self, turn: Turn) -> Moves {
        if turn.is_black() {
            bitboard::legal_moves(self.black, self.white)
        } else {
            bitboard::legal_moves(self.white, self.black)
        }
    }

    pub fn legal_moves_as_vec(&self) -> Vec<Move> {
        let mut moves = self.legal_moves();
        let mut vec = Vec::new();

        while moves != 0 {
            let mov = 1 << moves.trailing_zeros();
            vec.push(mov);
            moves &= !mov;
        }
        vec
    }

    pub fn is_black_turn(&self) -> bool {
        self.turn == Turn::Black
    }

    pub fn is_legal_move(&self, mov: Move) -> bool {
        self.count_reversible_stones(mov) > 0
    }

    pub fn is_game_over(&self) -> bool {
        let filled = self.count_hole() == 0;
        let no_black = self.legal_moves_for_player(Turn::Black) == 0;
        let no_white = self.legal_moves_for_player(Turn::White) == 0;
        filled || (no_black && no_white)
    }

    pub fn children(&self) -> Vec<Self> {
        let mut vec = Vec::new();
        let mut moves = self.legal_moves();

        if moves == 0 {
            if self.count_hole() == 0 || self.legal_moves_for_player(self.turn.opposit()) == 0 {
                // no child due to game over
            } else {
                // pass
                let mut child = self.clone();
                child.pass();
                vec.push(child);
            }
            return vec;
        }

        while moves != 0 {
            let mov = 1 << moves.trailing_zeros();
            let mut child = self.clone();

            child.put_stone(mov);
            vec.push(child);
            moves &= !mov;
        }
        vec
    }

    pub fn eval_score(&self, turn: Turn) -> i32 {
        self.eval_score_single(turn) - self.eval_score_single(turn.opposit())
    }

    #[allow(clippy::many_single_char_names)]
    fn eval_score_single(&self, turn: Turn) -> i32 {
        let mut bitboard = if turn.is_black() {
            self.black
        } else {
            self.white
        };

        let holes = self.count_hole();

        if holes == 0 {
            return bitboard.count_ones() as i32;
        }

        let mut score_table: [i32; 64] = if holes == 0 {
            [1; 64]
        } else {
            let (o, b, c, n, a, x) = (0, -1, -3, -12, -15, 30);
            [
                x, n, o, o, o, o, n, x, // 00..07
                n, a, c, c, c, c, a, n, // 08..15
                o, c, o, o, o, o, c, o, // 16..23
                b, c, b, b, b, o, o, b, // 24..31
                b, c, o, o, o, o, c, b, // 32..39
                o, c, o, o, o, o, c, o, // 40..47
                n, a, c, c, c, c, a, n, // 48..55
                x, n, o, o, o, o, n, x, // 56..63
            ]
        };

        if bitboard & 0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000 != 0
        {
            score_table[1] = 1;
            score_table[8] = 1;
            score_table[9] = 1;
        }
        if bitboard & 0b00000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000 != 0
        {
            score_table[6] = 1;
            score_table[14] = 1;
            score_table[15] = 1;
        }
        if bitboard & 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000000 != 0
        {
            score_table[48] = 1;
            score_table[49] = 1;
            score_table[57] = 1;
        }
        if bitboard & 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001 != 0
        {
            score_table[54] = 1;
            score_table[55] = 1;
            score_table[62] = 1;
        }
        let mut score = 0;
        while bitboard != 0 {
            let index = bitboard.trailing_zeros();
            score += score_table[63 - (index as usize)];
            bitboard &= !(1 << index);
        }
        score
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut black = self.black;
        let mut white = self.white;
        let mask: u64 = 1 << 63;

        writeln!(f, "  abcdefgh")?;

        for row in 1..=8 {
            write!(f, "{} ", row)?;

            for _col in 0..8 {
                let stone = if black & mask != 0 {
                    "x"
                } else if white & mask != 0 {
                    "o"
                } else {
                    "."
                };
                write!(f, "{}", stone)?;
                black <<= 1;
                white <<= 1;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
