use macroquad::prelude as mq;

use crate::{bullet, camera, colors, consts, hitbox, player, timer, util};

pub struct EnemyStunned {
    pub time_remaining: f32,
}

impl EnemyStunned {
    pub fn new() -> Self {
        Self {
            time_remaining: 0.0,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.time_remaining -= delta;
        self.time_remaining = self.time_remaining.max(0.0);
    }

    pub fn is_stunned(&self) -> bool {
        self.time_remaining > 0.0
    }
}

pub struct EnemyAttack {
    time_until_next_attack: f32,
    time_in_range: f32,
}

impl EnemyAttack {
    fn new() -> Self {
        Self {
            time_until_next_attack: 0.0,
            time_in_range: 0.0,
        }
    }

    fn is_charging(&self) -> bool {
        self.time_in_range > 0.0 && self.time_until_next_attack <= 0.0
    }

    fn update(&mut self, delta: f32) {
        self.time_until_next_attack -= delta;
    }
}

pub enum EnemyType {
    Melee,
    Ranged,
}

impl EnemyType {
    fn range(&self) -> f32 {
        match self {
            EnemyType::Melee => consts::ENEMY_MELEE_RANGE,
            EnemyType::Ranged => consts::ENEMY_RANGED_RANGE,
        }
    }

    fn charge_time(&self) -> f32 {
        match self {
            EnemyType::Melee => consts::ENEMY_MELEE_CHARGE_TIME,
            EnemyType::Ranged => consts::ENEMY_RANGED_CHARGE_TIME,
        }
    }

    fn reload_time(&self) -> f32 {
        match self {
            EnemyType::Melee => consts::ENEMY_MELEE_RELOAD_TIME,
            EnemyType::Ranged => consts::ENEMY_RANGED_RELOAD_TIME,
        }
    }

    fn is_melee(&self) -> bool {
        match self {
            EnemyType::Melee => true,
            EnemyType::Ranged => false,
        }
    }
}

pub struct Enemy {
    pos: mq::Vec2, // tiles
    pub health: f32,
    max_health: f32,
    damage: f32,
    speed: f32,     // tiles per second
    direction: f32, // radians
    enemy_type: EnemyType,
    enemy_attack: EnemyAttack,
    pub enemy_stunned: EnemyStunned,
}

impl Enemy {
    pub fn new(pos: mq::Vec2, health: f32, damage: f32, speed: f32, enemy_type: EnemyType) -> Self {
        Self {
            pos,
            health,
            max_health: health,
            damage,
            speed,
            direction: 0.0,
            enemy_type,
            enemy_attack: EnemyAttack::new(),
            enemy_stunned: EnemyStunned::new(),
        }
    }

    pub fn update(&mut self, player: &mut player::Player, delta: f32) -> util::Shot {
        self.enemy_stunned.update(delta);
        if self.enemy_stunned.is_stunned() {
            return util::Shot(false);
        }

        let vec_to_player = player.pos - self.pos;
        let distance_to_player = vec_to_player.length();

        self.direction = vec_to_player.y.atan2(vec_to_player.x);
        let mut movement =
            mq::Vec2::new(self.direction.cos(), self.direction.sin()) * self.speed * delta;

        let mut shot = util::Shot(false);

        self.enemy_attack.update(delta);

        if self.enemy_attack.time_until_next_attack > 0.0 {
            self.enemy_attack.time_until_next_attack -= delta;
        } else {
            match self.enemy_type {
                EnemyType::Ranged => {
                    if self.enemy_attack.time_in_range > 0.0 {
                        movement = mq::Vec2::ZERO;
                        self.enemy_attack.time_in_range += delta;
                        let charge_time = self.enemy_type.charge_time();
                        if self.enemy_attack.time_in_range >= charge_time {
                            let reload_time = self.enemy_type.reload_time();
                            self.enemy_attack.time_until_next_attack = reload_time;
                            self.enemy_attack.time_in_range = 0.0;

                            shot = util::Shot(true);
                        }
                    } else if distance_to_player < consts::ENEMY_RANGED_RANGE {
                        self.enemy_attack.time_in_range += delta;
                    }
                }
                EnemyType::Melee => {
                    let range = self.enemy_type.range();
                    if distance_to_player < range {
                        self.enemy_attack.time_in_range += delta;
                        let charge_time = self.enemy_type.charge_time();
                        if self.enemy_attack.time_in_range >= charge_time {
                            let reload_time = self.enemy_type.reload_time();
                            self.enemy_attack.time_until_next_attack = reload_time;
                            self.enemy_attack.time_in_range = 0.0;

                            player.health -= self.damage;
                        }
                    } else {
                        self.enemy_attack.time_in_range = 0.0;
                    }
                }
            };
        }

        if !self.enemy_type.is_melee() {
            if distance_to_player < consts::ENEMY_RANGED_RANGE {
                movement = mq::Vec2::ZERO;
            } else {
                movement *= consts::ENEMY_RANGED_SPEED_PENALTY;
            }
        }

        self.pos += movement;

        shot
    }

    pub fn draw_hp_bar(&self, camera: &camera::Camera, scale: f32) {
        let draw_pos = (self.pos - camera.pos) * scale / consts::TILES_PER_SCALE as f32
            + mq::Vec2::new(mq::screen_width() / 2.0, mq::screen_height() / 2.0);
        let square_radius = scale * consts::ENEMY_SIZE;

        let hp_bar_ratio = self.health / self.max_health;
        let hp_bar_width = square_radius * 2.0 * hp_bar_ratio;
        let hp_bar_height = square_radius / 4.0;
        mq::draw_rectangle(
            draw_pos.x - hp_bar_width / 2.0,
            draw_pos.y - square_radius - hp_bar_height,
            hp_bar_width,
            hp_bar_height,
            colors::NORD14,
        );
    }

    pub fn draw(&self, camera: &camera::Camera, scale: f32) {
        let draw_pos = (self.pos - camera.pos) * scale / consts::TILES_PER_SCALE as f32
            + mq::Vec2::new(mq::screen_width() / 2.0, mq::screen_height() / 2.0);

        // attack indicator
        if self.enemy_type.is_melee() {
            let draw_radius = consts::ENEMY_MELEE_RANGE * scale / consts::TILES_PER_SCALE as f32;
            if self.enemy_attack.is_charging() {
                mq::draw_circle(draw_pos.x, draw_pos.y, draw_radius, colors::NORD6_BIG_ALPHA);

                let charge_ratio =
                    1.0 - self.enemy_attack.time_in_range / consts::ENEMY_MELEE_CHARGE_TIME;

                mq::draw_circle_lines(
                    draw_pos.x,
                    draw_pos.y,
                    draw_radius * charge_ratio.clamp(0.0, 1.0),
                    consts::ENEMY_MELEE_CHARGE_THICKNESS * scale,
                    colors::NORD6_ALPHA,
                );
            }
        }

        // draw square rotated to point in direction of movement
        let square_radius = scale * consts::ENEMY_SIZE;
        mq::draw_poly(
            draw_pos.x,
            draw_pos.y,
            4,
            square_radius,
            util::rad_to_deg(self.direction + std::f32::consts::PI / 4.0), // rotate 45 degrees
            match self.enemy_type {
                EnemyType::Melee => colors::NORD11,
                EnemyType::Ranged => colors::NORD12,
            },
        );

        if self.enemy_stunned.is_stunned() {
            mq::draw_poly_lines(
                draw_pos.x,
                draw_pos.y,
                4,
                square_radius,
                util::rad_to_deg(self.direction + std::f32::consts::PI / 4.0), // rotate 45 degrees
                consts::ENEMY_STUNNED_THICKNESS * scale,
                colors::NORD10,
            );
        }
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
    pub wave: i32,                // used internally to calculate enemy stats
    enemies_until_next_wave: i32, // not enemies.len()
    spawn_timer: timer::Timer<()>,
    enemy_bullets: Vec<bullet::Bullet>,
}

impl EnemyManager {
    pub fn new() -> Self {
        Self {
            enemies: Vec::new(),
            wave: 0,
            enemies_until_next_wave: 0,
            spawn_timer: timer::Timer::new(1.0 / consts::ENEMY_WAVE_SPAWN_RATE(0)),
            enemy_bullets: Vec::new(),
        }
    }

    pub fn update(&mut self, player: &mut player::Player, delta: f32) -> util::EnemiesKilled {
        let previous_enemy_count = self.enemies.len() as i32;
        self.enemies.retain(|enemy| enemy.health > 0.0);
        let enemies_killed = util::EnemiesKilled(previous_enemy_count - self.enemies.len() as i32);

        for enemy in self.enemies.iter_mut() {
            let shot = enemy.update(player, delta);

            if let util::Shot(true) = shot {
                let bullet = bullet::Bullet::new(
                    enemy.pos,
                    enemy.direction,
                    consts::ENEMY_RANGED_BULLET_SPEED,
                    consts::ENEMT_RANGED_BULLET_RANGE,
                    bullet::BulletDamage::Standard(enemy.damage),
                    1,
                );
                self.enemy_bullets.push(bullet);
            }
        }

        self.enemy_bullets
            .iter_mut()
            .for_each(|bullet| bullet.update(delta));

        for bullet in self.enemy_bullets.iter_mut() {
            if hitbox::circles_collide(bullet, player) {
                player.health -= match bullet.bullet_damage {
                    bullet::BulletDamage::Standard(damage) => damage,
                    bullet::BulletDamage::Card(card) => card.damage(None),
                };

                bullet.remove();
            }
        }

        self.enemy_bullets.retain(bullet::Bullet::should_keep);

        if let util::Ticked(true) = self.spawn_timer.update(delta) {
            if self.enemies_until_next_wave <= 0 {
                self.wave += 1;
                self.enemies_until_next_wave = consts::ENEMY_WAVE_COUNT(self.wave);
                self.spawn_timer
                    .update_period(1.0 / consts::ENEMY_WAVE_SPAWN_RATE(self.wave));
            }

            self.enemies_until_next_wave -= 1;

            self.spawn_enemy(player);
        }

        enemies_killed
    }

    fn spawn_enemy(&mut self, player: &player::Player) {
        let random_angle = mq::rand::gen_range(0.0, 2.0 * std::f32::consts::PI);
        let spawn_pos = mq::Vec2::new(random_angle.cos(), random_angle.sin())
            * consts::ENEMY_SPAWN_RADIUS
            + player.pos;

        let enemy_type = if mq::rand::gen_range(0.0, 1.0) < consts::ENEMY_RANGED_CHANCE {
            EnemyType::Ranged
        } else {
            EnemyType::Melee
        };
        let enemy = Enemy::new(
            spawn_pos,
            consts::ENEMY_WAVE_HP(self.wave),
            consts::ENEMY_WAVE_DAMAGE(self.wave),
            consts::ENEMY_SPEED,
            enemy_type,
        );
        self.enemies.push(enemy);
    }

    pub fn draw(&self, camera: &camera::Camera, scale: f32) {
        for enemy in self.enemies.iter() {
            enemy.draw(camera, scale);
        }

        for bullet in self.enemy_bullets.iter() {
            bullet.draw(camera, scale);
        }
    }

    pub fn draw_hp_bars(&self, camera: &camera::Camera, scale: f32) {
        for enemy in self.enemies.iter() {
            enemy.draw_hp_bar(camera, scale);
        }
    }
}
