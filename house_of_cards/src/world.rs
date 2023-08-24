use macroquad::prelude as mq;
use std::collections::HashMap;

use crate::{colors, player::Player, TILES_PER_SCALE};

pub enum Tile {
	Red,
	Orange,
	Yellow,
	Green,
	Blue,
	Purple,
	Black,
}
impl Tile {
	pub fn get_color(&self) -> mq::Color {
		match self {
			Tile::Red => colors::NORD11,
			Tile::Orange => colors::NORD12,
			Tile::Yellow => colors::NORD13,
			Tile::Green => colors::NORD14,
			Tile::Blue => colors::NORD10,
			Tile::Purple => colors::NORD15,
			Tile::Black => colors::NORD0,
		}
	}

	pub fn can_place_next_to(&self, other: &Tile) -> bool {
		// red can only go next to orange
		// orange can only go next to red or yellow
		// yellow can only go next to orange or green
		// green can only go next to yellow or blue
		// blue can only go next to green or purple
		// purple can only go next to blue

		match (self, other) {
			(Tile::Red, Tile::Orange) => true,

			(Tile::Orange, Tile::Red) => true,
			(Tile::Orange, Tile::Yellow) => true,

			(Tile::Yellow, Tile::Orange) => true,
			(Tile::Yellow, Tile::Green) => true,

			(Tile::Green, Tile::Yellow) => true,
			(Tile::Green, Tile::Blue) => true,
			
			(Tile::Blue, Tile::Green) => true,
			(Tile::Blue, Tile::Purple) => true,
			
			(Tile::Purple, Tile::Blue) => true,
			
			_ => false,
		}
	}
}


struct WorldBoundsInfo {
	pub start_x: f32,
	pub start_x_floor: i32,
	pub end_x: f32,
	pub end_x_ceil: i32,

	pub start_y: f32,
	pub start_y_floor: i32,
	pub end_y: f32,
	pub end_y_ceil: i32,
	
	pub tiles_count_x: f32,
	pub tiles_count_y: f32,
	pub tile_size: f32,
}
pub struct World {
	tiles: HashMap<(i32, i32), Tile>
}
impl World {
	pub fn new() -> Self {
		let mut tiles = HashMap::new();
		tiles.insert((0, 0), Tile::Red);

		Self {
			tiles
		}
	}

	fn get_world_bounds_info(&self, player: &Player, scale: f32) -> WorldBoundsInfo {
		let tile_size = scale / TILES_PER_SCALE as f32;

		let tiles_count_x = mq::screen_width() / tile_size;
		let tiles_count_y = mq::screen_height() / tile_size;

		let start_x = player.pos.x - (tiles_count_x / 2.0);
		let start_x_floor = start_x.floor() as i32;
		let end_x = player.pos.x + (tiles_count_x / 2.0);
		let end_x_ceil = end_x.ceil() as i32;

		let start_y = player.pos.y - (tiles_count_y / 2.0);
		let start_y_floor = start_y.floor() as i32;
		let end_y = player.pos.y + (tiles_count_y / 2.0);
		let end_y_ceil = end_y.ceil() as i32;

		WorldBoundsInfo {
			start_x,
			start_x_floor,
			end_x,
			end_x_ceil,

			start_y,
			start_y_floor,
			end_y,
			end_y_ceil,
			
			tiles_count_x,
			tiles_count_y,
			tile_size,
		}
	}

	pub fn draw(&self, player: &Player, scale: f32) {
		let WorldBoundsInfo {
			start_x,
			start_x_floor,
			end_x: _,
			end_x_ceil,

			start_y,
			start_y_floor,
			end_y: _,
			end_y_ceil,
			
			tiles_count_x: _,
			tiles_count_y: _,
			tile_size,
		} = self.get_world_bounds_info(player, scale);

		for x in start_x_floor..end_x_ceil {
			for y in start_y_floor..end_y_ceil {
				let tile = self.tiles.get(&(x, y)).unwrap_or(&Tile::Purple);
				let color = tile.get_color();

				let x = (x as f32 - start_x) * tile_size;
				let y = (y as f32 - start_y) * tile_size;

				mq::draw_rectangle(x, y, tile_size, tile_size, color);
				mq::draw_rectangle_lines(x, y, tile_size, tile_size, 2.0, Tile::Black.get_color());
			}
		}
	}

	pub fn update(&mut self, player: &Player, scale: f32) {	
		// fill in all tiles that are within the player's view

		let WorldBoundsInfo {
			start_x: _,
			start_x_floor,
			end_x,
			end_x_ceil,

			start_y: _,
			start_y_floor,
			end_y,
			end_y_ceil,
			
			tiles_count_x: _,
			tiles_count_y: _,
			tile_size: _,
		} = self.get_world_bounds_info(player, scale);

		for x in start_x_floor..end_x_ceil {
			for y in start_y_floor..end_y_ceil {
				let has_tile = self.tiles.contains_key(&(x, y));
				if !has_tile {
					self.tiles.insert((x, y), Tile::Red);
				}
			}
		}		
	}

	fn place_tile(&mut self, location: (i32, i32)) {

		let mut valid_tiles = Vec::new();
		

	}
}