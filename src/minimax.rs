//! simple implementation of Mini-Max method.

use crate::board::{Board, Turn};
use std::cmp::{max, min};

/// return score for board
pub fn minimax(board: &Board, me: Turn, depth: i32) -> i32 {
    alpha_beta(board, me, depth, std::i32::MIN, std::i32::MAX)
}

fn alpha_beta(board: &Board, me: Turn, depth: i32, mut alpha: i32, mut beta: i32) -> i32 {
    if depth <= 0 {
        return board.eval_score(me);
    }

    let children = board.children();

    if children.is_empty() {
        // game over
        return board.eval_score(me);
    }

    if me == board.turn {
        for child in &children {
            let ab = alpha_beta(child, me, depth - 1, alpha, beta);
            alpha = max(alpha, ab);
            if alpha >= beta {
                break;
            }
        }
        alpha
    } else {
        for child in &children {
            let ab = alpha_beta(child, me, depth - 1, alpha, beta);
            beta = min(beta, ab);
            if alpha >= beta {
                break;
            }
        }
        beta
    }
}
