use macroquad::prelude as mq;
use rayon::prelude::*;
use std::collections::HashMap;

use crate::{colors, consts, player};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        // red can only go next to orange or purple or red
        // orange can only go next to red or yellow or orange
        // yellow can only go next to orange or green or yellow
        // green can only go next to yellow or blue or green
        // blue can only go next to green or purple or blue
        // purple can only go next to blue or red or purple

        self == other
            || matches!(
                (self, other),
                (Tile::Red, Tile::Orange)
                    | (Tile::Red, Tile::Purple)
                    | (Tile::Orange, Tile::Red)
                    | (Tile::Orange, Tile::Yellow)
                    | (Tile::Yellow, Tile::Orange)
                    | (Tile::Yellow, Tile::Green)
                    | (Tile::Green, Tile::Yellow)
                    | (Tile::Green, Tile::Blue)
                    | (Tile::Blue, Tile::Green)
                    | (Tile::Blue, Tile::Purple)
                    | (Tile::Purple, Tile::Blue)
                    | (Tile::Purple, Tile::Red)
            )
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
    tiles: HashMap<(i32, i32), Tile>,
    locations_to_build: Vec<(i32, i32)>,
}
impl World {
    pub fn new() -> Self {
        let mut tiles = HashMap::new();
        tiles.insert((0, 0), Tile::Red);

        let locations_to_build = vec![];

        Self {
            tiles,
            locations_to_build,
        }
    }

    fn get_world_bounds_info(&self, player: &player::Player, scale: f32) -> WorldBoundsInfo {
        let tile_size = scale / consts::TILES_PER_SCALE as f32;

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

    pub fn draw(&self, player: &player::Player, scale: f32) {
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
                let tile = self.tiles.get(&(x, y)).unwrap_or(&Tile::Black);
                let color = tile.get_color();

                let x = (x as f32 - start_x) * tile_size;
                let y = (y as f32 - start_y) * tile_size;

                mq::draw_rectangle(x, y, tile_size, tile_size, color);
                // mq::draw_rectangle_lines(x, y, tile_size, tile_size, 2.0, Tile::Black.get_color());
            }
        }
    }

    pub fn update_locations_to_build(&mut self, player: &player::Player, scale: f32) {
        // fill in all tiles that are within the player's view

        let WorldBoundsInfo {
            start_x: _,
            start_x_floor,
            end_x: _,
            end_x_ceil,

            start_y: _,
            start_y_floor,
            end_y: _,
            end_y_ceil,

            tiles_count_x: _,
            tiles_count_y: _,
            tile_size: _,
        } = self.get_world_bounds_info(player, scale);

        for x in start_x_floor..end_x_ceil {
            for y in start_y_floor..end_y_ceil {
                let has_tile = self.tiles.contains_key(&(x, y));
                if !has_tile {
                    // self.place_tile((x, y));
                    self.locations_to_build.push((x, y));
                }
            }
        }
    }

    fn get_tile_neighbors(&self, location: (i32, i32), range: i32) -> Vec<Tile> {
        let mut neighbors = Vec::new();
        for x in -range..=range {
            for y in -range..=range {
                if x == 0 && y == 0 {
                    continue;
                }
                let new_location = (location.0 + x, location.1 + y);

                let tile = self.tiles.get(&new_location);
                if let Some(tile) = tile {
                    neighbors.push(*tile);
                }
            }
        }

        neighbors
    }

    pub fn build_locations(&mut self) {
        // sort by number of neighbors

        let tile_neighbor_counts = self
            .locations_to_build
            .par_iter()
            .map(|location| (location, self.get_tile_neighbors(*location, 1).len()))
            .collect::<HashMap<_, _>>();

        if !tile_neighbor_counts.is_empty() {
            let max_location_and_key = tile_neighbor_counts
                .par_iter()
                .max_by_key(|(_, count)| *count)
                .map(|(location, _)| location)
                .unwrap();

            self.place_tile(**max_location_and_key);
        }
    }

    fn place_tile(&mut self, location: (i32, i32)) {
        if self.tiles.contains_key(&location) {
            self.locations_to_build.retain(|l| l != &location);
        }

        let neighbors = self.get_tile_neighbors(location, 1);

        let all_titles = [
            Tile::Red,
            Tile::Orange,
            Tile::Yellow,
            Tile::Green,
            Tile::Blue,
            Tile::Purple,
        ]
        .par_iter()
        .filter(|tile| {
            neighbors
                .iter()
                .all(|neighbor| tile.can_place_next_to(neighbor))
        })
        .collect::<Vec<_>>();

        if !all_titles.is_empty() {
            let index = mq::rand::gen_range(0, all_titles.len());
            self.tiles.insert(location, *all_titles[index]);

            self.locations_to_build.retain(|l| l != &location);
        }
    }
}
