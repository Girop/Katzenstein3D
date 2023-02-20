use std::collections::HashMap;
use std::f32::consts::PI;
use macroquad::{prelude::*, rand::gen_range};

const TILE_SIZE: f32 = 1.0;
const LOGICAL_TO_PHYSICAL_SIZE: f32 = 50.0;

struct Tile {
    pub value: i8,
    pub rect: Rect,
    pub position: Vec2,
}

impl Tile {
    pub fn new(value: i8, position: Vec2) -> Self {
        let rect = Rect::new(
            position.x,
            position.y,
            TILE_SIZE,
            TILE_SIZE,
        );
        Self{value, position, rect}
    }

    pub fn draw(&self) {
        if self.value == 0 {
            return;
        }
        draw_rectangle(
            self.rect.x * LOGICAL_TO_PHYSICAL_SIZE,
            self.rect.y * LOGICAL_TO_PHYSICAL_SIZE,
            self.rect.w * LOGICAL_TO_PHYSICAL_SIZE,
            self.rect.h * LOGICAL_TO_PHYSICAL_SIZE,
            WHITE,
        );
    }

    pub fn in_tile(&self, point: Vec2) -> bool {
        self.rect.contains(point)
    }
}

struct TileMap{
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize
}

impl TileMap {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        for row in 0..height {
            let mut row_vec: Vec<Tile> = Vec::new();
            for column in 0..width {
                row_vec.push(
                    Tile::new(
                        1,
                        Vec2::new(row as f32, column as f32)
                    )
                );
            }
            tiles.push(row_vec);
        }
        Self { tiles , width, height }
    }

    pub fn draw_map(&self) {
        for row in self.tiles.iter() {
            for value in row.iter() {
                value.draw();
            }
        }
    }

    pub fn carve(&mut self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) {
        for row_index in start_x..end_x {
            for column_index in start_y..end_y {
                self.tiles[row_index][column_index].value = 0;
            }
        }
    }

    pub fn is_tile_empty(&self, column_index: usize, row_index: usize) -> bool {
         if self.get_tile_value(column_index,row_index) == 0 {
             return true;
         }
         false
    }

    fn get_tile_value(&self, column_index: usize, row_index: usize) -> i8 {
        if !self.is_index_in_range(column_index,row_index) {
            panic!("Incorrect index");  // TODO something with that
        }
        self.tiles[row_index][column_index].value
    }

    fn is_index_in_range(&self, column_index: usize, row_index: usize) -> bool {
        if column_index > self.width && row_index > self.height {
            return false;
        }
        true
    }
}

struct Player {
    position: Vec2,
    rotation: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(0.0,0.0),
            rotation: PI / 2.0,
        }
    }

    pub fn random_location(&mut self, tile_map: &TileMap) {
        let mut start_x: usize;
        let mut start_y: usize;

        loop {
            start_x = gen_range(0,tile_map.width);
            start_y = gen_range(0,tile_map.height);

            if tile_map.is_tile_empty(start_x, start_y) {
                break;
            }
        }
        self.position = Vec2::new(start_x as f32, start_y as f32);
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.position.x * LOGICAL_TO_PHYSICAL_SIZE,
            self.position.y * LOGICAL_TO_PHYSICAL_SIZE,
            0.1 * LOGICAL_TO_PHYSICAL_SIZE,
            0.1 * LOGICAL_TO_PHYSICAL_SIZE,
            RED
        );
    }
}

#[derive(Debug, Copy, Clone)]
enum Action {
    MoveStraight,
    MoveBack,
    MoveLeft,
    MoveRight,
    RotateLeft,
    RotateRight,
}

fn get_actions(action_bindings: &HashMap<KeyCode,Action>) -> Vec<Action> {
    action_bindings
        .iter()
        .filter(|(key_code,_)| is_key_down(**key_code))
        .map(|(_, action)| *action)
        .collect()
}

const POSITION_CHANGE: f32 = 0.1;
const ROTATION_CHANGE: f32 = 0.02;

fn handle_movement_input(player: &mut Player, tile_map: &TileMap, key_bindings: &HashMap<KeyCode,Action>){
    let mut new_position = player.position.clone();
    let player_rotation = player.rotation;
    let beta_angle = 2.0 * PI - player_rotation;

    for action in get_actions(&key_bindings).into_iter() {
        match action {
            Action::MoveStraight => {
                new_position.y -= player_rotation.sin() * POSITION_CHANGE;
                new_position.x -= player_rotation.cos() * POSITION_CHANGE;
            },
            Action::MoveBack => {
                new_position.y += player_rotation.sin() * POSITION_CHANGE;
                new_position.x += player_rotation.cos() * POSITION_CHANGE;
            },
            Action::MoveRight => {
                new_position.x -= beta_angle.sin() * POSITION_CHANGE;
                new_position.y -= beta_angle.cos() * POSITION_CHANGE;
            },
            Action::MoveLeft => {
                new_position.x += beta_angle.sin() * POSITION_CHANGE;
                new_position.y += beta_angle.cos() * POSITION_CHANGE;
            },
            Action::RotateLeft => {
                player.rotation -= ROTATION_CHANGE;
            },
            Action::RotateRight => {
                player.rotation += ROTATION_CHANGE;
            }
        };

        if tile_map.is_tile_empty(new_position.y as usize,new_position.x as usize) {
            player.position = new_position;
        }
    }
}

struct GameData {
    key_bindings: HashMap<KeyCode,Action>,
    tile_map: TileMap,
    player: Player,
}

impl GameData {
    pub fn new() -> Self {
        let mut tile_map = TileMap::new(10,10);
        tile_map.carve(1,1,9,9);

        let mut player = Player::new();
        player.random_location(&tile_map);

        let key_bindings: HashMap<KeyCode,Action> = HashMap::from([
            (KeyCode::W, Action::MoveStraight),
            (KeyCode::S, Action::MoveBack),
            (KeyCode::A, Action::MoveLeft),
            (KeyCode::D, Action::MoveRight),
            (KeyCode::E, Action::RotateRight),
            (KeyCode::Q, Action::RotateLeft),
        ]);

        Self { key_bindings, tile_map, player}
    }
}

#[macroquad::main("Katzenstein")]
async fn main() {
    let game_data = GameData::new(); // TODO global resources system

    let mut player = game_data.player;
    let tile_map = game_data.tile_map;
    let key_bindings = game_data.key_bindings;

    loop {
        clear_background(BLACK);
        tile_map.draw_map();
        handle_movement_input(&mut player,&tile_map,&key_bindings);
        player.draw();

        next_frame().await
    }
}
