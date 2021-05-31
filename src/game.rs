use crate::position::Position;

pub struct Game {
    pub position: Position,
}

impl Game {
    pub fn default() -> Self {
        Self { position: Position::default() }
    }
}
