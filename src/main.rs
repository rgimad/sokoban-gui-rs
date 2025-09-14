#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

    loop {
        clear_background(WHITE);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await 
    }
}