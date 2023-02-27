use macroquad::prelude::*;
use crate::raycasting::*;
use crate::{Player,TileMap};

pub const LOGICAL_TO_PHYSICAL_SIZE: f32 = 50.0;

pub trait MinimapObject {
   fn minimap_color(&self) -> Color;

   fn world_position(&self) -> Vec2;

   fn world_size(&self) -> Vec2;
}

pub struct Minimap {
    position: Vec2,
    size_radius: f32,
    color: Color,
    view_range: f32,
}

impl Minimap { 

    const MINIMAP_SCALE: f32 = 25.0;

    pub fn new() -> Self {
        let width = 0.15 * screen_width();
        let height = 0.15 * screen_height();
        let offset_height = 0.05 * screen_height();
        let offset_width = 0.05 * screen_width();

        let size_radius =  width.max(height);
        let position = Vec2::new(width + offset_width, height + offset_height);

        let view_range = 100.0; 
        
        Self {
            position,
            size_radius,
            color: VIOLET,
            view_range,
        }
    }

    pub fn update(&self, player: &Player, objects: Vec<Box<dyn MinimapObject>>) {
        self.draw_empty();
        self.place_objects(player, objects);
    }

    fn draw_empty(&self) {
       draw_circle(self.position.x, self.position.y, self.size_radius, self.color);
    }
    
    fn place_objects(&self, player: &Player, objects: Vec<Box<dyn MinimapObject>>) { 
        for object in objects.into_iter() {
            if object.world_position().distance(player.position) > self.view_range {
                continue;
            }
 
            let minimap_position = object.world_position() * Self::MINIMAP_SCALE;
            let minimap_size = object.world_size() * Self::MINIMAP_SCALE;

            draw_rectangle(
                minimap_position.x + minimap_size.x / 2.0,
                minimap_position.y + minimap_size.y / 2.0,
                minimap_size.x,
                minimap_size.y,
                object.minimap_color(),
            );
        }
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

    // fn draw_minimap_rays_in_view(&self) {
    //     let player_position = self.player.position;
    //     for particle in get_particles_in_view(self.player, self.tile_map).iter() {
    //         draw_line(
    //             player_position.x * MINIMAP_SCALE,
    //             player_position.y * MINIMAP_SCALE,
    //             particle.point.x * MINIMAP_SCALE,
    //             particle.point.y * MINIMAP_SCALE,
    //             0.5,
    //             get_tile_color(particle.value, particle.plane),
    //         );
    //     }
    // }
    
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

