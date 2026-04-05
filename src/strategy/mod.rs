use crate::board;
use crate::model::{GameState, Move, Piece};

pub fn choose_move(state: &GameState, piece: &Piece) -> Option<Move> {
    if state.width == 0 || state.height == 0 {
        return None;
    }

    let opponents = collect_opponents(state);
    let center_x = (state.width as i32) / 2;
    let center_y = (state.height as i32) / 2;

    let mut best_move = None;
    let mut best_score = i64::MAX;

    for y in 0..state.height as i32 {
        for x in 0..state.width as i32 {
            if board::is_valid_move(state, piece, x, y) {
                let score = score_move(piece, x, y, &opponents, center_x, center_y);
                if score < best_score {
                    best_score = score;
                    best_move = Some(Move { x, y });
                }
            }
        }
    }

    best_move
}

fn collect_opponents(state: &GameState) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    for y in 0..state.height {
        for x in 0..state.width {
            if state.is_opponent_cell(state.board[y][x]) {
                out.push((x as i32, y as i32));
            }
        }
    }
    out
}

fn score_move(
    piece: &Piece,
    x: i32,
    y: i32,
    opponents: &[(i32, i32)],
    center_x: i32,
    center_y: i32,
) -> i64 {
    let mut dist_sum = 0_i64;

    for (dx, dy) in &piece.cells {
        let px = x + dx;
        let py = y + dy;

        if opponents.is_empty() {
            let center_dist = (px - center_x).abs() + (py - center_y).abs();
            dist_sum += center_dist as i64;
            continue;
        }

        let mut min_dist = i32::MAX;
        for (ox, oy) in opponents {
            let d = (px - *ox).abs() + (py - *oy).abs();
            if d < min_dist {
                min_dist = d;
            }
        }
        dist_sum += min_dist as i64;
    }

    // Prefer moves toward opponent while softly preferring central control.
    let anchor_center_dist = (x - center_x).abs() + (y - center_y).abs();
    dist_sum * 10 + anchor_center_dist as i64
}
