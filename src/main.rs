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

    fn get_cell(&self, row: usize, col: usize) -> Option<char> {
        if col < self.width && row < self.height {
            self.data[row].chars().nth(col)
        } else {
            None
        }
    }

    fn set_cell(&mut self, row: usize, col: usize, cell: char) -> Result<(), &'static str> {
        if col >= self.width || row >= self.height {
            return Err("Coordinates out of bounds");
        }

        let mut row_chars: Vec<char> = self.data[row].chars().collect();
        row_chars[col] = cell;
        self.data[row] = row_chars.into_iter().collect();
        
        Ok(())
    }

    fn is_valid_position(&self, row: usize, col: usize) -> bool {
        col < self.width && row < self.height
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    // (offset_row, offset_col)
    pub fn to_offset(&self) -> (i32, i32) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }
    
    pub fn invert(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

struct Game {
    level: GameLevel,
    textures: GameTextures,
    player_pos: (usize, usize),
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
                include_bytes!("../assets/sprites/lager-20-wall.png"),
                None,
            ),
            floor: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/lager-20-floor.png"),
                None,
            ),
            target: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/lager-20-dock.png"),
                None,
            ),
            crate_texture: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/lager-20-box.png"),
                None,
            ),
            crate_on_target: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/lager-20-box-docked.png"),
                None,
            ),
            player: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/lager-20-worker.png"),
                None,
            ),
            player_on_target: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/lager-20-worker-docked.png"),
                None,
            ),
        };

        let mut player_pos = (0, 0);
        for row in 0..level.height {
            for col in 0..level.width {
                if let Some(c) = level.get_cell(row, col) {
                    if c == '@' || c == '+' {
                        player_pos = (row, col);
                    }
                }
            }
        }

        println!("Level size: {}x{} cells, player pos: {} {}", level.width, level.height, player_pos.0, player_pos.1);
        println!("{} {}", textures.wall.width(), textures.wall.height());

        Self {
            level,
            textures,
            player_pos,
        }
    }

    fn load_level(&mut self, level_data: Vec<String>) {
        self.level = GameLevel::new(level_data);
        println!("Loaded new level: {}x{} cells", self.level.width, self.level.height);
    }

    fn render(&self) {
        for row in 0..self.level.height {
            for col in 0..self.level.width {
                if let Some(cell_char) = self.level.get_cell(row, col) {
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
                    
                    draw_texture_ex(
                        cell_texture,
                        LEVEL_SCREEN_POS_X + col as f32 * self.textures.wall.width() * 1.5,
                        LEVEL_SCREEN_POS_Y + row as f32 * self.textures.wall.height() * 1.5,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(
                                self.textures.wall.width() * 1.5,
                                self.textures.wall.height() * 1.5,
                            )),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }

    fn make_move(&mut self, dir: Direction) {
        println!("{:?}", dir);
        // TODO
    }


    // fn update(&mut self) {
    //     // is it really needed?
    // }

}

// TODO: IDEAS: make background not pure white but seiled with some gray brick like pattern? 

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

        if is_key_pressed(KeyCode::Left) {
            // println!("Left");
            game.make_move(Direction::Left);
        }

        if is_key_pressed(KeyCode::Right) {
            // println!("Right"); // TODO
            game.make_move(Direction::Right);
        }

        if is_key_pressed(KeyCode::Down) {
            // println!("Down"); // TODO
            game.make_move(Direction::Down);
        }

        if is_key_pressed(KeyCode::Up) {
            // println!("Up"); // TODO
            game.make_move(Direction::Up);
        }

        // game.update();
        game.render();
        
        next_frame().await;
    }
}