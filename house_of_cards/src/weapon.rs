use crate::util;

pub struct Weapon {
    pub fire_rate: f32, // shots per second
    pub range: f32,     // tiles
    ms_modifier_walking: f32,
    ms_modifier_shooting: f32,

    pub time_until_next_shot: f32, // seconds
    pub bullet_speed: f32,         // tiles per second
}

impl Weapon {
    // NOTE: `const fn`!!!
    pub const fn new(
        fire_rate: f32,
        range: f32,
        ms_modifier_walking: f32,
        ms_modifier_shooting: f32,
        bullet_speed: f32,
    ) -> Self {
        Self {
            fire_rate,
            range,
            ms_modifier_walking,
            ms_modifier_shooting,
            time_until_next_shot: 0.0,
            bullet_speed,
        }
    }

    pub fn get_ms_penalty(&self) -> f32 {
        if self.can_shoot() {
            self.ms_modifier_walking
        } else {
            self.ms_modifier_shooting
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.time_until_next_shot -= delta;
        if self.time_until_next_shot <= 0.0 {
            self.time_until_next_shot = 0.0;
        }
    }

    pub fn can_shoot(&self) -> bool {
        self.time_until_next_shot <= 0.0
    }

    pub fn try_shoot(&mut self) -> util::Shot {
        if self.can_shoot() {
            self.time_until_next_shot = 1.0 / self.fire_rate;
            util::Shot(true)
        } else {
            util::Shot(false)
        }
    }
}
