use crate::raycasting::ContactPlane;
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use rand_distr::Normal;

const TILE_SIZE: f32 = 1.0;

pub struct Tile {
    pub value: i8,
    pub size: Vec2,
    position: Vec2,
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

pub fn get_color(value: i8, plane: ContactPlane) -> Color {
    match (value, plane) {
        (0, _) => WHITE,

        (1, ContactPlane::Vertical) => Color::from_rgba(255, 0, 0, 255),
        (1, ContactPlane::Horizontal) => Color::from_rgba(193, 0, 0, 255),

        (2, ContactPlane::Vertical) => Color::from_rgba(0, 255, 0, 255),
        (2, ContactPlane::Horizontal) => Color::from_rgba(0, 193, 0, 255),

        (3, ContactPlane::Vertical) => Color::from_rgba(0, 0, 255, 255),
        (3, ContactPlane::Horizontal) => Color::from_rgba(0, 0, 193, 255),
        _ => {todo!();}
    }
}



#[derive(Debug, Clone, Copy)]
struct TilePosition {
    pub y: usize,
    pub x: usize,
}

impl TilePosition {
    pub fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

pub struct TileMap {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}

impl TileMap {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();

        for row_index in 0..height {
            let mut row_vec: Vec<Tile> = Vec::new();
            for column_index in 0..width {
                row_vec.push(Tile::new(
                    1,
                    Vec2::new(row_index as f32, column_index as f32),
                ));
            }
            tiles.push(row_vec);
        }
        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn generate(&mut self, room_count: u32) {
        let mut room_centers: Vec<TilePosition> = Vec::new();

        let normal = Normal::new(10.0, 2.0).unwrap();
        for _ in 0..room_count {
            let room_size = thread_rng().sample(normal) as usize;
            let position_y = thread_rng().gen_range(0..(self.height - room_size));
            let position_x = thread_rng().gen_range(0..(self.width - room_size));

            let position = TilePosition::new(position_y, position_x);
            let size = TilePosition::new(room_size, room_size);

            self.carve_room(position, size);
            room_centers.push(TilePosition::new(
                position_y + room_size / 2,
                position_x + room_size / 2,
            ));
        }

        for (x1, x2) in room_centers.iter().zip(room_centers.iter().skip(1)) {
            self.connect_rooms(*x1, *x2);
        }
    }

    fn carve_room(&mut self, position: TilePosition, size: TilePosition) {
        for y in position.y..(size.y + position.y) {
            for x in position.x..(size.x + position.x) {
                self.tiles[y][x].value = 0;
            }
        }
    }

    fn connect_rooms(&mut self, center1: TilePosition, center2: TilePosition) {
        let direction = |pos1, pos2| (pos2 as isize - pos1 as isize).signum();

        let mut current_x = center1.x;
        while direction(current_x, center2.x) != 0 {
            current_x = (current_x as isize + direction(current_x, center2.x)) as usize;
            self.tiles[center1.y][current_x].value = 0;
        }

        let mut current_y = center1.y;
        while direction(current_y, center2.y) != 0 {
            current_y = (current_y as isize + direction(current_y, center2.y)) as usize;
            self.tiles[current_y][current_x].value = 0;
        }
    }

    pub fn is_tile_empty(&self, column_index: usize, row_index: usize) -> bool {
        let tile_value = self.get_tile_value(column_index, row_index);
        match tile_value {
            0 => true,
            _ => false,
        }
    }

    pub fn get_tile_value(&self, column_index: usize, row_index: usize) -> i8 {
        if let Some(row_vec) = self.tiles.get(row_index) {
            if let Some(tile) = row_vec.get(column_index) {
                return tile.value;
            }
        }
        1
    }
}
