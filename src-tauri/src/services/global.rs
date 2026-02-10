use std::sync::OnceLock;
use super::server_manager::ServerManager;

static INSTANCE: OnceLock<ServerManager> = OnceLock::new();

pub fn manager() -> &'static ServerManager {
    INSTANCE.get_or_init(ServerManager::new)
}
