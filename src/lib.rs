//! flipout is a reversi (Othello) implementation in Rust
//!

pub type BitBoard = u64;
pub type Position = u64;
pub type Positions = u64;

pub mod bitboard;
pub mod board;
pub mod minimax;
pub mod player;
pub mod position;
pub mod ui_board;

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
