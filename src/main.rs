#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_variables)]

use macroquad::prelude::*;

/*
    # - wall
    . - floor
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


#[macroquad::main("Sokoban")]
async fn main() {

    let level = string_vec![
        "#######",
        "#~....#",
        "#.$...#",
        "#.@...#",
        "#######"
    ];

    let texture_wall = Texture2D::from_file_with_format( include_bytes!("../assets/sprites/yoshi-32-wall.png"), None);
    //texture_box.set_filter(FilterMode::Nearest);
    let texture_floor = Texture2D::from_file_with_format( include_bytes!("../assets/sprites/yoshi-32-floor.png"), None);
    let texture_target = Texture2D::from_file_with_format( include_bytes!("../assets/sprites/yoshi-32-dock.png"), None);
    let texture_box = Texture2D::from_file_with_format( include_bytes!("../assets/sprites/yoshi-32-box.png"), None);
    let texture_player = Texture2D::from_file_with_format( include_bytes!("../assets/sprites/yoshi-32-worker.png"), None);

    println!("{} {}", texture_wall.width(), texture_wall.height());

    // TODO render level with sprites
    // Firstly, do this in main, after that move this logic to a separate function
    
    loop {
        clear_background(WHITE);

        const START_X: f32 = 50.;
        const START_Y: f32 = 70.;

        for (row_idx, row_str) in level.iter().enumerate() {
            for (col_idx, cell_char) in row_str.chars().enumerate() {
                let mut cell_texture = &texture_floor;
                match cell_char {
                    '#' => {
                        cell_texture = &texture_wall;
                    },
                    '.' => {
                        cell_texture = &texture_floor;
                    },
                    '~' => {
                        cell_texture = &texture_target;
                    },
                    '$' => {
                        cell_texture = &texture_box;
                    },
                    '@' => {
                        cell_texture = &texture_player;
                    },
                    // TODO other types of cells
                    _ => {},
                }
                draw_texture(cell_texture, START_X + col_idx as f32 * texture_wall.width(), START_Y + row_idx as f32 * texture_wall.height(), WHITE);
            } 
        }
        
        next_frame().await;
    }
}