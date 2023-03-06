use input_handler::InputHandler;
use macroquad::prelude::*;
use map::TileMap;
use player::Player;
use raycasting::draw_walls;

mod input_handler;
mod map;
mod player;
mod raycasting;

struct State {
    player: Player,
    tile_map: TileMap,
    input_handler: InputHandler,
}

impl State {
    pub fn new() -> Self {
        let mut tile_map = TileMap::new(50, 50);
        tile_map.generate(5);

        let mut player = Player::new();
        player.random_location(&tile_map);

        let input_handler = InputHandler::new();

        Self {
            player,
            tile_map,
            input_handler,
        }
    }

    pub fn run(&mut self) {
        self.input_handler.handle_keyboard_input(&mut self.player, &self.tile_map);
        draw_walls(&self.player, &self.tile_map);
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
