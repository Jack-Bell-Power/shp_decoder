use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct AppConfig {
    pub pal_path: String,
    pub shp_path: String,
    pub output_path: String,
    pub half_index: usize,
}

impl AppConfig {
    fn path() -> PathBuf {
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("config.json")
    }

    pub fn load() -> Self {
        let path = Self::path();

        if let Ok(data) = fs::read_to_string(path) {
            serde_json::from_str(&data)
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }


    pub fn save(&self) {

        let path = Self::path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }

        let data = serde_json::to_string_pretty(self)
            .unwrap();

        fs::write(path, data).ok();
    }
}
