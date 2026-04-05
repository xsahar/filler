#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerId {
    P1,
    P2,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub width: usize,
    pub height: usize,
    pub board: Vec<Vec<char>>,
    pub player: PlayerId,
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<(i32, i32)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub x: i32,
    pub y: i32,
}

impl GameState {
    pub fn is_self_cell(&self, ch: char) -> bool {
        match self.player {
            PlayerId::P1 => ch == '@' || ch == 'a',
            PlayerId::P2 => ch == '$' || ch == 's',
        }
    }

    pub fn is_opponent_cell(&self, ch: char) -> bool {
        match self.player {
            PlayerId::P1 => ch == '$' || ch == 's',
            PlayerId::P2 => ch == '@' || ch == 'a',
        }
    }
}
