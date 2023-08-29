#[derive(PartialEq, Eq)]
pub enum GameState {
    Alive,
    Dead,
}

impl GameState {
    pub fn new() -> Self {
        Self::Alive
    }
}
