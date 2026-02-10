use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::server::*;
use crate::services::settings_manager::SettingsManager;

const DATA_FILE: &str = "sea_lantern_servers.json";

pub struct ServerManager {
    pub servers: Mutex<Vec<ServerInstance>>,
    pub processes: Mutex<HashMap<String, Child>>,
    pub logs: Mutex<HashMap<String, Vec<String>>>,
    pub data_dir: Mutex<String>,
}

impl ServerManager {
    pub fn new() -> Self {
        let data_dir = get_data_dir();
        let servers = load_servers(&data_dir);
        let mut logs_map = HashMap::new();
        for s in &servers { logs_map.insert(s.id.clone(), Vec::new()); }
        ServerManager {
            servers: Mutex::new(servers),
            processes: Mutex::new(HashMap::new()),
            logs: Mutex::new(logs_map),
            data_dir: Mutex::new(data_dir),
        }
    }

    fn save(&self) {
        let servers = self.servers.lock().unwrap();
        let data_dir = self.data_dir.lock().unwrap();
        save_servers(&data_dir, &servers);
    }

    fn get_app_settings(&self) -> crate::models::settings::AppSettings {
        SettingsManager::new().get()
    }

    pub fn create_server(&self, req: CreateServerRequest) -> Result<ServerInstance, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let jar_path_obj = std::path::Path::new(&req.jar_path);
        let server_dir = jar_path_obj.parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string());

        let server = ServerInstance {
            id: id.clone(), name: req.name, core_type: req.core_type,
            core_version: String::new(), mc_version: req.mc_version,
            path: server_dir, jar_path: req.jar_path, java_path: req.java_path,
            max_memory: req.max_memory, min_memory: req.min_memory,
            jvm_args: Vec::new(), port: req.port, created_at: now, last_started_at: None,
        };
        self.servers.lock().unwrap().push(server.clone());
        self.logs.lock().unwrap().insert(id, Vec::new());
        self.save();
        Ok(server)
    }

    pub fn import_server(&self, req: ImportServerRequest) -> Result<ServerInstance, String> {
        if !std::path::Path::new(&req.jar_path).exists() {
            return Err(format!("JAR file not found: {}", req.jar_path));
        }
        self.create_server(CreateServerRequest {
            name: req.name, core_type: "unknown".into(), mc_version: "unknown".into(),
            max_memory: req.max_memory, min_memory: req.min_memory,
            port: 25565, java_path: req.java_path, jar_path: req.jar_path,
        })
    }

    pub fn start_server(&self, id: &str) -> Result<(), String> {
        let server = {
            let servers = self.servers.lock().unwrap();
            servers.iter().find(|s| s.id == id)
                .ok_or_else(|| "Server not found".to_string())?.clone()
        };

        // Check if already running
        {
            let mut procs = self.processes.lock().unwrap();
            if let Some(child) = procs.get_mut(id) {
                match child.try_wait() {
                    Ok(Some(_)) => { procs.remove(id); } // Dead process, clean up
                    Ok(None) => return Err("Server is already running".to_string()),
                    Err(_) => { procs.remove(id); }
                }
            }
        }

        let settings = self.get_app_settings();
        if settings.auto_accept_eula {
            let eula = std::path::Path::new(&server.path).join("eula.txt");
            let _ = std::fs::write(&eula, "# Auto-accepted by Sea Lantern\neula=true\n");
        }

        let mut cmd = Command::new(&server.java_path);
        cmd.arg(format!("-Xmx{}M", server.max_memory));
        cmd.arg(format!("-Xms{}M", server.min_memory));
        let jvm = settings.default_jvm_args.trim();
        if !jvm.is_empty() { for arg in jvm.split_whitespace() { cmd.arg(arg); } }
        for arg in &server.jvm_args { cmd.arg(arg); }
        cmd.arg("-jar").arg(&server.jar_path).arg("nogui");
        cmd.current_dir(&server.path);
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).stdin(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| format!("Failed to start: {}", e))?;
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        self.processes.lock().unwrap().insert(id.to_string(), child);

        {
            let mut servers = self.servers.lock().unwrap();
            if let Some(s) = servers.iter_mut().find(|s| s.id == id) {
                s.last_started_at = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
            }
        }
        self.save();
        self.append_log(id, "[Sea Lantern] Server starting...");

        let logs_ref = &self.logs as *const Mutex<HashMap<String, Vec<String>>>;
        let max_lines = settings.max_log_lines as usize;

        if let Some(out) = stdout {
            let lid = id.to_string(); let ptr = logs_ref as usize; let ml = max_lines;
            std::thread::spawn(move || {
                let m = unsafe { &*(ptr as *const Mutex<HashMap<String, Vec<String>>>) };
                for line in BufReader::new(out).lines() {
                    match line {
                        Ok(t) => { if let Ok(mut l) = m.lock() { if let Some(v) = l.get_mut(&lid) { v.push(t); if v.len() > ml { let d = v.len()-ml; v.drain(0..d); } } } }
                        Err(_) => break,
                    }
                }
            });
        }
        if let Some(err) = stderr {
            let lid = id.to_string(); let ptr = logs_ref as usize; let ml = max_lines;
            std::thread::spawn(move || {
                let m = unsafe { &*(ptr as *const Mutex<HashMap<String, Vec<String>>>) };
                for line in BufReader::new(err).lines() {
                    match line {
                        Ok(t) => { if let Ok(mut l) = m.lock() { if let Some(v) = l.get_mut(&lid) { v.push(format!("[STDERR] {}", t)); if v.len() > ml { let d = v.len()-ml; v.drain(0..d); } } } }
                        Err(_) => break,
                    }
                }
            });
        }
        Ok(())
    }

    pub fn stop_server(&self, id: &str) -> Result<(), String> {
        // Check if actually running first
        let is_running = {
            let mut procs = self.processes.lock().unwrap();
            if let Some(child) = procs.get_mut(id) {
                match child.try_wait() {
                    Ok(Some(_)) => { procs.remove(id); false }
                    Ok(None) => true,
                    Err(_) => { procs.remove(id); false }
                }
            } else { false }
        };

        if !is_running {
            self.append_log(id, "[Sea Lantern] Server is not running");
            return Ok(());
        }

        // Send stop command
        self.append_log(id, "[Sea Lantern] Sending stop command...");
        let _ = self.send_command(id, "stop");

        // Wait for graceful shutdown (up to 10 seconds)
        for _ in 0..20 {
            std::thread::sleep(std::time::Duration::from_millis(500));
            let mut procs = self.processes.lock().unwrap();
            if let Some(child) = procs.get_mut(id) {
                match child.try_wait() {
                    Ok(Some(_)) => {
                        procs.remove(id);
                        self.append_log(id, "[Sea Lantern] Server stopped gracefully");
                        return Ok(());
                    }
                    Ok(None) => {} // Still running
                    Err(_) => {
                        procs.remove(id);
                        return Ok(());
                    }
                }
            } else {
                self.append_log(id, "[Sea Lantern] Server stopped");
                return Ok(());
            }
        }

        // Force kill after timeout
        let mut procs = self.processes.lock().unwrap();
        if let Some(mut child) = procs.remove(id) {
            let _ = child.kill();
            let _ = child.wait();
            self.append_log(id, "[Sea Lantern] Server force-killed after timeout");
        }
        Ok(())
    }

    pub fn send_command(&self, id: &str, command: &str) -> Result<(), String> {
        let mut procs = self.processes.lock().unwrap();
        let child = procs.get_mut(id).ok_or_else(|| "Server is not running".to_string())?;
        if let Some(ref mut stdin) = child.stdin {
            writeln!(stdin, "{}", command).map_err(|e| format!("Failed: {}", e))?;
            stdin.flush().map_err(|e| format!("Failed: {}", e))?;
        }
        Ok(())
    }

    pub fn get_server_list(&self) -> Vec<ServerInstance> {
        self.servers.lock().unwrap().clone()
    }

    pub fn get_server_status(&self, id: &str) -> ServerStatusInfo {
        let mut procs = self.processes.lock().unwrap();
        let is_running = if let Some(child) = procs.get_mut(id) {
            match child.try_wait() {
                Ok(Some(_)) => { procs.remove(id); false }
                Ok(None) => true,
                Err(_) => { procs.remove(id); false }
            }
        } else { false };
        ServerStatusInfo {
            id: id.to_string(),
            status: if is_running { ServerStatus::Running } else { ServerStatus::Stopped },
            pid: None, uptime: None,
        }
    }

    pub fn delete_server(&self, id: &str) -> Result<(), String> {
        // Only stop if actually running
        {
            let procs = self.processes.lock().unwrap();
            if procs.contains_key(id) {
                drop(procs);
                let _ = self.stop_server(id);
            }
        }
        self.servers.lock().unwrap().retain(|s| s.id != id);
        self.logs.lock().unwrap().remove(id);
        self.save();
        Ok(())
    }

    pub fn get_logs(&self, id: &str, since: usize) -> Vec<String> {
        let logs = self.logs.lock().unwrap();
        if let Some(v) = logs.get(id) {
            if since < v.len() { v[since..].to_vec() } else { Vec::new() }
        } else { Vec::new() }
    }

    fn append_log(&self, id: &str, msg: &str) {
        if let Ok(mut logs) = self.logs.lock() {
            if let Some(v) = logs.get_mut(id) { v.push(msg.to_string()); }
        }
    }

    pub fn stop_all_servers(&self) {
        let ids: Vec<String> = self.processes.lock().unwrap().keys().cloned().collect();
        for id in ids { let _ = self.stop_server(&id); }
    }
}

fn get_data_dir() -> String {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() { return dir.to_string_lossy().to_string(); }
    }
    ".".to_string()
}
fn load_servers(dir: &str) -> Vec<ServerInstance> {
    let p = std::path::Path::new(dir).join(DATA_FILE);
    if !p.exists() { return Vec::new(); }
    std::fs::read_to_string(&p).ok().and_then(|c| serde_json::from_str(&c).ok()).unwrap_or_default()
}
fn save_servers(dir: &str, servers: &[ServerInstance]) {
    let p = std::path::Path::new(dir).join(DATA_FILE);
    if let Ok(j) = serde_json::to_string_pretty(servers) { let _ = std::fs::write(&p, j); }
}
