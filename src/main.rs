#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_variables)]
#![allow(dead_code)]

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

const LEVEL_SCREEN_POS_X: f32 = 50.0;
const LEVEL_SCREEN_POS_Y: f32 = 70.0;

struct GameLevel {
    data: Vec<String>,
    width: usize,
    height: usize,
}

impl GameLevel {
    fn new(level_data: Vec<String>) -> Self {
        let height = level_data.len();
        let width = if height > 0 {
            level_data[0].len()
        } else {
            0
        };

        // Validate that all rows have the same length
        for row in &level_data {
            if row.len() != width {
                panic!("Level rows must all have the same length");
            }
        }

        Self {
            data: level_data,
            width,
            height,
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<char> {
        if x < self.width && y < self.height {
            self.data[y].chars().nth(x)
        } else {
            None
        }
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: char) -> Result<(), &'static str> {
        if x >= self.width || y >= self.height {
            return Err("Coordinates out of bounds");
        }

        let mut row_chars: Vec<char> = self.data[y].chars().collect();
        row_chars[x] = cell;
        self.data[y] = row_chars.into_iter().collect();
        
        Ok(())
    }

    fn is_valid_position(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

struct Game {
    level: GameLevel,
    textures: GameTextures,
}

impl Game {
    fn new() -> Self {
        let default_level_data = string_vec![
            "#######",
            "#~....#",
            "#.$...#",
            "#.@...#",
            "#######"
        ];

        let level = GameLevel::new(default_level_data);

        let textures = GameTextures {
            wall: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/yoshi-32-wall.png"),
                None,
            ),
            floor: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/yoshi-32-floor.png"),
                None,
            ),
            target: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/yoshi-32-dock.png"),
                None,
            ),
            crate_texture: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/yoshi-32-box.png"),
                None,
            ),
            crate_on_target: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/yoshi-32-box-docked.png"),
                None,
            ),
            player: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/yoshi-32-worker.png"),
                None,
            ),
            player_on_target: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/yoshi-32-worker-docked.png"),
                None,
            ),
        };

        println!("Level size: {}x{} cells", level.width, level.height);
        println!("{} {}", textures.wall.width(), textures.wall.height());

        Self {
            level,
            textures,
        }
    }

    fn load_level(&mut self, level_data: Vec<String>) {
        self.level = GameLevel::new(level_data);
        println!("Loaded new level: {}x{} cells", self.level.width, self.level.height);
    }

    fn render(&self) {
        for y in 0..self.level.height {
            for x in 0..self.level.width {
                if let Some(cell_char) = self.level.get_cell(x, y) {
                    let cell_texture = match cell_char {
                        '#' => &self.textures.wall,
                        '.' => &self.textures.floor,
                        '~' => &self.textures.target,
                        '$' => &self.textures.crate_texture,
                        '*' => &self.textures.crate_on_target,
                        '@' => &self.textures.player,
                        '+' => &self.textures.player_on_target,
                        _ => &self.textures.floor, // Default to floor for unknown characters
                    };
                    
                    draw_texture(
                        cell_texture,
                        LEVEL_SCREEN_POS_X + x as f32 * self.textures.wall.width(),
                        LEVEL_SCREEN_POS_Y + y as f32 * self.textures.wall.height(),
                        WHITE,
                    );
                }
            }
        }
    }

    fn update(&mut self) {
        // TODO: Add game update logic (player movement, box pushing, etc.)
    }

}

struct GameTextures {
    wall: Texture2D,
    floor: Texture2D,
    target: Texture2D,
    crate_texture: Texture2D,
    crate_on_target: Texture2D,
    player: Texture2D,
    player_on_target: Texture2D,
}

#[macroquad::main("Sokoban")]
async fn main() {
    let mut game = Game::new();

    // Example of loading a different level
    // let new_level = string_vec![
    //     "########",
    //     "#.~.~..#",
    //     "#.$$$$.#",
    //     "#.@....#",
    //     "########"
    // ];
    // game.load_level(new_level);

    loop {
        clear_background(WHITE);
        
        game.update();
        game.render();
        
        next_frame().await;
    }
}