//! Bitboard is a single 64bit-value for expressing positions of disks.
//!
//! MSB is `a1`, LSB is `h8`:
//!
//! ```text
//!    a  b  c  d  e  f  g  h
//!  +------------------------+
//! 1| 63 62 61 60 59 58 57 56|
//! 2| 55 54 53 52 51 ........|
//! 3| ....                   |
//! :| ....                   |
//! 8| 07 06 05 04 03 02 01 00|
//!  +-------------------------
//! ```
//!
//! Each 0/1 bit represents the absence/presence of a disk.
//! If you want to express a complete state of game,
//! you will need two bitboards for black and white.
//!

/// A single 64bit-value for expressing positions of disks.
///
/// MSB is `a1`, LSB is `h8`:
///
/// ```text
///    a  b  c  d  e  f  g  h
///  +------------------------+
/// 1| 63 62 61 60 59 58 57 56|
/// 2| 55 54 53 52 51 ........|
/// 3| ....                   |
/// :| ....                   |
/// 8| 07 06 05 04 03 02 01 00|
///  +-------------------------
/// ```
///
/// Each 0/1 bit represents the absence/presence of a disk.
/// If you want to express a complete state of game,
/// you will need two bitboards for black and white.
pub type BitBoard = u64;

/// A single 64bit-value for expressing a *move*.
///
/// MSB is `a1`, LSB is `h8`:
///
/// ```text
///    a  b  c  d  e  f  g  h
///  +------------------------+
/// 1| 63 62 61 60 59 58 57 56|
/// 2| 55 54 53 52 51 ........|
/// 3| ....                   |
/// :| ....                   |
/// 8| 07 06 05 04 03 02 01 00|
///  +-------------------------
/// ```
///
/// Since, it represents a position of a single move,
/// only a single bit should be 1, others should be 0.
pub type Move = u64;

/// A single 64bit-value for expressing *multiple* *moves*.
///
/// MSB is `a1`, LSB is `h8`:
///
/// ```text
///    a  b  c  d  e  f  g  h
///  +------------------------+
/// 1| 63 62 61 60 59 58 57 56|
/// 2| 55 54 53 52 51 ........|
/// 3| ....                   |
/// :| ....                   |
/// 8| 07 06 05 04 03 02 01 00|
///  +-------------------------
/// ```
///
/// It usually represents some possible moves.
/// Multiple-bits woule be 1.
pub type Moves = u64;

/// Find all legal moves for black player.
pub fn legal_moves(black: BitBoard, white: BitBoard) -> Moves {
    // H_SENTINEL V_SENTINEL B_SENTINEL
    //
    // 01111110   00000000   00000000,
    // 01111110   11111111   01111110,
    // 01111110   11111111   01111110,
    // 01111110   11111111   01111110,
    // 01111110   11111111   01111110,
    // 01111110   11111111   01111110,
    // 01111110   11111111   01111110,
    // 01111110   00000000   00000000,
    const H_SENTINEL: BitBoard = 0x7e7e_7e7e_7e7e_7e7e;
    const V_SENTINEL: BitBoard = 0x00ff_ffff_ffff_ff00;
    const B_SENTINEL: BitBoard = 0x007e_7e7e_7e7e_7e00;

    let hole = !(black | white);
    let mut moves: Moves = 0;
    let mut tmp: BitBoard;
    let mut mask: BitBoard;

    // left
    mask = white & H_SENTINEL;
    tmp = mask & (black << 1);
    tmp |= mask & (tmp << 1);
    tmp |= mask & (tmp << 1);
    tmp |= mask & (tmp << 1);
    tmp |= mask & (tmp << 1);
    tmp |= mask & (tmp << 1);
    moves |= hole & (tmp << 1);

    // right
    tmp = mask & (black >> 1);
    tmp |= mask & (tmp >> 1);
    tmp |= mask & (tmp >> 1);
    tmp |= mask & (tmp >> 1);
    tmp |= mask & (tmp >> 1);
    tmp |= mask & (tmp >> 1);
    moves |= hole & (tmp >> 1);

    // up
    mask = white & V_SENTINEL;
    tmp = mask & (black << 8);
    tmp |= mask & (tmp << 8);
    tmp |= mask & (tmp << 8);
    tmp |= mask & (tmp << 8);
    tmp |= mask & (tmp << 8);
    tmp |= mask & (tmp << 8);
    moves |= hole & (tmp << 8);

    // down
    tmp = mask & (black >> 8);
    tmp |= mask & (tmp >> 8);
    tmp |= mask & (tmp >> 8);
    tmp |= mask & (tmp >> 8);
    tmp |= mask & (tmp >> 8);
    tmp |= mask & (tmp >> 8);
    moves |= hole & (tmp >> 8);

    // upper-left
    mask = white & B_SENTINEL;
    tmp = mask & (black << 9);
    tmp |= mask & (tmp << 9);
    tmp |= mask & (tmp << 9);
    tmp |= mask & (tmp << 9);
    tmp |= mask & (tmp << 9);
    tmp |= mask & (tmp << 9);
    moves |= hole & (tmp << 9);

    // upper-right
    tmp = mask & (black << 7);
    tmp |= mask & (tmp << 7);
    tmp |= mask & (tmp << 7);
    tmp |= mask & (tmp << 7);
    tmp |= mask & (tmp << 7);
    tmp |= mask & (tmp << 7);
    moves |= hole & (tmp << 7);

    // down-left
    tmp = mask & (black >> 7);
    tmp |= mask & (tmp >> 7);
    tmp |= mask & (tmp >> 7);
    tmp |= mask & (tmp >> 7);
    tmp |= mask & (tmp >> 7);
    tmp |= mask & (tmp >> 7);
    moves |= hole & (tmp >> 7);

    // down-right
    tmp = mask & (black >> 9);
    tmp |= mask & (tmp >> 9);
    tmp |= mask & (tmp >> 9);
    tmp |= mask & (tmp >> 9);
    tmp |= mask & (tmp >> 9);
    tmp |= mask & (tmp >> 9);
    moves |= hole & (tmp >> 9);

    moves
}

/// Find reversible stones if black moves at `position`.
pub fn reversible_stones(black: BitBoard, white: BitBoard, position: Move) -> BitBoard {
    let mut reversible: BitBoard = 0;

    // Make sure no stone at `position`.
    if (black | white) & position != 0 {
        return 0;
    }

    // search for all directions from `position`.
    for direction in 1..=8 {
        let mut pos: BitBoard = shift(position, direction);
        let mut rev: BitBoard = 0;

        // while `pos` is not out-of-bounds,
        // and a white stone is found at `pos`.
        while pos != 0 && (pos & white) != 0 {
            // mark the white stone at `pos` is reversible.
            rev |= pos;
            // move pos to next at direction.
            pos = shift(pos, direction);
        }
        // discard `rev` if black stone is not found at the edge.
        reversible |= if (pos & black) == 0 { 0 } else { rev }
    }
    reversible
}

fn shift(position: Move, direction: u8) -> Move {
    match direction {
        1 => (position << 8) & 0xffffffffffffff00,
        2 => (position >> 8) & 0x00ffffffffffffff,
        3 => (position << 1) & 0xfefefefefefefefe,
        4 => (position >> 1) & 0x7f7f7f7f7f7f7f7f,
        5 => (position << 9) & 0xfefefefefefefe00,
        6 => (position << 7) & 0x7f7f7f7f7f7f7f00,
        7 => (position >> 7) & 0x00fefefefefefefe,
        8 => (position >> 9) & 0x007f7f7f7f7f7f7f,
        _ => panic!("Wrong direction"),
    }
}
