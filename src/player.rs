use macroquad::prelude::*;
use std::f32::consts::PI;
use ::rand::{thread_rng, Rng};

use crate::DrawToMinimap;
use crate::map::TileMap;

pub struct Player {
    pub position: Vec2,
    pub rotation: f32,
    pub fov: f32,
}

impl DrawToMinimap for Player {
    fn minimap_draw(&self, color: Color) {
        <Self as DrawToMinimap>::draw_rect(self.position, Vec2::splat(0.1), color);
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            rotation: PI * (3.0 / 2.0),
            fov: 2.0 * PI * (60.0 / 360.0),
        }
    }

    pub fn random_location(&mut self, tile_map: &TileMap) {
        loop { 
            let start_x = thread_rng().gen_range(0..tile_map.width);
            let start_y = thread_rng().gen_range(0..tile_map.height);

            if tile_map.is_tile_empty(start_x, start_y) {
                self.position = Vec2::new(start_x as f32, start_y as f32);
                return;
            }
        }
    }
}
