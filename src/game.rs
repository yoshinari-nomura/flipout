use crate::board::*;
use crate::player::*;
use crate::position::*;
use crate::ui_board::*;
use wasm_bindgen::prelude::*;

macro_rules! message {
    ( $id:expr, $( $t:tt )* ) => {
        screen_show_message($id, &format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Game {
    board: UiBoard,
    ai: Box<dyn Player>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        let ui = UiBoard::new(false);
        let ai: Box<dyn Player> = Box::new(CleverRobotPlayer::new());
        Game { board: ui, ai }
    }

    pub fn ui_move(&mut self, turn: Turn, x: i32, y: i32) -> bool {
        if let Some(pos) = Position::from_xy(x, y) {
            let action = Action::Move(pos);

            return match self.update(turn, action) {
                Some(t) if t == turn => false, // again, still your turn
                Some(_) => true,               // done, turn to AI
                None => false,                 // gameover, don't handover
            };
        }
        false // pos is invalid, try again.
    }

    pub fn ui_pass(&mut self, turn: Turn) -> bool {
        match self.update(turn, Action::Pass) {
            Some(t) if t == turn => false, // again, still your turn
            Some(_) => true,               // done, turn to AI
            None => false,                 // gameover, don't handover
        }
    }

    pub fn ai_action(&mut self, turn: Turn) -> bool {
        let board = &mut self.board;
        let action = self.ai.action(board);

        match self.update(turn, action) {
            Some(t) if t == turn => false, // again, still your turn
            Some(_) => true,               // done, turn to UI
            None => true,                  // gameover, turn to UI
        }
    }

    pub fn update_screen(&self) {
        self.update_screen_with_animation(0);
    }
}

impl Game {
    fn update(&mut self, turn: Turn, action: Action) -> Option<Turn> {
        if self.board.whatnow() != Some(turn) {
            return self.board.whatnow();
        }

        let name = if turn == Turn::Black { "you" } else { "com" };

        match action {
            Action::GiveUp => message!(name, "Give up"),
            Action::Pass => {
                if self.board.pass().is_ok() {
                    message!(name, "Pass");
                    self.update_screen();
                } else {
                    message!(name, "Can't pass");
                }
            }
            Action::Move(mov) => {
                let reversible = self.board.reversible_stones(mov.as_bits());
                if self.board.put_stone(mov.as_bits()).is_ok() {
                    message!(name, "Move {}", mov);
                    if let Some(next_turn) = self.board.whatnow() {
                        if next_turn == turn {
                            let name = if turn.opposit() == Turn::Black {
                                "you"
                            } else {
                                "com"
                            };
                            message!(name, "Pass");
                        }
                    }
                    self.update_screen_with_animation(reversible);
                }
            }
        }
        self.board.whatnow()
    }

    fn update_screen_with_animation(&self, reversed: u64) {
        let board = &self.board;
        message!("black", "{}", board.count_black());
        message!("white", "{}", board.count_white());

        for i in 0..64 {
            let (x, y) = (i % 8, i / 8);
            let pos = (1 << 63) >> i;
            let (opcode, color) = self.operation_at(pos, reversed);
            screen_update_grid(opcode, color, x, y);
        }
    }

    fn operation_at(&self, pos: Move, flipped: u64) -> (&str, &str) {
        let grid_color = match self.board.color_at(pos) {
            Color::White => "white",
            Color::Black => "black",
            Color::Empty => "empty",
        };

        let hint_color = match self.board.turn() {
            Some(Turn::White) => "white",
            Some(Turn::Black) => "black",
            None => "empty", // XXX it works, but should be cared in screen.js
        };

        if self.board.is_legal_move(pos) {
            ("hint", hint_color)
        } else if pos & flipped != 0 {
            ("flip", grid_color)
        } else if grid_color == "empty" {
            ("remove", "")
        } else {
            ("put", grid_color)
        }
    }
}

// #[wasm_bindgen]
#[wasm_bindgen(module = "/src/javascripts/screen.js")]
extern "C" {
    pub fn screen_update_grid(op: &str, color: &str, x: i32, y: i32);
    pub fn screen_put_stone(color: &str, x: i32, y: i32);
    pub fn screen_put_hint(color: &str, x: i32, y: i32);
    pub fn screen_remove_stone(x: i32, y: i32);
    pub fn screen_flip_to(color: &str, x: i32, y: i32, delay: i32);
    pub fn screen_show_message(id: &str, message: &JsValue);
}
