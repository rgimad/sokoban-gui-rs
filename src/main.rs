#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_variables)]
#![allow(dead_code)]

use macroquad::{miniquad::window::set_window_size, prelude::*};

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

const LEVEL_SCREEN_POS_X: usize = 50;
const LEVEL_SCREEN_POS_Y: usize = 40;
const TEXTURE_SCALE_COEF: f32 = 1.5;

struct GameLevel {
    data: Vec<String>,
    width: usize,
    height: usize,
}

impl GameLevel {

    fn new() -> Self {
        Self {
            data: Vec::<String>::new(),
            width: 0,
            height: 0,
        }
    }

    fn from(level_data: Vec<String>) -> Self {
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

    fn get_cell(&self, rowcol: (usize, usize)) -> Option<char> {
        let row = rowcol.0;
        let col = rowcol.1;
        if col < self.width && row < self.height {
            self.data[row].chars().nth(col)
        } else {
            None
        }
    }

    fn set_cell(&mut self, rowcol: (usize, usize), cell: char) -> Result<(), &'static str> {
        let row = rowcol.0;
        let col = rowcol.1;
        if col >= self.width || row >= self.height {
            return Err("Coordinates out of bounds");
        }

        let mut row_chars: Vec<char> = self.data[row].chars().collect();
        row_chars[col] = cell;
        self.data[row] = row_chars.into_iter().collect();
        
        Ok(())
    }

    fn is_valid_position(&self, rowcol: (isize, isize)) -> bool {
        let row = rowcol.0;
        let col = rowcol.1;
        return col >= 0 && col < (self.width as isize) && row >= 0 && row < (self.height as isize);
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
    background: Texture2D,
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
    pub fn to_offset(&self) -> (isize, isize) {
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
    boxes_total: usize,
    boxes_on_target: usize,
}

impl Game {
    fn new() -> Self {
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
            background: Texture2D::from_file_with_format(
                include_bytes!("../assets/sprites/background.png"),
                None,
            ), 
        };

        Self {
            level: GameLevel::new(),
            textures,
            player_pos: (0, 0),
            boxes_total: 0,
            boxes_on_target: 0,
        }
    }

    fn get_initial_player_pos(&self) -> Result<(usize, usize), &'static str> {
        for row in 0..self.level.height {
            for col in 0..self.level.width {
                if let Some(c) = self.level.get_cell((row, col)) {
                    if c == '@' || c == '+' {
                        return Ok((row, col));
                    }
                }
            }
        }
        Err("Player not found on level map")
    }

    fn get_boxes_count(&self) -> (usize, usize) {
        let mut total = 0usize;
        let mut on_target = 0usize;
        for row in 0..self.level.height {
            for col in 0..self.level.width {
                match self.level.get_cell((row, col)) {
                    Some('$') => {
                        total += 1;
                    },
                    Some('*') => {
                        total += 1;
                        on_target += 1;
                    },
                    _ => {},
                }
            }
        }
        (total, on_target)
    }

    fn _adjust_window_size(&self) {
        set_window_size(
            (self.level.width * (self.textures.wall.width()*TEXTURE_SCALE_COEF) as usize + LEVEL_SCREEN_POS_X*2) as u32,
            (self.level.height * (self.textures.wall.height()*TEXTURE_SCALE_COEF) as usize + LEVEL_SCREEN_POS_Y*2  + if cfg!(target_os = "macos") {25} else {0}) as u32
        );
    }

    fn load_level(&mut self, level_data: Vec<String>) {
        self.level = GameLevel::from(level_data);
        self.player_pos = self.get_initial_player_pos().unwrap();
        self._adjust_window_size();
        (self.boxes_total, self.boxes_on_target) = self.get_boxes_count();
        println!("Loaded new level: {}x{} cells, boxes total: {}, boxes on target: {}", self.level.width, self.level.height, self.boxes_total, self.boxes_on_target);
    }

    fn render(&self) {
        for row in 0..50 {
            for col in 0..50 {
                draw_texture_ex(
                        &self.textures.background,
                        col as f32 * self.textures.wall.width() * TEXTURE_SCALE_COEF,
                        row as f32 * self.textures.wall.height() * TEXTURE_SCALE_COEF,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(
                                self.textures.background.width() * TEXTURE_SCALE_COEF,
                                self.textures.background.height() * TEXTURE_SCALE_COEF,
                            )),
                            ..Default::default()
                        },
                    );
            }
        }

        for row in 0..self.level.height {
            for col in 0..self.level.width {
                if let Some(cell_char) = self.level.get_cell((row, col)) {
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
                        LEVEL_SCREEN_POS_X as f32 + col as f32 * self.textures.wall.width() * TEXTURE_SCALE_COEF,
                        LEVEL_SCREEN_POS_Y as f32 + row as f32 * self.textures.wall.height() * TEXTURE_SCALE_COEF,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(
                                self.textures.wall.width() * TEXTURE_SCALE_COEF,
                                self.textures.wall.height() * TEXTURE_SCALE_COEF,
                            )),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }

    fn make_move(&mut self, dir: Direction) {
        // println!("{:?}", dir);
        let prev_pos = self.player_pos;
        let cell_prev_pos = self.level.get_cell(prev_pos).unwrap();
        let offset = dir.to_offset();
        let next_pos = (self.player_pos.0 as isize + offset.0, self.player_pos.1 as isize + offset.1);
        if !self.level.is_valid_position(next_pos) {
            return;
        }
        let next_pos = (next_pos.0 as usize, next_pos.1 as usize);
        let cell_next_pos = self.level.get_cell(next_pos).unwrap();
        match cell_next_pos {
            '.' => {
                self.level.set_cell(next_pos, '@').unwrap();
                self.level.set_cell(prev_pos, if cell_prev_pos == '@' {'.'} else {'~'}).unwrap();
                self.player_pos = next_pos;
            },
            '~' => {
                self.level.set_cell(next_pos, '+').unwrap();
                self.level.set_cell(prev_pos, if cell_prev_pos == '@' {'.'} else {'~'}).unwrap();
                self.player_pos = next_pos;
            },
            '$' | '*' => {
                let box_next_pos = (self.player_pos.0 as isize + offset.0*2, self.player_pos.1 as isize + offset.1*2);
                if !self.level.is_valid_position(box_next_pos) {
                    return;
                }
                let box_next_pos = (box_next_pos.0 as usize, box_next_pos.1 as usize);
                let cell_box_next_pos = self.level.get_cell(box_next_pos).unwrap();
                match cell_box_next_pos {
                    '.' | '~' => {
                        self.level.set_cell(box_next_pos, if cell_box_next_pos == '.' {'$'} else {'*'}).unwrap();
                        self.level.set_cell(next_pos, if cell_next_pos == '$' {'@'} else {'+'}).unwrap();
                        self.level.set_cell(prev_pos, if cell_prev_pos == '@' {'.'} else {'~'}).unwrap();
                        self.player_pos = next_pos;
                    },
                    _ => {}
                }
            },
            _ => {},
        }
        
    }


    // fn update(&mut self) {
    //     // is it really needed?
    // }

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
    game.load_level(
        string_vec![
            "#######",
            "#~..$~#",
            "#.*~..#",
            "#.$.$.#",
            "#.@.$~#",
            "#######"
        ]
    );

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