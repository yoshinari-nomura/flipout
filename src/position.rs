//! Position

use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Debug)]
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
    pub fn new(position: u64) -> Self {
        Position(position)
    }

    pub fn from_xy(x: i32, y: i32) -> Option<Self> {
        if 0 <= x && x <= 7 && 0 <= y && y <= 7 {
            Some(Position((1 << 63) >> (y * 8 + x)))
        } else {
            None
        }
    }

    pub fn as_bits(&self) -> u64 {
        self.0
    }

    pub fn from_bits(position: u64) -> Option<Self> {
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
        let col: Vec<char> = "abcdefgh".chars().collect();
        write!(f, "{}{}", col[self.x() as usize], row[self.y() as usize])?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////
// Positions

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Positions(u64);

impl Positions {
    pub fn new(bits: u64) -> Self {
        Positions(bits)
    }

    pub fn fill() -> Self {
        Self::new(0xffff_ffff_ffff_ffff)
    }

    pub fn empty() -> Self {
        Self::new(0)
    }

    pub fn as_bits(&self) -> u64 {
        self.0
    }

    pub fn contains(&self, pos: Position) -> bool {
        pos.0 & self.0 != 0
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
                let pos1 = Position::from_str(&format!("{}{}", cc, rc)).unwrap();
                let pos2 = Position::from_bits((1 << 63) >> (ri * 8 + ci)).unwrap();
                assert_eq!(pos1, pos2);
            }
        }
    }
}
