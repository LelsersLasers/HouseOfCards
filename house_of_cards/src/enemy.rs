use macroquad::prelude as mq;

use crate::{bullet, camera, colors, consts, damage_number, hitbox, player, timer, util};

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

#[derive(PartialEq, Eq)]
pub enum EnemyType {
    Melee,
    Ranged,
    Super,
}

impl EnemyType {
    fn range(&self) -> f32 {
        match self {
            EnemyType::Melee => consts::ENEMY_MELEE_RANGE,
            EnemyType::Ranged => consts::ENEMY_RANGED_RANGE,
            EnemyType::Super => consts::ENEMY_SUPER_RANGE,
        }
    }

    fn charge_time(&self) -> f32 {
        match self {
            EnemyType::Melee => consts::ENEMY_MELEE_CHARGE_TIME,
            EnemyType::Ranged => consts::ENEMY_RANGED_CHARGE_TIME,
            EnemyType::Super => -1.0,
        }
    }

    fn reload_time(&self) -> f32 {
        match self {
            EnemyType::Melee => consts::ENEMY_MELEE_RELOAD_TIME,
            EnemyType::Ranged => consts::ENEMY_RANGED_RELOAD_TIME,
            EnemyType::Super => -1.0,
        }
    }

    fn size(&self) -> f32 {
        match self {
            EnemyType::Melee => consts::ENEMY_SIZE,
            EnemyType::Ranged => consts::ENEMY_SIZE,
            EnemyType::Super => consts::ENEMY_SUPER_SIZE,
        }
    }

    fn is_melee(&self) -> bool {
        match self {
            EnemyType::Melee => true,
            EnemyType::Ranged => false,
            EnemyType::Super => false,
        }
    }
}

enum EnemyShotType {
    None,
    Standard,
    Spread,
}

enum EnemyMovementType {
    Chase,
    Predict(f32), // lead time
}

pub struct Enemy {
    pub pos: mq::Vec2, // tiles
    pub health: f32,
    max_health: f32,
    damage: f32,
    speed: f32,     // tiles per second
    direction: f32, // radians
    pub enemy_type: EnemyType,
    enemy_attack: EnemyAttack,
    pub enemy_stunned: EnemyStunned,
    enemy_movement: EnemyMovementType,
    pub id: usize,
}

impl Enemy {
    pub fn new(
        pos: mq::Vec2,
        health: f32,
        damage: f32,
        speed: f32,
        enemy_type: EnemyType,
        id: usize,
    ) -> Self {
        let enemy_movement = if enemy_type.is_melee()
            && mq::rand::gen_range(0.0, 1.0) < consts::ENEMY_MOVEMENT_PREDICT_CHANCE
        {
            let lead_time = mq::rand::gen_range(
                consts::ENEMY_MOVEMENT_PREDICT_LEAD_TIME_MIN,
                consts::ENEMY_MOVEMENT_PREDICT_LEAD_TIME_MAX,
            );
            EnemyMovementType::Predict(lead_time)
        } else {
            EnemyMovementType::Chase
        };

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
            enemy_movement,
            id,
        }
    }

    fn update(&mut self, player: &mut player::Player, wave: i32, max_dist: f32, delta: f32) -> (EnemyShotType, Option<damage_number::DamageNumber>) {
        self.enemy_stunned.update(delta);
        let mut damage_number = None;
        if self.enemy_stunned.is_stunned() {
            return (EnemyShotType::None, damage_number);
        }

        let player_target_pos = match self.enemy_movement {
            EnemyMovementType::Chase => player.pos,
            EnemyMovementType::Predict(lead_time) => {
                player.pos + player.movement * consts::PLAYER_SPEED * lead_time
            }
        };

        let vec_to_target = player_target_pos - self.pos;
        let vec_to_player = player.pos - self.pos;
        let distance_to_player = vec_to_player.length();


        let max_wrap = consts::ENEMY_MAX_RANGE_MULT * max_dist;
        if distance_to_player > max_wrap {
            let wrap = vec_to_player.normalize_or_zero() * consts::ENEMY_WRAP_STRENGH * max_wrap;
            self.pos += wrap;
            return (EnemyShotType::None, damage_number);
        }
        
        self.direction = vec_to_target.y.atan2(vec_to_target.x);
        let mut movement =
            mq::Vec2::new(self.direction.cos(), self.direction.sin()) * self.speed * delta;

        let mut enemy_shot_type = EnemyShotType::None;

        let range = self.enemy_type.range();
        let charge_time = self.enemy_type.charge_time();
        let reload_time = self.enemy_type.reload_time();

        self.enemy_attack.update(delta);

        if self.enemy_attack.time_until_next_attack > 0.0 {
            self.enemy_attack.time_until_next_attack -= delta;
        } else {
            match self.enemy_type {
                EnemyType::Ranged => {
                    if self.enemy_attack.time_in_range > 0.0 {
                        movement = mq::Vec2::ZERO;
                        self.enemy_attack.time_in_range += delta;
                        if self.enemy_attack.time_in_range >= charge_time {
                            self.enemy_attack.time_until_next_attack = reload_time;
                            self.enemy_attack.time_in_range = 0.0;

                            enemy_shot_type = EnemyShotType::Standard;
                        }
                    } else if distance_to_player < consts::ENEMY_RANGED_RANGE {
                        self.enemy_attack.time_in_range += delta;
                    }
                }
                EnemyType::Melee => {
                    if distance_to_player < range {
                        self.enemy_attack.time_in_range += delta;
                        if self.enemy_attack.time_in_range >= charge_time {
                            self.enemy_attack.time_until_next_attack = reload_time;
                            self.enemy_attack.time_in_range = 0.0;

                            player.health -= self.damage;
                            damage_number = Some(damage_number::DamageNumber::new(
                                format!("-{}", self.damage).to_owned(),
                                consts::DAMAGE_NUMBER_TIME,
                                player.pos,
                                damage_number::DamageNumberColor::PlayerDamage
                            ));
                        }
                    } else {
                        self.enemy_attack.time_in_range = 0.0;
                    }
                }
                EnemyType::Super => {
                    if distance_to_player < range && self.enemy_attack.time_until_next_attack <= 0.0
                    {
                        self.enemy_attack.time_until_next_attack =
                            1.0 / consts::ENEMY_SUPER_WAVE_FIRE_RATE(wave);

                        enemy_shot_type = EnemyShotType::Spread;
                    }
                }
            };
        }

        match self.enemy_type {
            EnemyType::Ranged => {
                if distance_to_player < consts::ENEMY_RANGED_RANGE {
                    movement = mq::Vec2::ZERO;
                } else {
                    movement *= consts::ENEMY_RANGED_SPEED_PENALTY;
                }
            }
            EnemyType::Super => {
                if distance_to_player < consts::ENEMY_SUPER_MIN_RANGE {
                    movement = mq::Vec2::ZERO;
                }
            }
            _ => {}
        }

        self.pos += movement;

        (enemy_shot_type, damage_number)
    }

    pub fn draw_hp_bar(&self, camera: &camera::Camera, scale: f32) {
        let draw_pos = (self.pos - camera.pos) * scale / consts::TILES_PER_SCALE as f32
            + mq::Vec2::new(mq::screen_width() / 2.0, mq::screen_height() / 2.0);
        let square_radius = scale * self.enemy_type.size();
        let square_radius_small = scale * EnemyType::Melee.size();

        let hp_bar_ratio = self.health / self.max_health;
        let hp_bar_width = square_radius_small * 2.0 * hp_bar_ratio;
        let hp_bar_height = square_radius_small / 4.0;
        mq::draw_rectangle(
            draw_pos.x - hp_bar_width / 2.0,
            draw_pos.y - square_radius * 0.75 - hp_bar_height,
            hp_bar_width,
            hp_bar_height,
            colors::NORD14,
        );
    }

    pub fn draw(&self, camera: &camera::Camera, chess_texture: &mq::Texture2D, scale: f32) {
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

        let texture_info = consts::CHESS_TEXTURE_INFO[match self.enemy_type {
            EnemyType::Melee => consts::CHESS_PAWN_INDEX,
            EnemyType::Ranged => consts::CHESS_BISHOP_INDEX,
            EnemyType::Super => consts::CHESS_QUEEN_INDEX,
        }];
        let texture_source = mq::Rect::new(
            texture_info.0 as f32,
            texture_info.1 as f32,
            texture_info.2 as f32,
            texture_info.3 as f32,
        );

        let square_radius = scale * self.enemy_type.size();

        mq::draw_texture_ex(
            chess_texture,
            draw_pos.x - square_radius / 2.0,
            draw_pos.y - square_radius / 2.0,
            mq::WHITE,
            mq::DrawTextureParams {
                dest_size: Some(mq::Vec2::splat(square_radius)),
                source: Some(texture_source),
                ..Default::default()
            },
        );

        // draw square rotated to point in direction of movement
        // mq::draw_poly(
        //     draw_pos.x,
        //     draw_pos.y,
        //     4,
        //     square_radius,
        //     util::rad_to_deg(self.direction + std::f32::consts::PI / 4.0), // rotate 45 degrees
        //     match self.enemy_type {
        //         EnemyType::Melee => colors::NORD11,
        //         EnemyType::Ranged => colors::NORD12,
        //         EnemyType::Super => colors::NORD15,
        //     },
        // );

        if self.enemy_stunned.is_stunned() {
            // mq::draw_poly_lines(
            //     draw_pos.x,
            //     draw_pos.y,
            //     4,
            //     square_radius,
            //     util::rad_to_deg(self.direction + std::f32::consts::PI / 4.0), // rotate 45 degrees
            //     consts::ENEMY_STUNNED_THICKNESS * scale,
            //     colors::NORD10,
            // );
            mq::draw_rectangle_lines(
                draw_pos.x - square_radius / 2.0,
                draw_pos.y - square_radius / 2.0,
                square_radius,
                square_radius,
                consts::ENEMY_STUNNED_THICKNESS * scale,
                colors::NORD10,
            );
        }
    }
}

impl hitbox::Circle for Enemy {
    fn center(&self) -> mq::Vec2 {
        self.pos
    }

    fn radius(&self) -> f32 {
        match self.enemy_type {
            EnemyType::Super => consts::ENEMY_SUPER_SIZE * consts::TILES_PER_SCALE as f32,
            _ => consts::ENEMY_SIZE * consts::TILES_PER_SCALE as f32,
        }
    }

}

pub struct EnemiesKilled {
    pub count: i32,
    pub super_killed: bool,
}

pub struct EnemyManager {
    pub enemies: Vec<Enemy>,
    pub wave: i32,                // used internally to calculate enemy stats
    enemies_until_next_wave: i32, // not enemies.len()
    spawn_timer: timer::Timer<()>,
    enemy_bullets: Vec<bullet::Bullet>,
    should_spawn_super: bool,
    next_enemy_id: usize,
}

impl EnemyManager {
    pub fn new() -> Self {
        Self {
            enemies: Vec::new(),
            wave: 0,
            enemies_until_next_wave: 0,
            spawn_timer: timer::Timer::new(1.0 / consts::ENEMY_WAVE_SPAWN_RATE(0)),
            enemy_bullets: Vec::new(),
            should_spawn_super: false,
            next_enemy_id: 0,
        }
    }

    pub fn update(&mut self, player: &mut player::Player, max_dist: f32, delta: f32) -> (EnemiesKilled, Vec<damage_number::DamageNumber>) {
        let previous_enemy_count = self.enemies.len() as i32;
        let super_count = self
            .enemies
            .iter()
            .filter(|enemy| enemy.enemy_type == EnemyType::Super)
            .count();

        self.enemies.retain(|enemy| enemy.health > 0.0);

        let count = previous_enemy_count - self.enemies.len() as i32;
        let super_killed = super_count
            > self
                .enemies
                .iter()
                .filter(|enemy| enemy.enemy_type == EnemyType::Super)
                .count();

        let mut damage_numbers = Vec::new();

        for enemy in self.enemies.iter_mut() {
            let (enemy_shot_type, damage_number) = enemy.update(player, self.wave, max_dist, delta);

            if let Some(damage_number) = damage_number {
                damage_numbers.push(damage_number);
            }

            match enemy_shot_type {
                EnemyShotType::None => {}
                EnemyShotType::Standard => {
                    let bullet = bullet::Bullet::new(
                        enemy.pos,
                        enemy.direction,
                        consts::ENEMY_RANGED_BULLET_SPEED,
                        consts::ENEMY_RANGED_BULLET_RANGE,
                        bullet::BulletDamage::Standard(enemy.damage),
                        1,
                    );
                    self.enemy_bullets.push(bullet);
                }
                EnemyShotType::Spread => {
                    let spread = consts::ENEMY_SUPER_SPREAD;
                    let angle = enemy.direction + mq::rand::gen_range(-spread, spread);
                    let bullet = bullet::Bullet::new(
                        enemy.pos,
                        angle,
                        consts::ENEMY_RANGED_BULLET_SPEED,
                        consts::ENEMY_RANGED_BULLET_RANGE,
                        bullet::BulletDamage::Standard(enemy.damage),
                        1,
                    );
                    self.enemy_bullets.push(bullet);
                }
            }
        }

        self.enemy_bullets
            .iter_mut()
            .for_each(|bullet| bullet.update(delta));

        for bullet in self.enemy_bullets.iter_mut() {
            if hitbox::circles_collide(bullet, player) {
                let damage = match bullet.bullet_damage {
                    bullet::BulletDamage::Standard(damage) => damage,
                    bullet::BulletDamage::Card(card) => card.damage(None),
                };
                player.health -= damage;

                damage_numbers.push(damage_number::DamageNumber::new(
                    format!("-{}", damage).to_owned(),
                    consts::DAMAGE_NUMBER_TIME,
                    player.pos,
                    damage_number::DamageNumberColor::PlayerDamage
                ));
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
            } else if self.enemies_until_next_wave == consts::ENEMY_WAVE_COUNT(self.wave) / 2
                && self.wave > consts::ENEMY_SUPER_WAVE_START
            {
                self.should_spawn_super = true;
            }

            self.spawn_enemy(player);
        }

        (EnemiesKilled {
            count,
            super_killed,
        },
        damage_numbers)
    }

    fn spawn_enemy(&mut self, player: &player::Player) {
        let random_angle = mq::rand::gen_range(0.0, 2.0 * std::f32::consts::PI);
        let spawn_pos = mq::Vec2::new(random_angle.cos(), random_angle.sin())
            * consts::ENEMY_SPAWN_RADIUS
            + player.pos;

        let enemy_type = if self.should_spawn_super {
            self.should_spawn_super = false;
            EnemyType::Super
        } else if mq::rand::gen_range(0.0, 1.0) < consts::ENEMY_RANGED_CHANCE {
            EnemyType::Ranged
        } else {
            EnemyType::Melee
        };

        let mut hp = consts::ENEMY_WAVE_HP(self.wave);
        if enemy_type == EnemyType::Super {
            hp *= consts::ENEMY_SUPER_HP_MOD(self.wave);
        }

        let enemy = Enemy::new(
            spawn_pos,
            hp,
            consts::ENEMY_DAMAGE,
            consts::ENEMY_WAVE_SPEED(self.wave),
            enemy_type,
            self.next_enemy_id,
        );
        self.next_enemy_id += 1;

        self.enemies_until_next_wave -= 1;
        self.enemies.push(enemy);
    }

    pub fn draw(&self, camera: &camera::Camera, chess_texture: &mq::Texture2D, scale: f32) {
        for enemy in self.enemies.iter() {
            enemy.draw(camera, chess_texture, scale);
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
