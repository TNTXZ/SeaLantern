import { tauriInvoke } from "./tauri";

export interface PlayerEntry {
  uuid: string;
  name: string;
}

export interface BanEntry {
  uuid: string;
  name: string;
  reason: string;
  source: string;
  created: string;
  expires: string;
}

export interface OpEntry {
  uuid: string;
  name: string;
  level: number;
  bypasses_player_limit: boolean;
}

export const playerApi = {
  // Read (from files, works anytime)
  async getWhitelist(serverPath: string): Promise<PlayerEntry[]> {
    return tauriInvoke("get_whitelist", { serverPath });
  },
  async getBannedPlayers(serverPath: string): Promise<BanEntry[]> {
    return tauriInvoke("get_banned_players", { serverPath });
  },
  async getOps(serverPath: string): Promise<OpEntry[]> {
    return tauriInvoke("get_ops", { serverPath });
  },

  // Modify (sends commands to running server)
  async addToWhitelist(serverId: string, name: string): Promise<string> {
    return tauriInvoke("add_to_whitelist", { serverId, name });
  },
  async removeFromWhitelist(serverId: string, name: string): Promise<string> {
    return tauriInvoke("remove_from_whitelist", { serverId, name });
  },
  async banPlayer(serverId: string, name: string, reason: string = ""): Promise<string> {
    return tauriInvoke("ban_player", { serverId, name, reason });
  },
  async unbanPlayer(serverId: string, name: string): Promise<string> {
    return tauriInvoke("unban_player", { serverId, name });
  },
  async addOp(serverId: string, name: string): Promise<string> {
    return tauriInvoke("add_op", { serverId, name });
  },
  async removeOp(serverId: string, name: string): Promise<string> {
    return tauriInvoke("remove_op", { serverId, name });
  },
  async kickPlayer(serverId: string, name: string, reason: string = ""): Promise<string> {
    return tauriInvoke("kick_player", { serverId, name, reason });
  },

  async exportLogs(logs: string[], savePath: string): Promise<void> {
    return tauriInvoke("export_logs", { logs, savePath });
  },
};
