use macroquad::prelude as mq;

use crate::{bullet, camera, colors, consts, hitbox, player, util, weapon};

pub struct MeleeAttack {
    time_until_next_attack: f32,
    time_in_range: f32,
}

impl MeleeAttack {
    fn new() -> Self {
        Self {
            time_until_next_attack: 0.0,
            time_in_range: 0.0,
        }
    }

    fn is_charging(&self) -> bool {
        self.time_in_range > 0.0 && self.time_until_next_attack <= 0.0
    }

    // fn can_attack(&self) -> bool {
    //     self.time_in_range >= consts::ENEMY_MELEE_CHARGE_TIME && self.time_until_next_attack <= 0.0
    // }

    fn update(&mut self, delta: f32) {
        self.time_until_next_attack -= delta;
    }
}

pub enum EnemyType {
    Melee(MeleeAttack),
    Ranged(weapon::Weapon),
}

pub struct Enemy {
    pos: mq::Vec2, // tiles
    pub health: f32,
    max_health: f32,
    damage: f32,
    speed: f32,     // tiles per second
    direction: f32, // radians
    enemy_type: EnemyType,
}

pub struct Shot(bool);

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
        }
    }

    pub fn update(&mut self, player: &mut player::Player, delta: f32) -> Shot {
        // returns shot

        let vec_to_player = player.pos - self.pos;
        let distance_to_player = vec_to_player.length();

        self.direction = vec_to_player.y.atan2(vec_to_player.x);
        let mut movement =
            mq::Vec2::new(self.direction.cos(), self.direction.sin()) * self.speed * delta;

        if let EnemyType::Ranged(ref mut weapon) = self.enemy_type {
            weapon.update(delta);
            movement *= weapon.get_ms_penalty();
        }
        self.pos += movement;

        match self.enemy_type {
            EnemyType::Melee(ref mut melee_attack) => {
                melee_attack.update(delta);
                if melee_attack.time_until_next_attack <= 0.0 {
                    if distance_to_player < consts::ENEMY_MELEE_RANGE {
                        melee_attack.time_in_range += delta;
                        if melee_attack.time_in_range >= consts::ENEMY_MELEE_CHARGE_TIME {
                            melee_attack.time_until_next_attack = consts::ENEMY_MELEE_RELOAD_TIME;
                            melee_attack.time_in_range = 0.0;

                            player.health -= self.damage;
                        }
                    } else {
                        melee_attack.time_in_range = 0.0;
                    }
                }

                Shot(false)
            }
            EnemyType::Ranged(ref mut weapon) => {
                Shot(distance_to_player < consts::ENEMY_RANGED_RANGE && weapon.try_shoot())
            }
        }
    }

    pub fn draw(&self, camera: &camera::Camera, player: &player::Player, scale: f32) {
        let draw_pos = (self.pos - camera.pos) * scale / consts::TILES_PER_SCALE as f32
            + mq::Vec2::new(mq::screen_width() / 2.0, mq::screen_height() / 2.0);

        // attack indicator
        match self.enemy_type {
            EnemyType::Melee(ref melee_attack) => {
                let draw_radius =
                    consts::ENEMY_MELEE_RANGE * scale / consts::TILES_PER_SCALE as f32;
                if melee_attack.is_charging() {
                    mq::draw_circle(draw_pos.x, draw_pos.y, draw_radius, colors::NORD6_BIG_ALPHA);

                    let charge_ratio =
                        1.0 - melee_attack.time_in_range / consts::ENEMY_MELEE_CHARGE_TIME;

                    mq::draw_circle_lines(
                        draw_pos.x,
                        draw_pos.y,
                        draw_radius * charge_ratio.clamp(0.0, 1.0),
                        consts::ENEMY_MELEE_CHARGE_THICKNESS * scale,
                        colors::NORD6_ALPHA,
                    );
                }
            }
            EnemyType::Ranged(_) => {
                let distance_to_player = player.pos.distance(self.pos);
                if distance_to_player < consts::ENEMY_RANGED_RANGE {
                    let draw_radius =
                        consts::ENEMY_RANGED_RANGE * scale / consts::TILES_PER_SCALE as f32;
                    mq::draw_circle(draw_pos.x, draw_pos.y, draw_radius, colors::NORD6_BIG_ALPHA);
                }
            }
        }

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

        // hp bar
        let hp_bar_ratio = self.health / self.max_health;
        let hp_bar_width = square_raduis * 2.0 * hp_bar_ratio;
        let hp_bar_height = square_raduis / 4.0;
        mq::draw_rectangle(
            draw_pos.x - hp_bar_width / 2.0,
            draw_pos.y - square_raduis - hp_bar_height,
            hp_bar_width,
            hp_bar_height,
            colors::NORD14,
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

pub struct EnemyManagerUpdateResult {
    pub wave_finished: bool,
    pub enemies_killed: i32,
}

pub struct EnemyManager {
    pub enemies: Vec<Enemy>,
    pub wave: i32,
    enemies_left_to_spawn: i32, // not enemies.len()
    time_until_next_spawn: f32, // in seconds
    enemy_bullets: Vec<bullet::Bullet>,
}

impl EnemyManager {
    pub fn new() -> Self {
        Self {
            enemies: Vec::new(),
            wave: 0,
            enemies_left_to_spawn: 0,
            time_until_next_spawn: 0.0,
            enemy_bullets: Vec::new(),
        }
    }

    pub fn enemies_left(&self) -> i32 {
        self.enemies_left_to_spawn + self.enemies.len() as i32
    }

    pub fn update(&mut self, player: &mut player::Player, delta: f32) -> EnemyManagerUpdateResult {
        let previous_enemy_count = self.enemies.len() as i32;
        self.enemies.retain(|enemy| enemy.health > 0.0);
        let enemies_killed = previous_enemy_count - self.enemies.len() as i32;

        for enemy in self.enemies.iter_mut() {
            let shot = enemy.update(player, delta);
            let enemy_weapon = if let EnemyType::Ranged(ref mut weapon) = enemy.enemy_type {
                weapon
            } else {
                continue;
            };
            if let Shot(true) = shot {
                let bullet = bullet::Bullet::new(
                    enemy.pos,
                    enemy.direction,
                    enemy_weapon.bullet_speed,
                    enemy_weapon.range,
                    bullet::BulletDamage::Standard(enemy.damage),
                );
                self.enemy_bullets.push(bullet);
            }
        }

        self.enemy_bullets
            .iter_mut()
            .for_each(|bullet| bullet.update(delta));

        self.enemy_bullets.retain(bullet::Bullet::should_keep);

        self.time_until_next_spawn -= delta;

        if self.enemies.is_empty() && self.enemies_left_to_spawn <= 0 {
            self.wave += 1;
            self.enemies_left_to_spawn = consts::ENEMY_WAVE_COUNT(self.wave);
            self.time_until_next_spawn = 1.0 / consts::ENEMY_SPAWN_RATE;
            return EnemyManagerUpdateResult {
                wave_finished: true,
                enemies_killed,
            };
        } else if self.time_until_next_spawn <= 0.0 && self.enemies_left_to_spawn > 0 {
            self.spawn_enemy(player);
            self.time_until_next_spawn = 1.0 / consts::ENEMY_SPAWN_RATE;
        }

        EnemyManagerUpdateResult {
            wave_finished: false,
            enemies_killed,
        }
    }

    fn spawn_enemy(&mut self, player: &player::Player) {
        let random_angle = mq::rand::gen_range(0.0, 2.0 * std::f32::consts::PI);
        let spawn_pos = mq::Vec2::new(random_angle.cos(), random_angle.sin())
            * consts::ENEMY_SPAWN_RADIUS
            + player.pos;

        let enemy_type = if mq::rand::gen_range(0.0, 1.0) < consts::ENEMY_RANGED_CHANCE {
            EnemyType::Melee(MeleeAttack::new())
        } else {
            EnemyType::Ranged(consts::ENEMY_WEAPON)
        };
        let enemy = Enemy::new(
            spawn_pos,
            consts::ENEMY_WAVE_HP(self.wave),
            consts::ENEMY_WAVE_DAMAGE(self.wave),
            consts::ENEMY_SPEED,
            enemy_type,
        );
        self.enemies.push(enemy);
        self.enemies_left_to_spawn -= 1;
    }

    pub fn draw(&self, camera: &camera::Camera, player: &player::Player, scale: f32) {
        for enemy in self.enemies.iter() {
            enemy.draw(camera, player, scale);
        }

        for bullet in self.enemy_bullets.iter() {
            bullet.draw(camera, scale);
        }
    }
}
