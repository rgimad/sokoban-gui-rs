use macroquad::prelude::*;

pub struct GameTextures {
    pub wall: Texture2D,
    pub floor: Texture2D,
    pub target: Texture2D,
    pub crate_texture: Texture2D,
    pub crate_on_target: Texture2D,
    pub player: Texture2D,
    pub player_on_target: Texture2D,
    pub background: Texture2D,
}

impl GameTextures {
    pub fn new() -> Self {
        Self  {
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
        }
    }
}

