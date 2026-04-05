use crate::model::{GameState, Piece, PlayerId};
use std::io::BufRead;

#[derive(Debug, Clone)]
pub struct TurnInput {
    pub state: GameState,
    pub piece: Piece,
}

pub fn parse_turn(input: &str) -> Result<TurnInput, String> {
    let mut lines = input.lines();

    let player_line = lines
        .next()
        .ok_or_else(|| "missing player line".to_string())?;
    let player = if player_line.contains("p1") {
        PlayerId::P1
    } else if player_line.contains("p2") {
        PlayerId::P2
    } else {
        return Err("unknown player id".to_string());
    };

    parse_turn_after_player(&mut lines, player)
}

pub fn read_player<R: BufRead>(reader: &mut R) -> Result<PlayerId, String> {
    let line = read_line_trimmed(reader)?
        .ok_or_else(|| "missing player line".to_string())?;
    parse_player_line(&line)
}

pub fn read_next_turn<R: BufRead>(reader: &mut R, player: PlayerId) -> Result<Option<TurnInput>, String> {
    let anfield_line = match read_until_header(reader, "Anfield")? {
        Some(line) => line,
        None => return Ok(None),
    };

    parse_turn_from_header(reader, player, &anfield_line).map(Some)
}

fn parse_player_line(line: &str) -> Result<PlayerId, String> {
    if line.contains("p1") {
        Ok(PlayerId::P1)
    } else if line.contains("p2") {
        Ok(PlayerId::P2)
    } else {
        Err("unknown player id".to_string())
    }
}

fn parse_turn_after_player<'a, I>(lines: &mut I, player: PlayerId) -> Result<TurnInput, String>
where
    I: Iterator<Item = &'a str>,
{
    let anfield_line = lines
        .next()
        .ok_or_else(|| "missing anfield header".to_string())?;

    let (width, height) = parse_dims(anfield_line, "Anfield")?;

    let _header_digits = lines
        .next()
        .ok_or_else(|| "missing board index header".to_string())?;

    let mut board = Vec::with_capacity(height);
    for _ in 0..height {
        let row_line = lines
            .next()
            .ok_or_else(|| "missing board row".to_string())?;
        let row = parse_board_row(row_line)?;
        let chars: Vec<char> = row.chars().collect();
        if chars.len() != width {
            return Err("board row width mismatch".to_string());
        }
        board.push(chars);
    }

    let piece_line = lines
        .next()
        .ok_or_else(|| "missing piece header".to_string())?;
    let (piece_width, piece_height) = parse_dims(piece_line, "Piece")?;

    let mut piece_rows = Vec::with_capacity(piece_height);
    for _ in 0..piece_height {
        let row = lines
            .next()
            .ok_or_else(|| "missing piece row".to_string())?;
        if row.chars().count() != piece_width {
            return Err("piece row width mismatch".to_string());
        }
        piece_rows.push(row.to_string());
    }

    let cells = build_piece_cells(&piece_rows)?;

    Ok(TurnInput {
        state: GameState {
            width,
            height,
            board,
            player,
        },
        piece: Piece {
            width: piece_width,
            height: piece_height,
            cells,
        },
    })
}

fn parse_turn_from_header<R: BufRead>(reader: &mut R, player: PlayerId, anfield_line: &str) -> Result<TurnInput, String> {
    let (width, height) = parse_dims(anfield_line, "Anfield")?;

    let _header_digits = read_line_trimmed(reader)?
        .ok_or_else(|| "missing board index header".to_string())?;

    let mut board = Vec::with_capacity(height);
    for _ in 0..height {
        let row_line = read_line_trimmed(reader)?
            .ok_or_else(|| "missing board row".to_string())?;
        let row = parse_board_row(&row_line)?;
        let chars: Vec<char> = row.chars().collect();
        if chars.len() != width {
            return Err("board row width mismatch".to_string());
        }
        board.push(chars);
    }

    let piece_line = read_line_trimmed(reader)?
        .ok_or_else(|| "missing piece header".to_string())?;
    let (piece_width, piece_height) = parse_dims(&piece_line, "Piece")?;

    let mut piece_rows = Vec::with_capacity(piece_height);
    for _ in 0..piece_height {
        let row = read_line_trimmed(reader)?
            .ok_or_else(|| "missing piece row".to_string())?;
        if row.chars().count() != piece_width {
            return Err("piece row width mismatch".to_string());
        }
        piece_rows.push(row);
    }

    let cells = build_piece_cells(&piece_rows)?;

    Ok(TurnInput {
        state: GameState {
            width,
            height,
            board,
            player,
        },
        piece: Piece {
            width: piece_width,
            height: piece_height,
            cells,
        },
    })
}

fn parse_board_row(line: &str) -> Result<&str, String> {
    let mut parts = line.split_whitespace();
    let _idx = parts
        .next()
        .ok_or_else(|| "malformed board row".to_string())?;
    let row = parts
        .next()
        .ok_or_else(|| "malformed board row".to_string())?;
    Ok(row)
}

fn build_piece_cells(rows: &[String]) -> Result<Vec<(i32, i32)>, String> {
    let mut cells = Vec::new();
    for (dy, row) in rows.iter().enumerate() {
        for (dx, ch) in row.chars().enumerate() {
            if ch != '.' {
                cells.push((dx as i32, dy as i32));
            }
        }
    }

    if cells.is_empty() {
        return Err("piece has no occupied cells".to_string());
    }

    Ok(cells)
}

fn read_until_header<R: BufRead>(reader: &mut R, header: &str) -> Result<Option<String>, String> {
    loop {
        let line = match read_line_trimmed(reader)? {
            Some(line) => line,
            None => return Ok(None),
        };
        if line.starts_with(header) {
            return Ok(Some(line));
        }
    }
}

fn read_line_trimmed<R: BufRead>(reader: &mut R) -> Result<Option<String>, String> {
    let mut line = String::new();
    let n = reader
        .read_line(&mut line)
        .map_err(|e| format!("failed reading input: {}", e))?;
    if n == 0 {
        return Ok(None);
    }
    Ok(Some(line.trim_end_matches(&['\r', '\n'][..]).to_string()))
}

fn parse_dims(line: &str, label: &str) -> Result<(usize, usize), String> {
    if !line.starts_with(label) {
        return Err(format!("expected {} header", label));
    }

    let clean = line.trim_end_matches(':');
    let parts: Vec<&str> = clean.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(format!("malformed {} header", label));
    }

    let width = parts[1]
        .parse::<usize>()
        .map_err(|_| format!("bad {} width", label))?;
    let height = parts[2]
        .parse::<usize>()
        .map_err(|_| format!("bad {} height", label))?;

    Ok((width, height))
}
