use macroquad::prelude::*;
use crate::{DrawToMinimap,TILE_SIZE};
use ::rand::{thread_rng, Rng};
use rand_distr::Normal;

pub struct Tile {
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
                row_vec.push(Tile::new(1, Vec2::new(row_index as f32, column_index as f32)));
            }
            tiles.push(row_vec);
        }

        Self { tiles, width, height }
    }

    pub fn generate(&mut self, room_count: u32) {
        let mut room_positions: Vec<[usize;2]> = Vec::new();

        let normal = Normal::new(10.0, 2.0).unwrap(); 
        for _ in 0..room_count {
            let room_size = thread_rng().sample(normal) as usize;
            let position_x = thread_rng().gen_range(0..(self.width - room_size));
            let position_y = thread_rng().gen_range(0..(self.height - room_size));

            let position = [position_y,position_x];
            
            self.carve_room(position, [room_size,room_size]);
            room_positions.push(position);
        }

        println!("here");

        // for (x1,x2) in room_positions.iter().zip(room_positions.iter().skip(1)) {
        //     self.connect_rooms(*x1,*x2)
        // }
    }

    fn carve_room(&mut self, position: [usize;2], size: [usize;2]) {
        for y in position[0]..(size[0] + position[0]) {
            for x in position[1]..(size[1] + position[1]) {
                self.tiles[y][x].value = 0;
            }
        }
    }

    fn connect_rooms(&mut self, center1: [usize;2], center2: [usize;2]) {
        let x_distance = center1[0] as isize - center2[0] as isize;
        let y_distance = center1[1] as isize - center2[1] as isize;
        
        self.carve_room(center1, [1, center1[1] + x_distance as usize]);
        self.carve_room(center1, [center1[0] + y_distance as usize, 1]);
    }

    pub fn from_hand() -> Self {
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

    pub fn minimap_draw(&self) {
        for row in self.tiles.iter() {
            for value in row.iter() {
                value.minimap_draw(WHITE);
            }
        }
    }

    pub fn is_tile_empty(&self, column_index: usize, row_index: usize) -> bool {
        let tile_value = self.get_tile_value(column_index, row_index);
        match tile_value {
            0 => true,
            _ => false
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

