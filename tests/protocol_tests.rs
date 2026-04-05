use filler::model::Move;
use filler::protocol;

#[test]
fn formats_expected_coordinate_output() {
    let out = protocol::format_move(Some(Move { x: 7, y: 2 }));
    assert_eq!(out, "7 2\n");
}

#[test]
fn formats_fallback_when_no_move() {
    let out = protocol::format_move(None);
    assert_eq!(out, "0 0\n");
}
