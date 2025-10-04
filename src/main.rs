#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_variables)]
#![allow(dead_code)]

mod direction;
mod textures;
mod level;
mod levels_config;
mod save;
mod game;

use macroquad::prelude::*;
use direction::Direction;
use game::Game;

/*
    # - wall
    . - floor
    , - background
    $ - box
    ~ - target
    @ - player
    * - box on target
    + - player on target
*/

macro_rules! string_vec {
    ($($x:expr),* $(,)?) => {
        vec![$($x.to_string()),*]
    };
}

fn window_config() -> Conf {
    Conf {
        window_title: "Sokoban".to_string(),
        window_width: 50,
        window_height: 50,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut game = Game::new();
    game.load_level(game.levels_config.get_level(1).unwrap().data.clone()); // TODO load it ctor curr level

    loop {
        clear_background(WHITE);

        if is_key_pressed(KeyCode::Left) {
            game.make_move(Direction::Left);
        }

        if is_key_pressed(KeyCode::Right) {
            game.make_move(Direction::Right);
        }

        if is_key_pressed(KeyCode::Down) {
            game.make_move(Direction::Down);
        }

        if is_key_pressed(KeyCode::Up) {
            game.make_move(Direction::Up);
        }

        // game.update();
        game.render();
        
        next_frame().await;
    }
}