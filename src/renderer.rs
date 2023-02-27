use macroquad::prelude::*;
use crate::raycasting::*;
use crate::{Player,TileMap};

pub const TILE_SIZE: f32 = 1.0;
pub const LOGICAL_TO_PHYSICAL_SIZE: f32 = 50.0;
pub const MINIMAP_SCALE: f32 = 25.0;

pub trait DrawToMinimap {
    fn minimap_draw(&self, color: Color);

    // TODO better scaling / offset
    fn draw_rect(position: Vec2, size: Vec2, color: Color) {
        draw_rectangle(
            position.x * MINIMAP_SCALE,
            position.y * MINIMAP_SCALE,
            size.x * MINIMAP_SCALE,
            size.y * MINIMAP_SCALE,
            color,
        );
    }
}

pub struct Renderer<'a> {
    player: &'a Player,
    tile_map: &'a TileMap,
}

impl<'a> Renderer<'a>{

    pub fn new(player: &'a Player, tile_map: &'a TileMap) -> Self {
        Self { player, tile_map }
    }

    pub fn draw_world(&self) {
        self.draw_walls();
    }

    pub fn draw_minimap(&self) {
        self.draw_minimap_background();
        self.tile_map.minimap_draw();
        self.player.minimap_draw(RED);
        self.draw_minimap_rays_in_view();
    }

    fn draw_minimap_background(&self) {
        draw_rectangle(
            0.0,
            0.0,
            self.tile_map.width as f32 * MINIMAP_SCALE,
            self.tile_map.height as f32 * MINIMAP_SCALE,
            BLACK,
        );
    }

    fn draw_minimap_rays_in_view(&self) {
        let player_position = self.player.position;
        for particle in get_particles_in_view(self.player, self.tile_map).iter() {
            draw_line(
                player_position.x * MINIMAP_SCALE,
                player_position.y * MINIMAP_SCALE,
                particle.point.x * MINIMAP_SCALE,
                particle.point.y * MINIMAP_SCALE,
                0.5,
                get_tile_color(particle.value, particle.plane),
            );
        }
    }

    fn draw_walls(&self) {
        let particles = get_particles_in_view(self.player, self.tile_map);
        let wall_width = particles.len() as f32 / screen_width() * LOGICAL_TO_PHYSICAL_SIZE;

        for (index, particle) in particles.iter().enumerate() {
            let euclidean_distance = particle.point.distance(self.player.position);
            // potential fish eye effect, visible more with textures
            // maybe use distance perpendicular to camera plane
            let wall_height = screen_height() / euclidean_distance;

            draw_rectangle(
                index as f32 * wall_width,
                screen_height() / 2.0 - wall_height / 2.0,
                wall_width,
                wall_height,
                get_tile_color(particle.value, particle.plane),
            );
        }
    }
}
