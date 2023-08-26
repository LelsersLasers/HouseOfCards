use macroquad::prelude as mq;

use crate::colors;

pub const WINDOW_START_SIZE: u32 = 800;

pub const BACKGROUND_COLOR: mq::Color = crate::colors::NORD1;
pub const MOUSE_COLOR: mq::Color = crate::colors::NORD6;
pub const BACKGROUND_COLORS: [mq::Color; 6] = colors::SURFACE_COLORS;

pub const TILES_PER_SCALE: u32 = 12;

pub const PLAYER_SPEED: f32 = 1.5; // scale per second
pub const PLAYER_SIZE: f32 = 0.012; // scale

pub const TIME_TO_MOUSE_IDLE: f32 = 2.5; // seconds

pub const CROSSHAIR_SIZE: f32 = 0.02; // scale
pub const CROSSHAIR_THICKNESS: f32 = 0.002; // scale

pub const DECK_SPACING_OUTSIDE: f32 = 0.01; // scale
pub const DECK_SPACING_INSIDE: f32 = 0.02; // scale
pub const DECK_WIDTH: f32 = 0.12; // scale
pub const DECK_HEIGHT: f32 = 0.16; // scale
pub const DECK_THICKNESS: f32 = 0.01; // scale

pub const DISCARD_ROTATION: f32 = 0.1;
pub const DISCARD_OFFSET: f32 = 0.05;
pub const DISCARD_TO_DRAW: usize = 5;

pub const FPS_SPACING: f32 = 0.01; // scale
pub const FPS_FONT_SIZE: f32 = 0.04; // scale
