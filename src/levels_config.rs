use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LevelConfigData {
    pub name: String,
    pub data: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelsConfigInternal {
    pub levels: Vec<LevelConfigData>,
}

pub struct LevelsConfig {
    pub level_names: Vec<String>,
    levels: LevelsConfigInternal,
}

impl LevelsConfig {
    pub fn new() -> Self {
        let levels_bytes = include_bytes!("../assets/levels/levels.json");
        let config: LevelsConfigInternal = serde_json::from_slice(levels_bytes).unwrap();
        
        Self {
            level_names: config.levels
            .iter()
            .enumerate()
            .map(|(index, level)| format!("{}. {}", index + 1, level.name))
            .collect(),
            levels: config
        }
    }

    pub fn get_level(&self, index: usize) -> Option<&LevelConfigData> {
        self.levels.levels.get(index)
    }

    pub fn total_levels(&self) -> usize {
        self.levels.levels.len()
    }
}
