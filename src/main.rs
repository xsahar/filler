//entrypoint used by the game engine.
use std::io::{self, BufReader, BufWriter, Write};

fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    let player = match filler::parser::read_player(&mut reader) {
        Ok(p) => p,
        Err(_) => {
            println!("0 0");
            return;
        }
    };

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    loop {
        let next_turn = filler::parser::read_next_turn(&mut reader, player);
        let turn = match next_turn {
            Ok(Some(turn)) => turn,
            Ok(None) => break,
            Err(_) => {
                if writer.write_all(b"0 0\n").is_ok() {
                    let _ = writer.flush();
                }
                break;
            }
        };

        let chosen = filler::strategy::choose_move(&turn.state, &turn.piece);
        let out = filler::protocol::format_move(chosen);

        if writer.write_all(out.as_bytes()).is_err() {
            break;
        }
        if writer.flush().is_err() {
            break;
        }
    }

    if writer.flush().is_err() {
        return;
    }
}
