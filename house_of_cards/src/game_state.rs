#[derive(PartialEq, Eq)]
pub enum GameState {
    Alive,
    Dead,
    Paused,
    Powerup,
}

impl GameState {
    pub fn new() -> Self {
        Self::Alive
    }

    pub fn toggle_pause(&mut self) {
        match self {
            GameState::Alive => *self = GameState::Paused,
            GameState::Paused => *self = GameState::Alive,
            _ => {}
        }
    }
}
