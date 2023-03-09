use macroquad::prelude::*;
use macroquad::ui::{
    root_ui,
    widgets, 
};
use input_handler::{InputHandler, un_pause_game};
use map::TileMap;
use player::Player;
use raycasting::draw_walls;

mod input_handler;
mod map;
mod player;
mod raycasting;

pub fn pause_menu(game_state: &mut GameState) {
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(0, 0, 0, 128));


    let window_style = root_ui()
            .style_builder()
            .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
            .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
            .build();
 

    let button_pressed = widgets::Button::new("Click me")
        .position(Vec2::new(200.,200.))
        .ui(&mut *root_ui());
    
    if button_pressed {
        un_pause_game(game_state);
    }

}

pub enum GameState {
    MainMenu,
    InGame,
    Pause,
    LevelEditor,
}

struct State {
    player: Player,
    tile_map: TileMap,
    input_handler: InputHandler,
    game_state: GameState 
}

impl State {
    pub fn new() -> Self {
        let mut tile_map = TileMap::new(50, 50);
        tile_map.generate(5);

        let mut player = Player::new();
        player.random_location(&tile_map);

        let input_handler = InputHandler::new(); 
        // TODO: change
        let game_state = GameState::InGame;

        Self {
            player,
            tile_map,
            input_handler,
            game_state,
        }
    }

    pub fn run(&mut self){
        self.input_handler.handle_global_input(&mut self.game_state);
        draw_walls(&self.player, &self.tile_map);
        match self.game_state {
            GameState::InGame => { 
                self.input_handler.handle_player_input(&mut self.player, &self.tile_map);
            },
            GameState::Pause => {
                pause_menu(&mut self.game_state);
            }, 
            GameState::MainMenu => {

            }, 
            GameState::LevelEditor => {

            },
        }

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
