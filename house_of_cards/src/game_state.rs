#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Alive,
    Dead,
    Paused,
    ChooseCard,
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
    pub current_state: GameState,
    last_states: Vec<GameState>,
}

impl GameStateManager {
    pub fn new() -> Self {
        Self {
            current_state: GameState::new(),
            last_states: Vec::new(),
        }
    }

    pub fn show_mouse(&self) -> bool {
        matches!(
            self.current_state,
            GameState::Paused | GameState::ChooseCard | GameState::PowerupCard
        )
    }

    pub fn next(&mut self, next_state: GameState) {
        self.last_states.push(self.current_state);
        self.current_state = next_state;
    }

    pub fn back(&mut self) {
        self.current_state = self.last_states.pop().unwrap_or(GameState::Alive);
    }

    pub fn toggle_pause(&mut self) {
        self.current_state.toggle_pause();
    }

    pub fn current_state(&self) -> GameState {
        self.current_state
    }
}
