use crate::model::{GameState, Piece};

pub fn is_valid_move(state: &GameState, piece: &Piece, x: i32, y: i32) -> bool {
    let mut self_overlaps = 0;

    for (dx, dy) in &piece.cells {
        let bx = x + dx;
        let by = y + dy;

        if bx < 0 || by < 0 {
            return false;
        }

        let bx = bx as usize;
        let by = by as usize;

        if bx >= state.width || by >= state.height {
            return false;
        }

        let ch = state.board[by][bx];
        if state.is_opponent_cell(ch) {
            return false;
        }

        if state.is_self_cell(ch) {
            self_overlaps += 1;
            if self_overlaps > 1 {
                return false;
            }
        }
    }

    self_overlaps == 1
}
