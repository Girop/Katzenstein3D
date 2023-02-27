use crate::{map::TileMap, player::Player};
use macroquad::prelude::*;
use std::{collections::HashMap, f32::consts::PI};

#[derive(Debug, Copy, Clone)]
enum Action {
    MoveStraight,
    MoveBack,
    MoveLeft,
    MoveRight,
    RotateLeft,
    RotateRight,
}

pub struct InputHandler {
    key_bindings: HashMap<KeyCode, Action>,
    position_change: f32,
    rotation_change: f32,
}

impl InputHandler {
    pub fn new() -> Self {
        let default_bindings: HashMap<KeyCode, Action> = HashMap::from([
            (KeyCode::W, Action::MoveStraight),
            (KeyCode::S, Action::MoveBack),
            (KeyCode::A, Action::MoveLeft),
            (KeyCode::D, Action::MoveRight),
            (KeyCode::E, Action::RotateRight),
            (KeyCode::Q, Action::RotateLeft),
        ]);

        let position_change = 0.05;
        let rotation_change = 0.02;

        Self {
            key_bindings: default_bindings,
            position_change,
            rotation_change,
        }
    }

    pub fn handle_keyboard_input(&self, player: &mut Player, tile_map: &TileMap) {
        let mut new_position = player.position.clone();
        let beta_angle = 2.0 * PI - player.rotation;

        for action in self.get_actions().iter() {
            match action {
                Action::MoveStraight => {
                    new_position.y += player.rotation.sin() * self.position_change;
                    new_position.x += player.rotation.cos() * self.position_change;
                }
                Action::MoveBack => {
                    new_position.y -= player.rotation.sin() * self.position_change;
                    new_position.x -= player.rotation.cos() * self.position_change;
                }
                Action::MoveRight => {
                    new_position.x += beta_angle.sin() * self.position_change;
                    new_position.y += beta_angle.cos() * self.position_change;
                }
                Action::MoveLeft => {
                    new_position.x -= beta_angle.sin() * self.position_change;
                    new_position.y -= beta_angle.cos() * self.position_change;
                }
                Action::RotateLeft => {
                    player.rotation -= self.rotation_change;
                }
                Action::RotateRight => {
                    player.rotation += self.rotation_change;
                }
            };

            if tile_map.is_tile_empty(new_position.y as usize, new_position.x as usize) {
                player.position = new_position;
            }
        }
    }

    fn get_actions(&self) -> Vec<Action> {
        self.key_bindings
            .iter()
            .filter(|(key_code, _)| is_key_down(**key_code))
            .map(|(_key, action)| *action)
            .collect()
    }
}
