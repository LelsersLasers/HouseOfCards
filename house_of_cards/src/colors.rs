// ignore dead code warnings
#![allow(dead_code)]

use macroquad::prelude as mq;
use macroquad::prelude::Color;

// nord0 #2E3440
// nord1 #3B4252
// nord2 #434C5E
// nord3 #4C566A
// nord4 #D8DEE9
// nord5 #E5E9F0
// nord6 #ECEFF4
// nord7 #8FBCBB
// nord8 #88C0D0
// nord9 #81A1C1
// nord10 #5E81AC
// nord11 #BF616A
// nord12 #D08770
// nord13 #EBCB8B
// nord14 #A3BE8C
// nord15 #B48EAD

// dracula0 #282a36
// dracula1 #44475a
// dracula2 #6272a4

// Surface2	#626880	rgb(98, 104, 128)	hsl(228, 13%, 44%)
// Surface1	#51576d	rgb(81, 87, 109)	hsl(227, 15%, 37%)
// Surface0	#414559	rgb(65, 69, 89)	hsl(230, 16%, 30%)
// Base	#303446	rgb(48, 52, 70)	hsl(229, 19%, 23%)
// Mantle	#292c3c	rgb(41, 44, 60)	hsl(231, 19%, 20%)
// Crust	#232634	rgb(35, 38, 52)	hsl(229, 20%, 17%)

pub const NORD0: mq::Color = mq::Color::new(0.180, 0.204, 0.251, 1.0);
pub const NORD1: mq::Color = mq::Color::new(0.231, 0.259, 0.322, 1.0);
pub const NORD2: mq::Color = mq::Color::new(0.263, 0.298, 0.369, 1.0);
pub const NORD3: mq::Color = mq::Color::new(0.298, 0.337, 0.416, 1.0);
pub const NORD4: mq::Color = mq::Color::new(0.847, 0.871, 0.914, 1.0);
pub const NORD5: mq::Color = mq::Color::new(0.898, 0.914, 0.957, 1.0);
pub const NORD6: mq::Color = mq::Color::new(0.925, 0.937, 0.969, 1.0);
pub const NORD7: mq::Color = mq::Color::new(0.561, 0.737, 0.729, 1.0);
pub const NORD8: mq::Color = mq::Color::new(0.533, 0.753, 0.816, 1.0);
pub const NORD9: mq::Color = mq::Color::new(0.506, 0.631, 0.757, 1.0);
pub const NORD10: mq::Color = mq::Color::new(0.369, 0.506, 0.631, 1.0);
pub const NORD11: mq::Color = mq::Color::new(0.749, 0.380, 0.416, 1.0);
pub const NORD12: mq::Color = mq::Color::new(0.816, 0.498, 0.439, 1.0);
pub const NORD13: mq::Color = mq::Color::new(0.922, 0.796, 0.545, 1.0);
pub const NORD14: mq::Color = mq::Color::new(0.639, 0.745, 0.549, 1.0);
pub const NORD15: mq::Color = mq::Color::new(0.706, 0.557, 0.678, 1.0);

pub const NORD3_ALPHA: mq::Color = mq::Color::new(0.298, 0.337, 0.416, 0.9);
pub const NORD6_ALPHA: mq::Color = mq::Color::new(0.925, 0.937, 0.969, 0.9);
pub const NORD11_ALPHA: mq::Color = mq::Color::new(0.749, 0.380, 0.416, 0.9);
pub const NORD14_ALPHA: mq::Color = mq::Color::new(0.639, 0.745, 0.549, 0.9);

pub const NORD0_BIG_ALPHA: mq::Color = mq::Color::new(0.180, 0.204, 0.251, 0.5);
pub const NORD4_BIG_ALPHA: mq::Color = mq::Color::new(0.847, 0.871, 0.914, 0.5);
pub const NORD6_BIG_ALPHA: mq::Color = mq::Color::new(0.925, 0.937, 0.969, 0.5);

// pub const DRACULA0: mq::Color = mq::Color::new(0.157, 0.165, 0.212, 1.0);
// pub const DRACULA1: mq::Color = mq::Color::new(0.267, 0.286, 0.400, 1.0);
// pub const DRACULA2: mq::Color = mq::Color::new(0.384, 0.408, 0.545, 1.0);

pub const SURFACE2: mq::Color = mq::color_u8!(98, 104, 128, 255);
pub const SURFACE1: mq::Color = mq::color_u8!(81, 87, 109, 255);
pub const SURFACE0: mq::Color = mq::color_u8!(65, 69, 89, 255);
pub const BASE: mq::Color = mq::color_u8!(48, 52, 70, 255);
pub const MANTLE: mq::Color = mq::color_u8!(41, 44, 60, 255);
pub const CRUST: mq::Color = mq::color_u8!(35, 38, 52, 255);

pub const NORD_COLORS: [mq::Color; 16] = [
    NORD0, NORD1, NORD2, NORD3, NORD4, NORD5, NORD6, NORD7, NORD8, NORD9, NORD10, NORD11, NORD12,
    NORD13, NORD14, NORD15,
];

// pub const DRACULA_COLORS: [mq::Color; 3] = [DRACULA0, DRACULA1, DRACULA2];

pub const SURFACE_COLORS: [mq::Color; 6] = [SURFACE2, SURFACE1, SURFACE0, BASE, MANTLE, CRUST];
