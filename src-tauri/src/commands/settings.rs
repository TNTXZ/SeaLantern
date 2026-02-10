use std::sync::OnceLock;
use crate::models::settings::AppSettings;
use crate::services::settings_manager::SettingsManager;

fn settings_mgr() -> &'static SettingsManager {
    static INSTANCE: OnceLock<SettingsManager> = OnceLock::new();
    INSTANCE.get_or_init(SettingsManager::new)
}

#[tauri::command]
pub fn get_settings() -> AppSettings {
    settings_mgr().get()
}

#[tauri::command]
pub fn save_settings(settings: AppSettings) -> Result<(), String> {
    settings_mgr().update(settings)
}

#[tauri::command]
pub fn reset_settings() -> Result<AppSettings, String> {
    settings_mgr().reset()
}

#[tauri::command]
pub fn export_settings() -> Result<String, String> {
    let s = settings_mgr().get();
    serde_json::to_string_pretty(&s).map_err(|e| format!("Export failed: {}", e))
}

#[tauri::command]
pub fn import_settings(json: String) -> Result<AppSettings, String> {
    let s: AppSettings = serde_json::from_str(&json)
        .map_err(|e| format!("Invalid JSON: {}", e))?;
    settings_mgr().update(s.clone())?;
    Ok(s)
}
