pub struct Weapon {
    fire_rate: f32,   // shots per second
    reload_time: f32, // seconds
    reloading: bool,
    pub range: f32,                   // tiles
    ms_modifier_walking: f32,
    ms_modifier_shooting: f32,
    
    time_until_next_shot: f32,        // seconds
    pub bullet_speed: f32,            // tiles per second

}

impl Weapon {
    // NOTE: `const fn`!!!
    pub const fn new(
        fire_rate: f32,
        reload_time: f32,
        range: f32,
        ms_modifier_walking: f32,
        ms_modifier_shooting: f32,
        bullet_speed: f32,
    ) -> Self {
        Self {
            fire_rate,
            reload_time,
            reloading: false,
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

    pub fn reload(&mut self) {
        self.time_until_next_shot = self.reload_time;
        self.reloading = true;
    }

    pub fn update(&mut self, delta: f32) {
        self.time_until_next_shot -= delta;
        if self.time_until_next_shot <= 0.0 {
            self.reloading = false;
            self.time_until_next_shot = 0.0;
        }
    }

    pub fn can_shoot(&self) -> bool {
        self.time_until_next_shot <= 0.0
    }

    pub fn is_reloading(&self) -> bool {
        self.reloading
    }

    pub fn try_shoot(&mut self) -> bool {
        if self.can_shoot() {
            self.time_until_next_shot = 1.0 / self.fire_rate;
            true
        } else {
            false
        }
    }
}
