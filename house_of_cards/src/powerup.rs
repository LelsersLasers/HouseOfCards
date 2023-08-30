use macroquad::prelude as mq;

use crate::consts;

#[derive(PartialEq, Eq)]
pub enum Powerup {
    Damage,
    Health,
    Reload,
    Speed,
    Diamonds,
    Hearts,
    Clubs,
    Spades,
}

pub struct Powerups {
    pub powerups: Vec<Powerup>,
}

impl Powerups {
    pub fn new() -> Self {
        Self {
            powerups: Vec::new(),
        }
    }

    pub fn add(&mut self, powerup: Powerup) {
        self.powerups.push(powerup);
    }

    pub fn count(&self, powerup: &Powerup) -> usize {
        self.powerups.iter().filter(|p| **p == *powerup).count()
    }

    pub fn damage_add(&self) -> f32 {
        self.count(&Powerup::Damage) as f32 * consts::DAMAGE_ADD
    }

    pub fn health_add(&self) -> f32 {
        self.count(&Powerup::Health) as f32 * consts::HEALTH_ADD
    }

    pub fn reload_mod(&self) -> f32 {
        (1.0 - consts::RELOAD_MOD).powi(self.count(&Powerup::Reload) as i32)
    }

    pub fn speed_mod(&self) -> f32 {
        self.count(&Powerup::Speed) as f32 * consts::SPEED_MOD + 1.0
    }

    pub fn diamonds_bullet_hp(&self) -> i32 {
        self.count(&Powerup::Diamonds) as i32 + 1
    }

    pub fn hearts_heal_amount(&self) -> f32 {
        let count = self.count(&Powerup::Hearts);
        let mut amount = 0.0;
        for _ in 0..count {
            if mq::rand::gen_range(0.0, 1.0) < consts::HEARTS_HEAL_CHANCE {
                amount += 1.0;
            }
        }

        amount
    }

    pub fn clubs_stun_time(&self) -> f32 {
        self.count(&Powerup::Clubs) as f32 * consts::CLUBS_STUN_TIME
    }

    pub fn spades_damage_mod(&self) -> f32 {
        let count = self.count(&Powerup::Spades);
        let mut modifier = 1.0;
        for _ in 0..count {
            if mq::rand::gen_range(0.0, 1.0) < consts::SPADES_DAMAGE_CHANCE {
                modifier *= 2.0;
            }
        }

        modifier
    }
}
