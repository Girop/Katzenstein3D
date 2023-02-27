use crate::{Player, TileMap};
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum ContactPlane {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct ContactPoint {
    pub point: Vec2,
    pub plane: ContactPlane,
    pub value: i8,
}

impl ContactPoint {
    pub fn new(point: Vec2, plane: ContactPlane, value: i8) -> Self {
        Self {
            point,
            plane,
            value,
        }
    }
}

pub fn get_particles_in_view(player: &Player, tile_map: &TileMap) -> Vec<ContactPoint> {
    angles_in_fov(&player)
        .into_iter()
        .map(|angle| get_particle_contact_point(&player, angle, &tile_map))
        .collect()
}

pub fn angles_in_fov(player: &Player) -> Vec<f32> {
    // TODO fix resoultion problem
    const ANGLE_STEP: f32 = 0.005;
    let mut angles: Vec<f32> = Vec::new();
    let mut current_angle = player.rotation - player.fov / 2.0;
    let end_angle = player.rotation + player.fov / 2.0;

    while current_angle < end_angle {
        angles.push(current_angle);
        current_angle += ANGLE_STEP;
    }
    angles
}

fn get_particle_contact_point(player: &Player, angle: f32, tile_map: &TileMap) -> ContactPoint {
    const DISTANCE_STEP: f32 = 0.05;
    let mut particle_position = player.position;
    let mut counter: u32 = 0;
    let mut value = 0;

    while tile_map.is_tile_empty(particle_position.y as usize, particle_position.x as usize) {
        match counter % 2 {
            0 => {
                particle_position.x += angle.cos() * DISTANCE_STEP;
            }
            1 => {
                particle_position.y += angle.sin() * DISTANCE_STEP;
            }
            _ => (),
        }
        value = tile_map.get_tile_value(particle_position.y as usize, particle_position.x as usize);
        counter += 1;
    }

    let plane = match counter % 2 {
        0 => ContactPlane::Horizontal,
        1 => ContactPlane::Vertical,
        _ => panic!("Unkown dimension"),
    };
    ContactPoint::new(particle_position, plane, value)
}
