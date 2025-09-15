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

    let box_texture = Texture2D::from_file_with_format( include_bytes!("../assets/sprites/yoshi-32-box.png"), None);

    println!("{} {}", box_texture.width(), box_texture.height());
    
    box_texture.set_filter(FilterMode::Nearest);

    // TODO render level with sprites
    // Firstly, do this in main, after that move this logic to a separate function
    
    loop {
        clear_background(WHITE);
        
        // draw_texture(&texture, 50.0, 70.0, WHITE);

        for (row_idx, row_str) in level.iter().enumerate() {
            for (col_idx, cell_char) in row_str.chars().enumerate() {
                match cell_char {
                    '#' => {
                        draw_texture(&box_texture, 50. + col_idx as f32 * box_texture.width(), 70.0 + row_idx as f32 * box_texture.height(), WHITE);
                    },
                    _ => {},
                }
            }
        }
        
        next_frame().await;
    }
}