use macroquad::prelude as mq;

use crate::{colors, weapon};

pub const WINDOW_START_SIZE: u32 = 800;

pub const MAX_DELTA: f32 = 1.0 / 10.0; // seconds

pub const CARDS_TEXTURE_PATH: &str = "resources/nord-cards-transparent.png";
pub const CARD_PX_WIDTH: f32 = 132.0;
pub const CARD_PX_HEIGHT: f32 = 180.0;
pub const CARD_PX_SPACING: f32 = 8.0;

pub const FONT_PATH: &str = "resources/Assistant-SemiBold.ttf";
pub const MUSIC_PATH: &str = "resources/INTERSTELLAR-SHORT.wav";

pub const BACKGROUND_COLOR: mq::Color = crate::colors::CRUST;
pub const MOUSE_COLOR: mq::Color = crate::colors::NORD6;
pub const BACKGROUND_COLORS: [mq::Color; 6] = colors::SURFACE_COLORS;

pub const TILES_PER_SCALE: u32 = 12;

pub const JOYSTICK_MAX_RADIUS: f32 = 0.175; // scale
pub const JOYSTICK_BALL_SIZE: f32 = 0.05; // scale
pub const JOYSTICK_THICKNESS: f32 = 0.01; // scale
pub const JOYSTICK_HEIGHT: f32 = 2.0 / 3.0; // height

pub const PAUSE_BUTTON_WIDTH: f32 = 0.2;
pub const PAUSE_BUTTON_HEIGHT: f32 = 0.3;

pub const PLAYER_SPEED: f32 = 1.2; // tiles per second
pub const PLAYER_SIZE: f32 = 0.012; // scale
pub const PLAYER_MAX_HEALTH: f32 = 10.0;

pub const PLAYER_HP_BAR_WIDTH: f32 = 0.5; // scale
pub const PLAYER_HP_BAR_HEIGHT: f32 = 0.025; // scale
pub const PLAYER_HP_BAR_BOT_OFFSET: f32 = 0.033; // scale
pub const PLAYER_HP_BAR_THICKNESS: f32 = 0.0075; // scale
pub const PLAYER_XP_BAR_OFFSET: f32 = 0.01; // scale
pub const PLAYER_BARS_FONT_RATIO: f32 = 0.85; // percent

pub const FPS_TEXT_UPDATE_PERIOD: f32 = 1.0 / 10.0; // seconds

pub const CAMERA_FOLLOW_SPEED: f32 = 0.95;
pub const CAMERA_Y_OFFSET: f32 = 1.0;

pub const BAR_UPDATE_SPEED: f32 = 0.99999;

pub const TIME_TO_MOUSE_IDLE: f32 = 5.0; // seconds

pub const CROSSHAIR_SIZE: f32 = 0.02; // scale
pub const CROSSHAIR_THICKNESS: f32 = 0.002; // scale

pub const FPS_FONT_SPACING: f32 = 0.01; // scale
pub const FPS_FONT_SIZE: f32 = 0.025; // scale
pub const SCORE_FONT_SIZE: f32 = 0.06; // scale
pub const SCORE_FONT_SPACING: f32 = 0.02; // scale

pub const LARGE_FONT_SIZE: f32 = 0.15; // scale
pub const SMALL_FONT_SIZE: f32 = 0.04; // scale
pub const SMALL_FONT_SPACING: f32 = 0.035; // scale
pub const SMALL_FONT_LARGE_SPACING: f32 = 0.05; // scale
pub const LARGE_FONT_BOUNCE_MAX: f32 = 0.125; // font size
pub const LARGE_FONT_BOUNCE_SPEED: f32 = 0.8;

pub const HEARTS_HEAL_CHANCE: f32 = 0.05;
pub const CLUBS_STUN_TIME: f32 = 0.25; // seconds
pub const SPADES_DAMAGE_CHANCE: f32 = 0.2;

pub const POWERUP_PICK_OUTLINE_WIDTH: f32 = 0.95; // scale
pub const POWERUP_PICK_OUTLINE_HEIGHT: f32 = 0.95; // scale
pub const POWERUP_PICK_OUTLINE_RATIO: f32 = 2. / 1.0; // width / height
pub const POWERUP_PICK_OUTLINE_PADDING: f32 = 0.02; // scale
pub const POWERUP_PICK_OUTLINE_THICKNESS: f32 = 0.01;
pub const POWERUP_PICK_FONT_LARGE: f32 = 0.043; // scale
pub const POWERUP_PICK_FONT_SMALL: f32 = 0.03; // scale
pub const POWERUP_PICK_FONT_SPACING_CENTER: f32 = 0.01;

pub const POWERUP_DISPLAY_SIZE: f32 = 0.05;
pub const POWERUP_DIPLAY_SPACING: f32 = 0.01;
pub const POWERUP_DIPLAY_Y_OFFSET: f32 = 0.027;
pub const POWERUP_DISPLAY_MAX_HEIGHT: usize = 8;
pub const POWERUP_OUTLINE_THICKNESS: f32 = 0.0075;

pub const HAND_BOTTOM_PADDING: f32 = 0.1;
pub const HAND_TOTAL_MAX_WIDTH: f32 = 0.6;
pub const HAND_TOTAL_MAX_HEIGHT: f32 = 0.3;
pub const HAND_SPACING: f32 = 0.2; // relative to card width
pub const HAND_OUTLINE_THICKNESS: f32 = 0.002;
pub const SLOT_MAX_START_VALUE: u8 = 4;

pub const CARD_CHOICE_TOP_PADDING: f32 = 0.05;
pub const CARD_CHOICE_MAX_WIDTH: f32 = 0.9;
pub const CARD_CHOICE_MAX_HEIGHT: f32 = 0.5;
pub const CARD_CHOICE_SPACING: f32 = 0.1; // relative to card width

pub const JOKER_WEAPON: weapon::Weapon = weapon::Weapon::new(0.1, 3.0, 0.9, 0.6, 5.0);
pub const ACE_WEAPON: weapon::Weapon = weapon::Weapon::new(0.2, 20.0, 0.9, 0.7, 10.0);
pub const FACE_WEAPON: weapon::Weapon = weapon::Weapon::new(2.0, 15.0, 1.0, 0.75, 7.5);
pub const ELSE_WEAPON: weapon::Weapon = weapon::Weapon::new(5.0, 15.0, 1.0, 0.75, 7.5);

pub const BULLET_SIZE: f32 = 0.005; // scale
pub const BULLET_OUTLINE: f32 = 0.0005; // scale

pub const ENEMY_SIZE: f32 = 0.02; // scale
pub const ENEMY_SPEED: f32 = PLAYER_SPEED * 0.6; // tiles per second
pub const ENEMY_STUNNED_THICKNESS: f32 = 0.005; // scale

pub const ENEMY_MELEE_RANGE: f32 = 0.5; // tiles
pub const ENEMY_MELEE_CHARGE_TIME: f32 = 0.5; // seconds
pub const ENEMY_MELEE_RELOAD_TIME: f32 = 1.0; // seconds

pub const ENEMY_RANGED_RANGE: f32 = 4.0; // tiles
pub const ENEMY_RANGED_CHARGE_TIME: f32 = 0.75; // seconds
pub const ENEMY_RANGED_RELOAD_TIME: f32 = 4.0; // seconds
pub const ENEMY_RANGED_SPEED_PENALTY: f32 = 0.75; // percent

pub const ENEMY_RANGED_BULLET_SPEED: f32 = 4.0; // tiles per second
pub const ENEMY_RANGED_BULLET_RANGE: f32 = 12.0; // tiles

pub const ENEMY_MELEE_CHARGE_THICKNESS: f32 = 0.005; // scale

pub const ENEMY_SUPER_SIZE: f32 = 0.05;
pub const ENEMY_SUPER_RANGE: f32 = 5.0;
pub const ENEMY_SUPER_MIN_RANGE: f32 = 1.0;
pub const ENEMY_SUPER_WAVE_START: i32 = 2;
pub const ENEMY_SUPER_WAVE_FIRE_RATE: fn(i32) -> f32 =
    |wave| (wave - ENEMY_SUPER_WAVE_START) as f32 / 5.0;
pub const ENEMY_SUPER_SPREAD: f32 = 0.6; // radians
pub const ENEMY_SUPER_HP_MOD: fn(i32) -> f32 = |wave| 5.0 + wave as f32;

pub const ENEMY_SPAWN_RADIUS: f32 = TILES_PER_SCALE as f32 + 2.0; // tiles
pub const ENEMY_RANGED_CHANCE: f32 = 0.33; // percent

// Note: wave spawning starts at 1
pub const ENEMY_WAVE_COUNT: fn(i32) -> i32 = |wave| 5 + 3 * (wave - 1);

pub const ENEMY_WAVE_HP: fn(i32) -> f32 = |wave| 10.0 + 10.0 * (wave - 1) as f32;
pub const ENEMY_DAMAGE: f32 = 1.0;
pub const ENEMY_WAVE_SPEED: fn(i32) -> f32 = |wave| ENEMY_SPEED * (1.0 + 0.015 * (wave - 1) as f32);

pub const XP_PER_LEVEL: fn(i32) -> i32 = |level| 2 * level.pow(2);

// x, a, b, c
// f(x) = ((b - a) / log(c + 1)) * log(x + 1) + a
// where a = f(0) and b = f(c)
pub const AUTO_LOG: fn(f32, f32, f32, f32) -> f32 =
    |x, a, b, c| ((b - a) / (c + 1.0).log10()) * (x + 1.0).log10() + a;
pub const ENEMY_WAVE_SPAWN_RATE: fn(i32) -> f32 =
    |wave| AUTO_LOG(wave as f32, 1.0 / 5.0, 1.0 / 2.0, 10.0);
