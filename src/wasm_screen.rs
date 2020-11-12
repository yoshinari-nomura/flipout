use crate::board::Turn;
use crate::player::Action;
use crate::position::*;
use crate::ui_board::{Color, UiBoard};

use wasm_bindgen::prelude::*;

macro_rules! message {
    ( $id:expr, $( $t:tt )* ) => {
        screen_show_message($id, &format!( $( $t )* ).into());
    }
}

pub struct WasmScreen;

impl Default for WasmScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl WasmScreen {
    pub fn new() -> Self {
        WasmScreen
    }

    pub fn update_screen_with_animation(&self, reversed: Positions, board: &UiBoard) {
        self.update_message(board, Turn::Black);
        self.update_message(board, Turn::White);

        for pos in Positions::fill() {
            let (opcode, color) = self.operation_at(board, pos, reversed);
            screen_update_grid(opcode, color, pos.x(), pos.y());
        }
    }

    fn update_message(&self, board: &UiBoard, turn: Turn) {
        if turn == Turn::Black {
            message!("black", "{}", board.count_black());
        } else {
            message!("white", "{}", board.count_white());
        }

        let name = if turn == Turn::Black { "you" } else { "com" };
        let last_action = board.last_action(turn);

        match last_action {
            Some(Action::GiveUp) => message!(name, "Give up"),
            Some(Action::Pass) => message!(name, "Pass"),
            Some(Action::Move(pos)) => message!(name, "Move {}", pos),
            None => (),
        }
    }

    fn operation_at(&self, board: &UiBoard, pos: Position, flipped: Positions) -> (&str, &str) {
        let grid_color = match board.color_at(pos) {
            Color::White => "white",
            Color::Black => "black",
            Color::Empty => "empty",
        };

        let hint_color = match board.turn() {
            Some(Turn::White) => "white",
            Some(Turn::Black) => "black",
            None => "empty", // XXX it works, but should be cared in screen.js
        };

        if board.is_legal_move(pos) {
            ("hint", hint_color)
        } else if flipped.contains(pos) {
            ("flip", grid_color)
        } else if grid_color == "empty" {
            ("remove", "")
        } else {
            ("put", grid_color)
        }
    }
}

#[wasm_bindgen(module = "/src/javascripts/screen.js")]
extern "C" {
    pub fn screen_update_grid(op: &str, color: &str, x: i32, y: i32);
    pub fn screen_show_message(id: &str, message: &JsValue);
}
