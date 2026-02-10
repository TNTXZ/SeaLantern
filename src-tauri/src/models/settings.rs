use serde::{Deserialize, Serialize};
use crate::services::java_detector::JavaInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(default = "default_true")]
    pub close_servers_on_exit: bool,

    #[serde(default = "default_true")]
    pub auto_accept_eula: bool,

    #[serde(default = "default_max_memory")]
    pub default_max_memory: u32,

    #[serde(default = "default_min_memory")]
    pub default_min_memory: u32,

    #[serde(default = "default_port")]
    pub default_port: u16,

    #[serde(default)]
    pub default_java_path: String,

    #[serde(default)]
    pub default_jvm_args: String,

    #[serde(default = "default_console_font")]
    pub console_font_size: u32,

    #[serde(default = "default_log_lines")]
    pub max_log_lines: u32,

    #[serde(default)]
    pub cached_java_list: Vec<JavaInfo>,
}

fn default_true() -> bool { true }
fn default_max_memory() -> u32 { 2048 }
fn default_min_memory() -> u32 { 512 }
fn default_port() -> u16 { 25565 }
fn default_console_font() -> u32 { 13 }
fn default_log_lines() -> u32 { 5000 }

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            close_servers_on_exit: true,
            auto_accept_eula: true,
            default_max_memory: 2048,
            default_min_memory: 512,
            default_port: 25565,
            default_java_path: String::new(),
            default_jvm_args: String::new(),
            console_font_size: 13,
            max_log_lines: 5000,
            cached_java_list: Vec::new(),
        }
    }
}
