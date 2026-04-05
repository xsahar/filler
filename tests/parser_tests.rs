use filler::parser;

#[test]
fn parses_sample_turn_fixture() {
    let input = std::fs::read_to_string("tests/fixtures/sample_turn_p1.txt")
        .expect("fixture should be readable");

    let parsed = parser::parse_turn(&input).expect("fixture should parse");

    assert_eq!(parsed.state.width, 20);
    assert_eq!(parsed.state.height, 15);
    assert_eq!(parsed.state.board.len(), 15);
    assert_eq!(parsed.piece.width, 4);
    assert_eq!(parsed.piece.height, 1);
    assert_eq!(parsed.piece.cells, vec![(1, 0), (2, 0)]);
}
