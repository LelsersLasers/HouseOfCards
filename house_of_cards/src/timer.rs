use crate::util;

pub struct Timer<T> {
    period: f32,
    time_until_next: f32,
    state: T,
}

impl<T: Default> Timer<T> {
    pub fn new(period: f32) -> Self {
        Self {
            period,
            time_until_next: 0.0,
            state: Default::default(),
        }
    }

    pub fn new_with_state(period: f32, state: T) -> Self {
        Self {
            period,
            time_until_next: 0.0,
            state,
        }
    }

    pub fn update_period(&mut self, period: f32) {
        self.period = period;
    }

    pub fn get_state(&self) -> &T {
        &self.state
    }

    pub fn update_state(&mut self, state: T) {
        self.state = state;
    }

    pub fn update(&mut self, delta: f32) -> util::Ticked {
        self.time_until_next -= delta;
        if self.time_until_next <= 0.0 {
            self.time_until_next += self.period;
            util::Ticked(true)
        } else {
            util::Ticked(false)
        }
    }
}
