use crate::models::server::*;
use std::sync::OnceLock;
use crate::models::server::*;
use crate::services::server_manager::ServerManager;

fn manager() -> &'static ServerManager {
    static INSTANCE: OnceLock<ServerManager> = OnceLock::new();
    INSTANCE.get_or_init(ServerManager::new)
}


#[tauri::command]
pub fn create_server(
    name: String,
    core_type: String,
    mc_version: String,
    max_memory: u32,
    min_memory: u32,
    port: u16,
    java_path: String,
    jar_path: String,
) -> Result<ServerInstance, String> {
    let req = CreateServerRequest {
        name, core_type, mc_version, max_memory, min_memory, port, java_path, jar_path,
    };
    manager().create_server(req)
}

#[tauri::command]
pub fn import_server(
    name: String,
    jar_path: String,
    java_path: String,
    max_memory: u32,
    min_memory: u32,
) -> Result<ServerInstance, String> {
    let req = ImportServerRequest { name, jar_path, java_path, max_memory, min_memory };
    manager().import_server(req)
}

#[tauri::command]
pub fn start_server(id: String) -> Result<(), String> {
    manager().start_server(&id)
}

#[tauri::command]
pub fn stop_server(id: String) -> Result<(), String> {
    manager().stop_server(&id)
}

#[tauri::command]
pub fn send_command(id: String, command: String) -> Result<(), String> {
    manager().send_command(&id, &command)
}

#[tauri::command]
pub fn get_server_list() -> Vec<ServerInstance> {
    manager().get_server_list()
}

#[tauri::command]
pub fn get_server_status(id: String) -> ServerStatusInfo {
    manager().get_server_status(&id)
}

#[tauri::command]
pub fn delete_server(id: String) -> Result<(), String> {
    manager().delete_server(&id)
}

#[tauri::command]
pub fn get_server_logs(id: String, since: usize) -> Vec<String> {
    manager().get_logs(&id, since)
}
