#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Alive,
    Dead,
    Paused,
    PowerupStat,
    PowerupCard,
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

pub struct GameStateManager {
    current_state: GameState,
    last_state: GameState,
}

impl GameStateManager {
    pub fn new() -> Self {
        Self {
            current_state: GameState::new(),
            last_state: GameState::new(),
        }
    }

    pub fn show_mouse(&self) -> bool {
        self.powerup() || self.current_state == GameState::Paused
    }

    pub fn powerup(&self) -> bool {
        matches!(
            self.current_state,
            GameState::PowerupStat | GameState::PowerupCard
        )
    }

    pub fn next(&mut self, next_state: GameState) {
        self.last_state = self.current_state;
        self.current_state = next_state;
    }

    pub fn back(&mut self) {
        std::mem::swap(&mut self.current_state, &mut self.last_state);
    }

    pub fn toggle_pause(&mut self) {
        self.current_state.toggle_pause();
    }

    pub fn current_state(&self) -> GameState {
        self.current_state
    }
}
