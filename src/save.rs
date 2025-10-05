use serde::{Deserialize, Serialize};
use std::fs;

const SAVE_FILE: &str = "sokoban_save.dat";

#[derive(Serialize, Deserialize, Default)]
pub struct SaveData {
    max_unlocked_level: usize,
}

impl SaveData {
    pub fn new() -> Self {
        match fs::read_to_string(SAVE_FILE) {
            Ok(contents) => {
                match serde_json::from_str(&contents) {
                    Ok(save_data) => save_data,
                    Err(e) => {
                        println!("Save file corrupted: {}, using default", e);
                        Self::default()
                    }
                }
            }
            Err(_) => {
                println!("No save file found, creating default");
                let default_save = Self::default();
                default_save.save().unwrap_or_else(|e| {
                    eprintln!("Failed to create save file: {}", e);
                });
                default_save
            }
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(SAVE_FILE, json)?;
        //println!(" saved to {} (level: {})", SAVE_FILE, self.max_unlocked_level);
        Ok(())
    }

    pub fn set_max_unlocked_level(&mut self, level_index: usize) {
        self.max_unlocked_level = level_index;
        let _ = self.save();
    }

    pub fn get_max_unlocked_level(&self) -> usize {
        self.max_unlocked_level
    }
}