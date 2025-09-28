use macroquad::{miniquad::window::set_window_size, prelude::*};
use macroquad::ui::{hash, root_ui};
use crate::level::GameLevel;
use crate::textures::GameTextures;
use crate::direction::Direction;

pub struct Game {
    pub level: GameLevel,
    pub textures: GameTextures,
    pub player_pos: (usize, usize),
    pub boxes_total: usize,
    pub boxes_on_target: usize,
    pub moves: usize,
    cb_idx: usize,
}

const LEVEL_SCREEN_POS_X: usize = 50;
const LEVEL_SCREEN_POS_Y: usize = 50;
const TEXTURE_SCALE_COEF: f32 = 1.5;

impl Game {
    pub fn new() -> Self {
        Self {
            level: GameLevel::new(),
            textures: GameTextures::new(),
            player_pos: (0, 0),
            boxes_total: 0,
            boxes_on_target: 0,
            moves: 0,
            cb_idx: 0,
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

    fn adjust_window_size(&self) {
        set_window_size(
            (self.level.width * (self.textures.wall.width()*TEXTURE_SCALE_COEF) as usize + LEVEL_SCREEN_POS_X*2) as u32,
            (self.level.height * (self.textures.wall.height()*TEXTURE_SCALE_COEF) as usize + LEVEL_SCREEN_POS_Y*3/2  + if cfg!(target_os = "macos") {25} else {0}) as u32
        );
    }

    pub fn load_level(&mut self, level_data: Vec<String>) {
        self.level = GameLevel::from(level_data);
        self.player_pos = self.get_initial_player_pos().unwrap();
        self.adjust_window_size();
        (self.boxes_total, self.boxes_on_target) = self.get_boxes_count();
        self.moves = 0;
        println!("Loaded new level: {}x{} cells, boxes total: {}, boxes on target: {}", self.level.width, self.level.height, self.boxes_total, self.boxes_on_target);
    }

    pub fn render(&mut self) {
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
        // draw_text(&level_status, LEVEL_SCREEN_POS_X as f32, LEVEL_SCREEN_POS_Y as f32/3.0, 24.0, BLUE);
        // draw_text(&boxes_status, LEVEL_SCREEN_POS_X as f32, LEVEL_SCREEN_POS_Y as f32/3.0 + 20.0, 24.0, BLUE);
        // draw_text(&moves_status, LEVEL_SCREEN_POS_X as f32, LEVEL_SCREEN_POS_Y as f32/3.0 + 40.0, 24.0, BLUE);
        root_ui().combo_box(hash!(), "Level", &["opt 1", "opt 2", "opt 3", "opt 4"], &mut self.cb_idx);
        let status = format!("Boxes: {}/{}  Moves: {}", self.boxes_on_target, self.boxes_total, self.moves);
        root_ui().label(None, &status);
        
    }

    pub fn make_move(&mut self, dir: Direction) {
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
                self.moves += 1;
            },
            '~' => {
                self.level.set_cell(next_pos, '+').unwrap();
                self.level.set_cell(prev_pos, if cell_prev_pos == '@' {'.'} else {'~'}).unwrap();
                self.player_pos = next_pos;
                self.moves += 1;
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
                        self.level.set_cell(box_next_pos, if cell_box_next_pos == '.' {if cell_next_pos != '$' {self.boxes_on_target -= 1;} '$'} else {if cell_next_pos != '*' {self.boxes_on_target += 1;} '*'}).unwrap();
                        self.level.set_cell(next_pos, if cell_next_pos == '$' {'@'} else {'+'}).unwrap();
                        self.level.set_cell(prev_pos, if cell_prev_pos == '@' {'.'} else {'~'}).unwrap();
                        self.player_pos = next_pos;
                        self.moves += 1;
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