use macroquad::prelude as mq;

use crate::{colors, weapon};

pub const WINDOW_START_SIZE: u32 = 800;

pub const BACKGROUND_COLOR: mq::Color = crate::colors::NORD1;
pub const MOUSE_COLOR: mq::Color = crate::colors::NORD6;
pub const BACKGROUND_COLORS: [mq::Color; 6] = colors::SURFACE_COLORS;

pub const TILES_PER_SCALE: u32 = 12;

pub const PLAYER_SPEED: f32 = 1.25; // tiles per second
pub const PLAYER_SIZE: f32 = 0.012; // scale
pub const PLAYER_MAX_HEALTH: f32 = 10.0;

pub const PLAYER_HP_BAR_WIDTH: f32 = 0.5; // scale
pub const PLAYER_HP_BAR_HEIGHT: f32 = 0.025; // scale
pub const PLAYER_HP_BAR_BOT_OFFSET: f32 = 0.05; // scale
pub const PLAYER_HP_BAR_THICKNESS: f32 = 0.0075; // scale

pub const CAMERA_FOLLOW_SPEED: f32 = 0.95;

pub const TIME_TO_MOUSE_IDLE: f32 = 2.5; // seconds

pub const CROSSHAIR_SIZE: f32 = 0.02; // scale
pub const CROSSHAIR_THICKNESS: f32 = 0.002; // scale

pub const DECK_SPACING_OUTSIDE: f32 = 0.01; // scale
pub const DECK_SPACING_INSIDE: f32 = 0.01; // scale
pub const DECK_WIDTH: f32 = 0.12; // scale
pub const DECK_HEIGHT: f32 = 0.16; // scale
pub const DECK_THICKNESS: f32 = 0.01; // scale

pub const DISCARD_ROTATION: f32 = 0.1;
pub const DISCARD_OFFSET: f32 = 0.05;
pub const DISCARD_TO_DRAW: usize = 5;

pub const FONT_SPACING: f32 = 0.01; // scale
pub const FONT_SIZE: f32 = 0.04; // scale

pub const LARGE_FONT_SIZE: f32 = 0.15; // scale
pub const LARGE_FONT_BOUNCE_MAX: f32 = 0.125; // font size
pub const LARGE_FONT_BOUNCE_SPEED: f32 = 0.8;

pub const AR: weapon::Weapon = weapon::Weapon::new(5.0, 2.5, 12.0, 1.0, 0.75, 7.5);

pub const BULLET_SIZE: f32 = 0.005; // scale
pub const BULLET_OUTLINE: f32 = 0.0005; // scale

pub const ENEMY_SIZE: f32 = 0.02; // scale
pub const ENEMY_SPEED: f32 = PLAYER_SPEED * 0.6; // tiles per second

pub const ENEMY_MELEE_RANGE: f32 = 0.5; // tiles
pub const ENEMY_MELEE_CHARGE_TIME: f32 = 0.5; // seconds
pub const ENEMY_MELEE_RELOAD_TIME: f32 = 1.0; // seconds

pub const ENEMY_RANGED_RANGE: f32 = 4.0; // tiles
pub const ENEMY_RANGED_MIN_RANGE: f32 = 2.0; // tiles
pub const ENEMY_RANGED_CHARGE_TIME: f32 = 0.75; // seconds
pub const ENEMY_RANGED_RELOAD_TIME: f32 = 4.0; // seconds
pub const ENEMY_RANGED_SPEED_PENALTY: f32 = 0.75; // percent

pub const ENEMY_RANGED_BULLET_SPEED: f32 = 4.0; // tiles per second
pub const ENEMT_RANGED_BULLET_RANGE: f32 = 12.0; // tiles

pub const ENEMY_MELEE_CHARGE_THICKNESS: f32 = 0.005; // scale

pub const ENEMY_SPAWN_RATE: f32 = 0.25; // enemies / seconds
pub const ENEMY_SPAWN_RADIUS: f32 = TILES_PER_SCALE as f32 + 2.0; // tiles
pub const ENEMY_RANGED_CHANCE: f32 = 0.33; // percent

// Note: wave spawning starts at 1
pub const ENEMY_WAVE_COUNT: fn(i32) -> i32 = |wave| 5 + 5 * (wave - 1);
pub const ENEMY_WAVE_SCORE: fn(i32) -> i32 = |wave| (wave - 1).pow(2);

pub const ENEMY_WAVE_HP: fn(i32) -> f32 = |wave| 5.0 + 2.0 * (wave - 1) as f32;
pub const ENEMY_WAVE_DAMAGE: fn(i32) -> f32 = |wave| 1.0 + 0.1 * (wave - 1) as f32;

pub const AUTO_RELOAD: bool = true;
