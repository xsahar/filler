use crate::parser;
use crate::protocol;
use crate::strategy;

pub fn run_turn_from_str(input: &str) -> Result<String, String> {
    let turn = parser::parse_turn(input)?;
    let chosen = strategy::choose_move(&turn.state, &turn.piece);
    Ok(protocol::format_move(chosen))
}
