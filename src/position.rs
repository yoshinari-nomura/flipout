//! Position

use crate::BitBoard;
use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Position {
    pos: BitBoard,
}

impl Position {
    pub fn from_str(position_str: &str) -> Option<Self> {
        let ascii = position_str.as_bytes();

        if ascii.len() != 2 {
            return None;
        }

        let col: i32 = (ascii[0] as i32) - ('a' as i32);
        let row: i32 = (ascii[1] as i32) - ('1' as i32);

        if 0 <= col && col <= 7 && 0 <= row && row <= 7 {
            Some(Position {
                pos: (1 << 63) >> (row * 8 + col),
            })
        } else {
            None
        }
    }

    pub fn as_bitboard(&self) -> BitBoard {
        self.pos
    }

    pub fn from_u64(position: u64) -> Option<Self> {
        if position.count_ones() != 1 {
            None
        } else {
            Some(Position { pos: position })
        }
    }
}

// impl Iterator for board::Positions {
//     type Item = Position;
//     fn next(&mut self) -> Option<Self::Item> {
//         // --snip--
//     }
// }

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let row: Vec<char> = "12345678".chars().collect();
        let col: Vec<char> = "abcdefgh".chars().collect();
        let num = self.pos.leading_zeros();
        write!(f, "{}{}", col[(num % 8) as usize], row[(num / 8) as usize])?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn position_from_str() {
        for (ci, cc) in "abcdefgh".chars().enumerate() {
            for (ri, rc) in "12345678".chars().enumerate() {
                let pos1 = Position::from_str(&format!("{}{}", cc, rc));
                let pos2 = Position::from_u64((1 << 63) >> (ri * 8 + ci));
                assert_eq!(pos1, pos2);
            }
        }
    }
}
