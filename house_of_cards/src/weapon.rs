pub struct Weapon {
    fire_rate: f32, // shots per second
    range: f32,     // scale
    movement_speed_modifier: f32,
    time_since_last_shot: f32,
}

impl Weapon {
    // NOTE: `const fn`!!!
    pub const fn new(fire_rate: f32, range: f32, movement_speed_modifier: f32) -> Self {
        Self {
            fire_rate,
            range,
            movement_speed_modifier,
            time_since_last_shot: 0.0,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.time_since_last_shot += delta;
    }

    pub fn can_shoot(&self) -> bool {
        self.time_since_last_shot >= 1.0 / self.fire_rate
    }

    pub fn try_shoot(&mut self) -> bool {
        if self.can_shoot() {
            self.time_since_last_shot = 0.0;
            true
        } else {
            false
        }
    }
}
