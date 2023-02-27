use input_handler::InputHandler;
use macroquad::prelude::*;
use map::TileMap;
use player::Player;
use renderer::{Renderer, Minimap, MinimapObject};

mod input_handler;
mod map;
mod player;
mod raycasting;
mod renderer;

struct State {
    player: Player,
    tile_map: TileMap,
    input_handler: InputHandler,
    minimap: Minimap,
}

impl State {
    pub fn new() -> Self {
        let mut tile_map = TileMap::new(50, 50);
        tile_map.generate(5);

        let mut player = Player::new();
        player.random_location(&tile_map);

        let input_handler = InputHandler::new();

        let minimap = Minimap::new();

        Self {
            player,
            tile_map,
            input_handler,
            minimap,
        }
    }

    pub fn run(&mut self) {
        self.input_handler.handle_keyboard_input(&mut self.player, &self.tile_map);

        let renderer = Renderer::new(&self.player, &self.tile_map);
        renderer.draw_world();

        self.minimap.update(&self.player, self.get_minmap_objects());
    }

    fn get_minmap_objects(&self) -> Vec<Box<dyn MinimapObject>> {
        let mut objects: Vec<Box<dyn MinimapObject>> = Vec::new();
        for tile in self.tile_map.tiles.iter().flatten() {
            objects.push(Box::new(*tile));
        } 
        objects.push(Box::new(self.player));
        objects
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Katzenstein".to_string(),
        sample_count: 4,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = State::new();
    loop {
        clear_background(BLACK);
        game.run();
        next_frame().await
    }
}
