#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_variables)]

use macroquad::prelude::*;

/*
    # - wall
    . - floor
    $ - $ - container
    ~ - target
    @ - player
    * - container on target
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

    let texture = Texture2D::from_file_with_format( include_bytes!("../assets/sprites/yoshi-32-box.png"), None);
    
    texture.set_filter(FilterMode::Nearest);
    
    loop {
        clear_background(WHITE);
        
        draw_texture(&texture, 50.0, 70.0, WHITE);
        
        next_frame().await;
    }
}