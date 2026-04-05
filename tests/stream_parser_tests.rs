use std::io::Cursor;

use filler::model::PlayerId;
use filler::parser;

#[test]
fn reads_player_and_single_turn_from_stream() {
    let fixture = std::fs::read_to_string("tests/fixtures/sample_turn_p1.txt")
        .expect("fixture should be readable");
    let mut reader = Cursor::new(fixture);

    let player = parser::read_player(&mut reader).expect("player should parse");
    assert_eq!(player, PlayerId::P1);

    let turn = parser::read_next_turn(&mut reader, player)
        .expect("turn parsing should succeed")
        .expect("one turn should exist");

    assert_eq!(turn.state.width, 20);
    assert_eq!(turn.state.height, 15);
    assert_eq!(turn.piece.width, 4);
    assert_eq!(turn.piece.height, 1);
}

#[test]
fn returns_none_at_stream_end() {
    let fixture = std::fs::read_to_string("tests/fixtures/sample_turn_p1.txt")
        .expect("fixture should be readable");
    let mut reader = Cursor::new(fixture);

    let player = parser::read_player(&mut reader).expect("player should parse");
    let _first = parser::read_next_turn(&mut reader, player)
        .expect("turn parsing should succeed")
        .expect("one turn should exist");
    let second = parser::read_next_turn(&mut reader, player).expect("end of stream should be handled");

    assert!(second.is_none());
}
