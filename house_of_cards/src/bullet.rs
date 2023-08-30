use macroquad::prelude as mq;

use crate::{camera, colors, consts, deck, hitbox, powerup};

pub enum BulletDamage {
    Standard(f32),
    Card(deck::Card),
}

pub struct BulletHitResult {
    pub damage: f32,
    pub stun_time: f32,
    pub heal_amount: f32,
}

pub struct Bullet {
    start_pos: mq::Vec2,     // in tiles
    pos: mq::Vec2,           // in tiles
    direction: f32,          // in radians
    speed: f32,              // in tiles per second
    distance_to_travel: f32, // in tiles
    pub bullet_damage: BulletDamage,
    hp: i32,
}

impl Bullet {
    pub fn new(
        start_pos: mq::Vec2,
        direction: f32,
        speed: f32,
        distance_to_travel: f32,
        bullet_damage: BulletDamage,
        hp: i32,
    ) -> Self {
        Self {
            start_pos,
            pos: start_pos,
            direction,
            speed,
            distance_to_travel,
            bullet_damage,
            hp,
        }
    }

    pub fn should_keep(&self) -> bool {
        self.hp > 0
    }

    pub fn hit(&mut self) {
        self.hp -= 1;
    }

    pub fn remove(&mut self) {
        self.hp = 0;
    }

    pub fn hit_result(&mut self, powerups: &powerup::Powerups) -> BulletHitResult {
        match self.bullet_damage {
            BulletDamage::Standard(damage) => BulletHitResult {
                damage,
                stun_time: 0.0,
                heal_amount: 0.0,
            },
            BulletDamage::Card(card) => BulletHitResult {
                damage: card.damage(Some(powerups)),
                stun_time: card.stun_time(powerups),
                heal_amount: card.heal_amount(powerups),
            },
        }
    }

    pub fn update(&mut self, delta: f32) {
        let direction_vec = mq::Vec2::new(self.direction.cos(), self.direction.sin());
        self.pos += direction_vec * self.speed * delta;
        let distance_traveled = self.pos.distance(self.start_pos);
        if distance_traveled >= self.distance_to_travel {
            self.remove();
        }
    }

    pub fn draw(&self, camera: &camera::Camera, scale: f32) {
        let draw_pos = (self.pos - camera.pos) * scale / consts::TILES_PER_SCALE as f32
            + mq::Vec2::new(mq::screen_width() / 2.0, mq::screen_height() / 2.0);
        mq::draw_circle(
            draw_pos.x,
            draw_pos.y,
            scale * consts::BULLET_SIZE,
            match self.bullet_damage {
                BulletDamage::Standard(_) => colors::NORD8,
                BulletDamage::Card(card) => {
                    if card.is_red() {
                        colors::NORD11
                    } else {
                        colors::NORD0
                    }
                }
            },
        );
        mq::draw_circle_lines(
            draw_pos.x,
            draw_pos.y,
            scale * consts::BULLET_SIZE,
            scale * consts::BULLET_OUTLINE,
            colors::NORD4,
        )
    }
}

impl hitbox::Circle for Bullet {
    fn center(&self) -> mq::Vec2 {
        self.pos
    }

    fn radius(&self) -> f32 {
        consts::BULLET_SIZE * consts::TILES_PER_SCALE as f32
    }
}
