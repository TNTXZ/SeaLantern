import { tauriInvoke } from "./tauri";

export const systemApi = {
  async getSystemInfo() {
    return tauriInvoke("get_system_info");
  },

  async pickJarFile(): Promise<string | null> {
    return tauriInvoke("pick_jar_file");
  },

  async pickJavaFile(): Promise<string | null> {
    return tauriInvoke("pick_java_file");
  },
};
