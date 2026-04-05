use crate::model::Move;

pub fn format_move(chosen: Option<Move>) -> String {
    match chosen {
        Some(mv) => format!("{} {}\n", mv.x, mv.y),
        None => "0 0\n".to_string(),
    }
}
