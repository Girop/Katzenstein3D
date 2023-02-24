use macroquad::prelude::*;
use crate::{DrawToMinimap,TILE_SIZE};

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

    pub fn is_tile_empty(&self, column_index: usize, row_index: usize) -> bool {
        if self.get_tile_value(column_index, row_index) == 0 {
            return true;
        }
        false
    }

    pub fn get_tile_value(&self, column_index: usize, row_index: usize) -> i8 {
        self.tiles[row_index][column_index].value
    }
}
