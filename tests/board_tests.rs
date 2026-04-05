use filler::board;
use filler::model::{GameState, Piece, PlayerId};

fn state_from_rows(rows: &[&str], player: PlayerId) -> GameState {
    let board: Vec<Vec<char>> = rows.iter().map(|r| r.chars().collect()).collect();
    GameState {
        width: rows[0].chars().count(),
        height: rows.len(),
        board,
        player,
    }
}

#[test]
fn valid_when_exactly_one_self_overlap() {
    let state = state_from_rows(
        &[
            ".....",
            "..@..",
            ".....",
            ".....",
            ".....",
        ],
        PlayerId::P1,
    );

    let piece = Piece {
        width: 2,
        height: 1,
        cells: vec![(0, 0), (1, 0)],
    };

    assert!(board::is_valid_move(&state, &piece, 1, 1));
}

#[test]
fn invalid_when_zero_self_overlap() {
    let state = state_from_rows(
        &[
            ".....",
            "..@..",
            ".....",
            ".....",
            ".....",
        ],
        PlayerId::P1,
    );

    let piece = Piece {
        width: 1,
        height: 1,
        cells: vec![(0, 0)],
    };

    assert!(!board::is_valid_move(&state, &piece, 0, 0));
}

#[test]
fn invalid_when_more_than_one_self_overlap() {
    let state = state_from_rows(
        &[
            ".....",
            ".@@..",
            ".....",
            ".....",
            ".....",
        ],
        PlayerId::P1,
    );

    let piece = Piece {
        width: 2,
        height: 1,
        cells: vec![(0, 0), (1, 0)],
    };

    assert!(!board::is_valid_move(&state, &piece, 1, 1));
}

#[test]
fn invalid_when_overlapping_opponent() {
    let state = state_from_rows(
        &[
            ".....",
            "..$..",
            "..@..",
            ".....",
            ".....",
        ],
        PlayerId::P1,
    );

    let piece = Piece {
        width: 1,
        height: 2,
        cells: vec![(0, 0), (0, 1)],
    };

    assert!(!board::is_valid_move(&state, &piece, 2, 1));
}

#[test]
fn invalid_when_out_of_bounds() {
    let state = state_from_rows(
        &[
            ".....",
            "..@..",
            ".....",
            ".....",
            ".....",
        ],
        PlayerId::P1,
    );

    let piece = Piece {
        width: 2,
        height: 2,
        cells: vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    };

    assert!(!board::is_valid_move(&state, &piece, 4, 4));
    assert!(!board::is_valid_move(&state, &piece, -1, 1));
}
