use macroquad::prelude as mq;

use crate::{colors, consts, player, util, weapon, hitbox};

pub struct Enemy {
    pos: mq::Vec2, // tiles
    pub health: f32,
    max_health: f32,
    damage: f32,
    speed: f32,     // tiles per second
    direction: f32, // radians
    weapon: weapon::Weapon,
}

impl Enemy {
    pub fn new(
        pos: mq::Vec2,
        health: f32,
        damage: f32,
        speed: f32,
        weapon: weapon::Weapon,
    ) -> Self {
        Self {
            pos,
            health,
            max_health: health,
            damage,
            speed,
            direction: 0.0,
            weapon,
        }
    }

    pub fn update(&mut self, player: &player::Player, delta: f32) -> bool {
        let vec_to_player = player.pos - self.pos;
        let distance_to_player = vec_to_player.length();

        self.direction = vec_to_player.y.atan2(vec_to_player.x);
        let movement =
            mq::Vec2::new(self.direction.cos(), self.direction.sin()) * self.speed * delta;
        self.pos += movement;

        distance_to_player < self.weapon.range && self.weapon.try_shoot()
    }

    pub fn draw(&self, player: &player::Player, scale: f32) {
        let draw_pos = (self.pos - player.pos) * scale / consts::TILES_PER_SCALE as f32
            + mq::Vec2::new(mq::screen_width() / 2.0, mq::screen_height() / 2.0);

        // draw square rotated to point in direction of movement
        let square_raduis = scale * consts::ENEMY_SIZE;
        mq::draw_poly(
            draw_pos.x,
            draw_pos.y,
            4,
            square_raduis,
            util::rad_to_deg(self.direction + std::f32::consts::PI / 4.0), // rotate 45 degrees
            colors::NORD11,
        );
    }
}

impl hitbox::Rectangle for Enemy {
    fn position(&self) -> mq::Vec2 {
        self.pos
    }

    fn width(&self) -> f32 {
        consts::ENEMY_SIZE * 2.0 * consts::TILES_PER_SCALE as f32
    }

    fn height(&self) -> f32 {
        consts::ENEMY_SIZE * 2.0 * consts::TILES_PER_SCALE as f32
    }

    fn rotation(&self) -> f32 {
        self.direction
    }
}

pub struct EnemyManager {
    pub enemies: Vec<Enemy>,
    wave: i32,
    enemies_left_to_spawn: i32,  // not enemies.len()
    time_until_next_spawn: f32, // in seconds
}

impl EnemyManager {
    pub fn new() -> Self {
        Self {
            enemies: Vec::new(),
            wave: 0,
            enemies_left_to_spawn: 0,
            time_until_next_spawn: 0.0,
        }
    }

    pub fn update(&mut self, player: &player::Player, delta: f32) {
        self.enemies.retain(|enemy| enemy.health > 0.0);
        for enemy in self.enemies.iter_mut() {
            // if enemy.update(player, delta) {
            //     player.take_damage(enemy.damage);
            // }
            enemy.update(player, delta);
        }

        // self.enemies.retain(|enemy| enemy.health > 0.0);

        self.time_until_next_spawn -= delta;

        if self.enemies.is_empty() && self.enemies_left_to_spawn <= 0 {
            self.wave += 1;
            self.enemies_left_to_spawn = consts::ENEMY_WAVE_COUNTS(self.wave);
            self.time_until_next_spawn = 1.0 / consts::ENEMY_SPAWN_RATE;
        } else if self.time_until_next_spawn <= 0.0 && self.enemies_left_to_spawn > 0 {
            self.spawn_enemy(player);
            self.time_until_next_spawn = 1.0 / consts::ENEMY_SPAWN_RATE;
        }
    }

    fn spawn_enemy(&mut self, player: &player::Player) {
        let random_angle = mq::rand::gen_range(0.0, 2.0 * std::f32::consts::PI);
        let spawn_pos = mq::Vec2::new(random_angle.cos(), random_angle.sin())
            * consts::ENEMY_SPAWN_RADIUS / 5.0
            + player.pos;

        let enemy = Enemy::new(
            spawn_pos,
            self.wave as f32,
            self.wave as f32,
            consts::ENEMY_SPEED,
            consts::SWORD,
        );
        self.enemies.push(enemy);
        self.enemies_left_to_spawn -= 1;
    }

    pub fn draw(&self, player: &player::Player, scale: f32) {
        for enemy in self.enemies.iter() {
            enemy.draw(player, scale);
        }
    }
}
