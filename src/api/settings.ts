import { tauriInvoke } from "./tauri";
import type { JavaInfo } from "./java";

export interface AppSettings {
  close_servers_on_exit: boolean;
  auto_accept_eula: boolean;
  default_max_memory: number;
  default_min_memory: number;
  default_port: number;
  default_java_path: string;
  default_jvm_args: string;
  console_font_size: number;
  max_log_lines: number;
  cached_java_list: JavaInfo[];
}

export const settingsApi = {
  async get(): Promise<AppSettings> {
    return tauriInvoke("get_settings");
  },
  async save(settings: AppSettings): Promise<void> {
    return tauriInvoke("save_settings", { settings });
  },
  async reset(): Promise<AppSettings> {
    return tauriInvoke("reset_settings");
  },
  async exportJson(): Promise<string> {
    return tauriInvoke("export_settings");
  },
  async importJson(json: string): Promise<AppSettings> {
    return tauriInvoke("import_settings", { json });
  },
};
