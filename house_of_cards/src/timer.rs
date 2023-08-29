pub struct Timer<T> {
    period: f32,
    last: f32,
    state: T,
}

impl<T: Default> Timer<T> {
    pub fn new(period: f32) -> Self {
        Self {
            period,
            last: 0.0,
            state: Default::default(),
        }
    }

    pub fn new_with_state(period: f32, state: T) -> Self {
        Self {
            period,
            last: 0.0,
            state,
        }
    }

    pub fn get_state(&self) -> &T {
        &self.state
    }

    pub fn update_state(&mut self, state: T) {
        self.state = state;
    }

    pub fn update(&mut self, ticks: f32) -> bool {
        if self.last + self.period <= ticks {
            self.last += self.period;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.last = 0.0;
    }
}
