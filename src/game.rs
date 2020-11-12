use crate::board::*;
use crate::player::*;
use crate::position::*;
use crate::ui_board::*;
use crate::wasm_screen::*;

use wasm_bindgen::prelude::*;

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

        match action {
            Action::GiveUp => (),
            Action::Pass => {
                if self.board.pass().is_ok() {
                    self.update_screen();
                }
            }
            Action::Move(pos) => {
                let reversible = self.board.reversible_stones(pos);
                if self.board.put_stone(pos).is_ok() {
                    self.screen
                        .update_screen_with_animation(reversible, &self.board);
                }
            }
        }
        self.board.whatnow()
    }
}
