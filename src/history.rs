use crate::board::Turn;
use crate::player::Action;
use crate::position::*;

#[derive(Debug)]
pub struct History {
    pub turn: Turn,
    pub action: Action,
    pub flipped: Positions,
}

impl History {
    pub fn new(turn: Turn, action: Action, flipped: Positions) -> Self {
        History {
            turn,
            action,
            flipped,
        }
    }
}
