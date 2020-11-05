//! Position

use crate::BitBoard;
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Position(u64);

impl FromStr for Position {
    type Err = ();

    fn from_str(position_str: &str) -> Result<Self, ()> {
        let ascii = position_str.as_bytes();

        if ascii.len() != 2 {
            return Err(());
        }

        let col: i32 = (ascii[0] as i32) - ('a' as i32);
        let row: i32 = (ascii[1] as i32) - ('1' as i32);

        if 0 <= col && col <= 7 && 0 <= row && row <= 7 {
            Ok(Position((1 << 63) >> (row * 8 + col)))
        } else {
            Err(())
        }
    }
}

impl Position {
    /// XXX Result<Position, PositionParseErr> is suitable
    pub fn from_str_opt(position_str: &str) -> Option<Self> {
        let ascii = position_str.as_bytes();

        if ascii.len() != 2 {
            return None;
        }

        let col: i32 = (ascii[0] as i32) - ('a' as i32);
        let row: i32 = (ascii[1] as i32) - ('1' as i32);

        if 0 <= col && col <= 7 && 0 <= row && row <= 7 {
            Some(Position((1 << 63) >> (row * 8 + col)))
        } else {
            None
        }
    }

    pub fn from_xy(x: i32, y: i32) -> Option<Self> {
        if 0 <= x && x <= 7 && 0 <= y && y <= 7 {
            Some(Position((1 << 63) >> (y * 8 + x)))
        } else {
            None
        }
    }

    pub fn as_bitboard(&self) -> BitBoard {
        self.0
    }

    pub fn from_u64(position: u64) -> Option<Self> {
        if position.count_ones() != 1 {
            None
        } else {
            Some(Position(position))
        }
    }

    pub fn x(&self) -> i32 {
        (self.0.leading_zeros() % 8) as i32
    }

    pub fn y(&self) -> i32 {
        (self.0.leading_zeros() / 8) as i32
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let row: Vec<char> = "12345678".chars().collect();
        let col: Vec<char> = "ABCDEFGH".chars().collect();
        write!(f, "{}{}", col[self.x() as usize], row[self.y() as usize])?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////
// Positions

struct Positions(u64);

impl Positions {
    pub fn new(bits: u64) -> Self {
        Positions(bits)
    }
}

impl Iterator for Positions {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 != 0 {
            let mov = 1 << self.0.trailing_zeros();
            self.0 &= !mov;
            Some(Position(mov))
        } else {
            None
        }
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
