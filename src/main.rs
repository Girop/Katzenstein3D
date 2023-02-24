use macroquad::{prelude::*, rand::gen_range};
use std::f32::consts::PI;
use std::collections::HashMap;

const TILE_SIZE: f32 = 1.0;
const LOGICAL_TO_PHYSICAL_SIZE: f32 = 50.0;
const MINIMAP_SCALE: f32 = 25.0;

trait DrawToMinimap {
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

struct Tile {
    pub value: i8,
    pub size: Vec2,
    position: Vec2,
}

impl DrawToMinimap for Tile {
    fn minimap_draw(&self, color: Color) {
        if self.value == 0 {
            return;
        }
        <Tile as DrawToMinimap>::draw_rect(self.position, self.size, color);
    }
}

impl Tile {
    pub fn new(value: i8, position: Vec2) -> Self {
        let size = Vec2::splat(TILE_SIZE);
        Self {
            value,
            position,
            size,
        }
    }
}

struct TileMap {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}

impl TileMap {
    pub fn new_filled(width: usize, height: usize) -> Self {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        for row in 0..height {
            let mut row_vec: Vec<Tile> = Vec::new();
            for column in 0..width {
                row_vec.push(Tile::new(1, Vec2::new(row as f32, column as f32)));
            }
            tiles.push(row_vec);
        }
        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn minimap_draw(&self) {
        for row in self.tiles.iter() {
            for value in row.iter() {
                value.minimap_draw(WHITE);
            }
        }
    }

    pub fn by_hand() -> Self {
        let tile_temp = [
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 2, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 2, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 2, 0, 3, 3, 3, 0, 0, 1],
            [1, 0, 2, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 2, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        let mut tile_map: Vec<Vec<Tile>> = Vec::new();
        for (row_index, row) in tile_temp.iter().enumerate() {
            let mut row_vec: Vec<Tile> = Vec::new();
            for (column, value) in row.iter().enumerate() {
                row_vec.push(Tile::new(
                    *value,
                    Vec2::new(row_index as f32, column as f32),
                ));
            }
            tile_map.push(row_vec);
        }

        Self {
            tiles: tile_map,
            width: 10,
            height: 10,
        }
    }

    fn carve(
        &mut self,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
    ) -> Option<bool> {
        if !(self.is_index_in_range(start_y, start_x) && self.is_index_in_range(end_y, end_x)) {
            return None;
        }

        for row_index in start_x..end_x {
            for column_index in start_y..end_y {
                self.tiles[row_index][column_index].value = 0;
            }
        }
        Some(true)
    }

    pub fn is_tile_empty(&self, column_index: usize, row_index: usize) -> bool {
        if self.get_tile_value(column_index, row_index) == 0 {
            return true;
        }
        false
    }

    fn get_tile_value(&self, column_index: usize, row_index: usize) -> i8 {
        if !self.is_index_in_range(column_index, row_index) {
            panic!("Incorrect index"); // TODO
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
        let mut start_x: usize;
        let mut start_y: usize;

        loop {
            start_x = gen_range(0, tile_map.width);
            start_y = gen_range(0, tile_map.height);

            if tile_map.is_tile_empty(start_x, start_y) {
                break;
            }
        }
        self.position = Vec2::new(start_x as f32, start_y as f32);
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

fn get_actions(action_bindings: &HashMap<KeyCode, Action>) -> Vec<Action> {
    action_bindings
        .iter()
        .filter(|(key_code, _)| is_key_down(**key_code))
        .map(|(_key, action)| *action)
        .collect()
}

const POSITION_CHANGE: f32 = 0.05;
const ROTATION_CHANGE: f32 = 0.02;

fn handle_movement_input(
    player: &mut Player,
    tile_map: &TileMap,
    key_bindings: &HashMap<KeyCode, Action>,
) {
    let mut new_position = player.position.clone();
    let beta_angle = 2.0 * PI - player.rotation;

    for action in get_actions(&key_bindings).into_iter() {
        match action {
            Action::MoveStraight => {
                new_position.y += player.rotation.sin() * POSITION_CHANGE;
                new_position.x += player.rotation.cos() * POSITION_CHANGE;
            }
            Action::MoveBack => {
                new_position.y -= player.rotation.sin() * POSITION_CHANGE;
                new_position.x -= player.rotation.cos() * POSITION_CHANGE;
            }
            Action::MoveRight => {
                new_position.x += beta_angle.sin() * POSITION_CHANGE;
                new_position.y += beta_angle.cos() * POSITION_CHANGE;
            }
            Action::MoveLeft => {
                new_position.x -= beta_angle.sin() * POSITION_CHANGE;
                new_position.y -= beta_angle.cos() * POSITION_CHANGE;
            }
            Action::RotateLeft => {
                player.rotation -= ROTATION_CHANGE;
            }
            Action::RotateRight => {
                player.rotation += ROTATION_CHANGE;
            }
        };

        if tile_map.is_tile_empty(new_position.y as usize, new_position.x as usize) {
            player.position = new_position;
        }
    }
}

struct GameData {
    key_bindings: HashMap<KeyCode, Action>,
    tile_map: TileMap,
    player: Player,
}

impl GameData {
    pub fn new() -> Self {
        let tile_map = TileMap::by_hand();

        let mut player = Player::new();
        player.random_location(&tile_map);

        let key_bindings: HashMap<KeyCode, Action> = HashMap::from([
            (KeyCode::W, Action::MoveStraight),
            (KeyCode::S, Action::MoveBack),
            (KeyCode::A, Action::MoveLeft),
            (KeyCode::D, Action::MoveRight),
            (KeyCode::E, Action::RotateRight),
            (KeyCode::Q, Action::RotateLeft),
        ]);

        Self {
            key_bindings,
            tile_map,
            player,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ContactPlane {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct ContactPoint {
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

fn get_particle_contact_point(
    particle_position: &Vec2,
    angle: f32,
    tile_map: &TileMap,
) -> ContactPoint {
    const DISTANCE_STEP: f32 = 0.05;
    let mut particle_position = particle_position.clone();
    let mut counter: u32 = 0;
    let mut value = 0;
    while tile_map.is_tile_empty(particle_position.y as usize, particle_position.x as usize) {
        if counter % 2 == 0 {
            particle_position.x += angle.cos() * DISTANCE_STEP;
        } else {
            particle_position.y += angle.sin() * DISTANCE_STEP;
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

fn get_particles_in_view(player: &Player, tile_map: &TileMap) -> Vec<ContactPoint> {
    const ANGLE_STEP: f32 = 0.01;
    let start_angle = player.rotation - player.fov / 2.0;
    let end_angle = player.rotation + player.fov / 2.0;

    let mut current_angle = start_angle;

    let mut angles: Vec<f32> = Vec::new();
    while current_angle < end_angle {
        angles.push(current_angle);
        current_angle += ANGLE_STEP;
    }

    angles
        .iter()
        .map(|angle| get_particle_contact_point(&player.position, *angle, &tile_map))
        .collect()
}

fn get_tile_color(value: i8, plane: ContactPlane) -> Color {
    match (value,plane) { 
        (0,_) => Color::new(0.0,0.0,0.0,0.0),

        (1, ContactPlane::Vertical) => Color::from_rgba(255, 0, 0, 255),
        (1, ContactPlane::Horizontal) => Color::from_rgba(193, 0, 0, 255),

        (2, ContactPlane::Vertical) => Color::from_rgba(0, 255, 0, 255),
        (2, ContactPlane::Horizontal) => Color::from_rgba(0, 193, 0, 255),

        (3, ContactPlane::Vertical) => Color::from_rgba(0, 0, 255, 255),
        (3, ContactPlane::Horizontal) => Color::from_rgba(0, 0, 193, 255),
        _ => {todo!();}
    }
}

fn draw_minimap_rays_in_view(player: &Player, tile_map: &TileMap) {
    let player_position = player.position;
    for particle in get_particles_in_view(&player, &tile_map).iter() {
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

fn draw_walls(player: &Player, tile_map: &TileMap) {
    // FIXME black stripe on right
    let particles = get_particles_in_view(&player, &tile_map);
    let particle_count = particles.len() as f32;
    let rect_width = (particle_count / screen_width()) * LOGICAL_TO_PHYSICAL_SIZE;

    for (index, particle) in particles.iter().enumerate() {
        draw_rectangle(
            index as f32 * rect_width,
            0.0,
            rect_width,
            screen_height(),
            get_tile_color(particle.value, particle.plane),
        );
    }
}

fn draw_minimap_background(tile_map: &TileMap) {
    draw_rectangle(
        0.0,
        0.0,
        tile_map.width as f32 * MINIMAP_SCALE,
        tile_map.height as f32 * MINIMAP_SCALE,
        BLACK,
    );
}

fn minimap(tile_map: &TileMap, player: &Player) {
    draw_minimap_background(tile_map);
    tile_map.minimap_draw();
    player.minimap_draw(RED);
    draw_minimap_rays_in_view(&player, &tile_map);
}

fn scene(tile_map: &TileMap, player: &Player) {
    draw_walls(&player, &tile_map);
}

#[macroquad::main("Katzenstein")]
async fn main() {
    let game_data = GameData::new(); // TODO global resource system
    let mut player = game_data.player;
    let tile_map = game_data.tile_map;
    let key_bindings = game_data.key_bindings;

    loop {
        clear_background(BLACK);

        handle_movement_input(&mut player, &tile_map, &key_bindings);
        scene(&tile_map, &player);
        minimap(&tile_map, &player);

        next_frame().await
    }
}
