use crate::board::*;
use crate::player::*;
use crate::position::*;
use crate::ui_board::*;
use crate::wasm_screen::*;

use wasm_bindgen::prelude::*;

macro_rules! message {
    ( $id:expr, $( $t:tt )* ) => {
        screen_show_message($id, &format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Game {
    board: UiBoard,
    screen: WasmScreen,
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
        let ui = UiBoard::new();
        let ai: Box<dyn Player> = Box::new(CleverRobotPlayer::new());
        let screen = WasmScreen::new();
        Game {
            board: ui,
            ai,
            screen,
        }
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
        let board = &self.board;
        self.screen
            .update_screen_with_animation(Positions::empty(), board);
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
            Action::Move(pos) => {
                let reversible = self.board.reversible_stones(pos);
                if self.board.put_stone(pos).is_ok() {
                    message!(name, "Move {}", pos);
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
                    self.screen
                        .update_screen_with_animation(reversible, &self.board);
                }
            }
        }
        self.board.whatnow()
    }
}

#[wasm_bindgen(module = "/src/javascripts/screen.js")]
extern "C" {
    pub fn screen_show_message(id: &str, message: &JsValue);
}
