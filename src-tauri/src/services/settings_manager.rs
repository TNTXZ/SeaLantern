use crate::models::settings::AppSettings;
use std::sync::Mutex;

const SETTINGS_FILE: &str = "sea_lantern_settings.json";

pub struct SettingsManager {
    pub settings: Mutex<AppSettings>,
    pub data_dir: String,
}

impl SettingsManager {
    pub fn new() -> Self {
        let data_dir = get_data_dir();
        let settings = load_settings(&data_dir);
        SettingsManager {
            settings: Mutex::new(settings),
            data_dir,
        }
    }

    pub fn get(&self) -> AppSettings {
        self.settings.lock().unwrap().clone()
    }

    pub fn update(&self, new_settings: AppSettings) -> Result<(), String> {
        *self.settings.lock().unwrap() = new_settings.clone();
        save_settings(&self.data_dir, &new_settings)
    }

    pub fn reset(&self) -> Result<AppSettings, String> {
        let default = AppSettings::default();
        *self.settings.lock().unwrap() = default.clone();
        save_settings(&self.data_dir, &default)?;
        Ok(default)
    }
}

fn get_data_dir() -> String {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            return dir.to_string_lossy().to_string();
        }
    }
    ".".to_string()
}

fn load_settings(data_dir: &str) -> AppSettings {
    let path = std::path::Path::new(data_dir).join(SETTINGS_FILE);
    if !path.exists() {
        let default = AppSettings::default();
        let _ = save_settings(data_dir, &default);
        return default;
    }
    match std::fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => AppSettings::default(),
    }
}

fn save_settings(data_dir: &str, settings: &AppSettings) -> Result<(), String> {
    let path = std::path::Path::new(data_dir).join(SETTINGS_FILE);
    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    std::fs::write(&path, json)
        .map_err(|e| format!("Failed to save settings: {}", e))
}
