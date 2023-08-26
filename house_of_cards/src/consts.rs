use macroquad::prelude as mq;

pub const WINDOW_START_SIZE: u32 = 800;
pub const BACKGROUND_COLOR: mq::Color = crate::colors::NORD1;
pub const MOUSE_COLOR: mq::Color = crate::colors::NORD6;
pub const TILES_PER_SCALE: u32 = 10;
pub const PLAYER_SPEED: f32 = 2.0; // tiles per second
pub const PLAYER_SIZE: f32 = 0.15; // tiles
pub const TIME_TO_MOUSE_IDLE: f32 = 2.5; // seconds