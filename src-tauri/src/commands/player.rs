use crate::services::player_manager;
use crate::services::player_manager::{PlayerEntry, BanEntry, OpEntry};
use std::sync::OnceLock;
use crate::services::server_manager::ServerManager;

fn manager() -> &'static ServerManager {
    static INSTANCE: OnceLock<ServerManager> = OnceLock::new();
    INSTANCE.get_or_init(ServerManager::new)
}


// ---- Read lists from files ----

#[tauri::command]
pub fn get_whitelist(server_path: String) -> Result<Vec<PlayerEntry>, String> {
    player_manager::read_whitelist(&server_path)
}

#[tauri::command]
pub fn get_banned_players(server_path: String) -> Result<Vec<BanEntry>, String> {
    player_manager::read_banned_players(&server_path)
}

#[tauri::command]
pub fn get_ops(server_path: String) -> Result<Vec<OpEntry>, String> {
    player_manager::read_ops(&server_path)
}

// ---- Modify via server console commands ----

#[tauri::command]
pub fn add_to_whitelist(server_id: String, name: String) -> Result<String, String> {
    let cmd = format!("whitelist add {}", name);
    manager().send_command(&server_id, &cmd)?;
    Ok(format!("Sent: {}", cmd))
}

#[tauri::command]
pub fn remove_from_whitelist(server_id: String, name: String) -> Result<String, String> {
    let cmd = format!("whitelist remove {}", name);
    manager().send_command(&server_id, &cmd)?;
    Ok(format!("Sent: {}", cmd))
}

#[tauri::command]
pub fn ban_player(server_id: String, name: String, reason: String) -> Result<String, String> {
    let cmd = if reason.is_empty() {
        format!("ban {}", name)
    } else {
        format!("ban {} {}", name, reason)
    };
    manager().send_command(&server_id, &cmd)?;
    Ok(format!("Sent: {}", cmd))
}

#[tauri::command]
pub fn unban_player(server_id: String, name: String) -> Result<String, String> {
    let cmd = format!("pardon {}", name);
    manager().send_command(&server_id, &cmd)?;
    Ok(format!("Sent: {}", cmd))
}

#[tauri::command]
pub fn add_op(server_id: String, name: String) -> Result<String, String> {
    let cmd = format!("op {}", name);
    manager().send_command(&server_id, &cmd)?;
    Ok(format!("Sent: {}", cmd))
}

#[tauri::command]
pub fn remove_op(server_id: String, name: String) -> Result<String, String> {
    let cmd = format!("deop {}", name);
    manager().send_command(&server_id, &cmd)?;
    Ok(format!("Sent: {}", cmd))
}

#[tauri::command]
pub fn kick_player(server_id: String, name: String, reason: String) -> Result<String, String> {
    let cmd = if reason.is_empty() {
        format!("kick {}", name)
    } else {
        format!("kick {} {}", name, reason)
    };
    manager().send_command(&server_id, &cmd)?;
    Ok(format!("Sent: {}", cmd))
}

#[tauri::command]
pub fn export_logs(logs: Vec<String>, save_path: String) -> Result<(), String> {
    let content = logs.join("\n");
    std::fs::write(&save_path, content)
        .map_err(|e| format!("Failed to write: {}", e))
}
