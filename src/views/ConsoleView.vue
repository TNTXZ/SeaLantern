<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, computed, watch } from "vue";
import { useRoute } from "vue-router";
import SLButton from "../components/common/SLButton.vue";
import SLSelect from "../components/common/SLSelect.vue";
import { useServerStore } from "../stores/serverStore";
import { useConsoleStore } from "../stores/consoleStore";
import { serverApi } from "../api/server";
import { playerApi } from "../api/player";

const route = useRoute();
const serverStore = useServerStore();
const consoleStore = useConsoleStore();

const commandInput = ref("");
const logContainer = ref<HTMLElement | null>(null);
const userScrolledUp = ref(false);
const showSuggestions = ref(false);
const suggestionIndex = ref(0);
const commandHistory = ref<string[]>([]);
const historyIndex = ref(-1);
let pollTimer: ReturnType<typeof setInterval> | null = null;

const allCommands = [
  "help", "list", "stop", "say",
  "time set day", "time set night", "time set noon",
  "weather clear", "weather rain", "weather thunder",
  "gamemode survival", "gamemode creative", "gamemode adventure", "gamemode spectator",
  "difficulty peaceful", "difficulty easy", "difficulty normal", "difficulty hard",
  "give", "tp", "teleport", "kill", "kick", "ban", "pardon",
  "op", "deop", "whitelist add", "whitelist remove", "whitelist list",
  "gamerule keepInventory true", "gamerule keepInventory false",
  "gamerule doDaylightCycle true", "gamerule doDaylightCycle false",
  "gamerule mobGriefing true", "gamerule mobGriefing false",
  "save-all", "tps", "plugins", "version",
];

const quickCommands = [
  { label: "白天", cmd: "time set day" },
  { label: "夜晚", cmd: "time set night" },
  { label: "晴天", cmd: "weather clear" },
  { label: "下雨", cmd: "weather rain" },
  { label: "保存", cmd: "save-all" },
  { label: "玩家列表", cmd: "list" },
  { label: "TPS", cmd: "tps" },
  { label: "保留物品 开", cmd: "gamerule keepInventory true" },
  { label: "保留物品 关", cmd: "gamerule keepInventory false" },
  { label: "怪物破坏 关", cmd: "gamerule mobGriefing false" },
];

const filteredSuggestions = computed(() => {
  const input = commandInput.value.trim().toLowerCase();
  if (!input) return [];
  return allCommands.filter((c) => c.toLowerCase().startsWith(input) && c.toLowerCase() !== input).slice(0, 8);
});

const serverId = computed(() =>
  (route.params.id as string) || consoleStore.activeServerId || serverStore.currentServerId || ""
);

const currentLogs = computed(() => consoleStore.logs[serverId.value] || []);

const serverOptions = computed(() =>
  serverStore.servers.map((s) => ({ label: s.name + " (" + s.id.substring(0, 8) + ")", value: s.id }))
);

watch(() => currentLogs.value.length, () => {
  if (!userScrolledUp.value) doScroll();
});

function switchServer(id: string | number) {
  consoleStore.setActiveServer(String(id));
  serverStore.setCurrentServer(String(id));
  userScrolledUp.value = false;
  nextTick(() => doScroll());
}

onMounted(async () => {
  await serverStore.refreshList();
  if (serverId.value) {
    consoleStore.setActiveServer(serverId.value);
    serverStore.setCurrentServer(serverId.value);
    await serverStore.refreshStatus(serverId.value);
  }
  startPolling();
  nextTick(() => doScroll());
});

onUnmounted(() => { stopPolling(); });

function startPolling() {
  stopPolling();
  pollTimer = setInterval(async () => {
    const sid = serverId.value;
    if (!sid) return;
    const cursor = consoleStore.getLogCursor(sid);
    try {
      const newLines = await serverApi.getLogs(sid, cursor);
      if (newLines.length > 0) {
        consoleStore.appendLogs(sid, newLines);
        consoleStore.setLogCursor(sid, cursor + newLines.length);
      }
    } catch (_e) {}
    await serverStore.refreshStatus(sid);
  }, 800);
}

function stopPolling() { if (pollTimer) { clearInterval(pollTimer); pollTimer = null; } }

async function sendCommand(cmd?: string) {
  const command = (cmd || commandInput.value).trim();
  const sid = serverId.value;
  if (!command || !sid) return;
  consoleStore.appendLocal(sid, "> " + command);
  commandHistory.value.push(command);
  historyIndex.value = -1;
  try { await serverApi.sendCommand(sid, command); }
  catch (e) { consoleStore.appendLocal(sid, "[ERROR] " + String(e)); }
  commandInput.value = "";
  showSuggestions.value = false;
  userScrolledUp.value = false;
  doScroll();
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === "Enter") {
    if (showSuggestions.value && filteredSuggestions.value.length > 0) {
      commandInput.value = filteredSuggestions.value[suggestionIndex.value];
      showSuggestions.value = false;
    } else { sendCommand(); }
    return;
  }
  if (e.key === "Tab") {
    e.preventDefault();
    if (filteredSuggestions.value.length > 0) {
      commandInput.value = filteredSuggestions.value[suggestionIndex.value];
      showSuggestions.value = false;
    }
    return;
  }
  if (e.key === "ArrowUp") {
    e.preventDefault();
    if (showSuggestions.value && suggestionIndex.value > 0) suggestionIndex.value--;
    else if (commandHistory.value.length > 0 && historyIndex.value < commandHistory.value.length - 1) {
      historyIndex.value++;
      commandInput.value = commandHistory.value[commandHistory.value.length - 1 - historyIndex.value];
    }
    return;
  }
  if (e.key === "ArrowDown") {
    e.preventDefault();
    if (showSuggestions.value && suggestionIndex.value < filteredSuggestions.value.length - 1) suggestionIndex.value++;
    else if (historyIndex.value > 0) { historyIndex.value--; commandInput.value = commandHistory.value[commandHistory.value.length - 1 - historyIndex.value]; }
    else { historyIndex.value = -1; commandInput.value = ""; }
    return;
  }
  if (e.key === "Escape") { showSuggestions.value = false; return; }
  nextTick(() => {
    showSuggestions.value = commandInput.value.trim().length > 0 && filteredSuggestions.value.length > 0;
    suggestionIndex.value = 0;
  });
}

function doScroll() {
  nextTick(() => { if (logContainer.value) logContainer.value.scrollTop = logContainer.value.scrollHeight; });
}

function handleScroll() {
  if (!logContainer.value) return;
  const el = logContainer.value;
  userScrolledUp.value = el.scrollHeight - el.scrollTop - el.clientHeight > 40;
}

async function handleStart() {
  const sid = serverId.value; if (!sid) return;
  try { await serverApi.start(sid); await serverStore.refreshStatus(sid); }
  catch (e) { consoleStore.appendLocal(sid, "[ERROR] " + String(e)); }
}

async function handleStop() {
  const sid = serverId.value; if (!sid) return;
  try { await serverApi.stop(sid); await serverStore.refreshStatus(sid); }
  catch (e) { consoleStore.appendLocal(sid, "[ERROR] " + String(e)); }
}

async function exportLogs() {
  const logs = currentLogs.value;
  if (logs.length === 0) return;
  // Copy to clipboard as fallback
  const text = logs.join("\n");
  try {
    await navigator.clipboard.writeText(text);
    consoleStore.appendLocal(serverId.value, "[Sea Lantern] Logs copied to clipboard (" + logs.length + " lines)");
  } catch (_e) {
    consoleStore.appendLocal(serverId.value, "[Sea Lantern] Failed to copy logs");
  }
}

function getStatusClass(): string {
  const s = serverStore.statuses[serverId.value]?.status;
  return s === "Running" ? "running" : s === "Starting" ? "starting" : s === "Stopping" ? "stopping" : "stopped";
}

function getStatusText(): string {
  const s = serverStore.statuses[serverId.value]?.status;
  return s === "Running" ? "Running" : s === "Starting" ? "Starting" : s === "Stopping" ? "Stopping" : "Stopped";
}
</script>

<template>
  <div class="console-view animate-fade-in-up">
    <div class="console-toolbar">
      <div class="toolbar-left">
        <div v-if="serverOptions.length > 1" class="server-selector">
          <SLSelect :options="serverOptions" :modelValue="serverId" placeholder="选择服务器" @update:modelValue="switchServer" />
        </div>
        <div v-else-if="serverId" class="server-name-display">
          {{ serverStore.servers.find((s) => s.id === serverId)?.name || "Server" }}
        </div>
        <div class="status-indicator" :class="getStatusClass()">
          <span class="status-dot"></span>
          <span class="status-label">{{ getStatusText() }}</span>
        </div>
      </div>
      <div class="toolbar-right">
        <SLButton variant="primary" size="sm" @click="handleStart">启动</SLButton>
        <SLButton variant="danger" size="sm" @click="handleStop">停止</SLButton>
        <SLButton variant="secondary" size="sm" @click="exportLogs">复制日志</SLButton>
        <SLButton variant="ghost" size="sm" @click="serverId && consoleStore.clearLogs(serverId)">清屏</SLButton>
      </div>
    </div>

    <div v-if="!serverId" class="no-server"><p class="text-body">请先创建并选择一个服务器</p></div>

    <template v-else>
      <div class="quick-commands">
        <span class="quick-label">快捷:</span>
        <div class="quick-groups">
          <div v-for="cmd in quickCommands" :key="cmd.cmd" class="quick-btn" @click="sendCommand(cmd.cmd)" :title="cmd.cmd">{{ cmd.label }}</div>
        </div>
      </div>

      <div class="console-output" ref="logContainer" @scroll="handleScroll">
        <div v-for="(line, i) in currentLogs" :key="i" class="log-line" :class="{
          'log-error': line.includes('[ERROR]') || line.includes('ERROR') || line.includes('[STDERR]'),
          'log-warn': line.includes('[WARN]') || line.includes('WARNING'),
          'log-command': line.startsWith('>'),
          'log-system': line.startsWith('[Sea Lantern]'),
        }">{{ line }}</div>
        <div v-if="currentLogs.length === 0" class="log-empty">等待输出...</div>
      </div>

      <div v-if="userScrolledUp" class="scroll-btn" @click="userScrolledUp = false; doScroll()">回到底部</div>

      <div class="console-input-wrapper">
        <div v-if="showSuggestions && filteredSuggestions.length > 0" class="suggestions-popup">
          <div v-for="(sug, i) in filteredSuggestions" :key="sug" class="suggestion-item" :class="{ active: i === suggestionIndex }" @mousedown.prevent="commandInput = sug; showSuggestions = false">{{ sug }}</div>
          <div class="suggestion-hint">Tab 补全 / Up Down 选择</div>
        </div>
        <div class="console-input-bar">
          <span class="input-prefix">&gt;</span>
          <input class="console-input" v-model="commandInput" placeholder="输入命令... (Tab 补全)" @keydown="handleKeydown" />
          <SLButton variant="primary" size="sm" @click="sendCommand()">发送</SLButton>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.console-view { display: flex; flex-direction: column; height: calc(100vh - var(--sl-header-height) - var(--sl-space-lg) * 2); gap: var(--sl-space-sm); position: relative; }
.console-toolbar { display: flex; align-items: center; justify-content: space-between; padding: var(--sl-space-sm) var(--sl-space-md); background: var(--sl-surface); border: 1px solid var(--sl-border-light); border-radius: var(--sl-radius-md); flex-shrink: 0; }
.toolbar-left { display: flex; align-items: center; gap: var(--sl-space-md); }
.toolbar-right { display: flex; gap: var(--sl-space-xs); }
.server-selector { min-width: 240px; }
.server-name-display { font-weight: 600; }
.status-indicator { display: flex; align-items: center; gap: var(--sl-space-xs); padding: 2px 10px; border-radius: var(--sl-radius-full); font-size: 0.8125rem; font-weight: 500; }
.status-dot { width: 8px; height: 8px; border-radius: 50%; }
.status-indicator.running { background: rgba(34,197,94,0.1); color: var(--sl-success); }
.status-indicator.running .status-dot { background: var(--sl-success); }
.status-indicator.stopped { background: var(--sl-bg-tertiary); color: var(--sl-text-tertiary); }
.status-indicator.stopped .status-dot { background: var(--sl-text-tertiary); }
.status-indicator.starting,.status-indicator.stopping { background: rgba(245,158,11,0.1); color: var(--sl-warning); }
.status-indicator.starting .status-dot,.status-indicator.stopping .status-dot { background: var(--sl-warning); }
.no-server { flex: 1; display: flex; align-items: center; justify-content: center; }
.quick-commands { display: flex; align-items: center; gap: var(--sl-space-sm); padding: var(--sl-space-xs) var(--sl-space-sm); background: var(--sl-surface); border: 1px solid var(--sl-border-light); border-radius: var(--sl-radius-md); flex-shrink: 0; overflow-x: auto; }
.quick-label { font-size: 0.75rem; color: var(--sl-text-tertiary); white-space: nowrap; }
.quick-groups { display: flex; gap: 4px; }
.quick-btn { padding: 3px 10px; border-radius: var(--sl-radius-sm); font-size: 0.75rem; cursor: pointer; border: 1px solid var(--sl-border); color: var(--sl-text-secondary); background: var(--sl-bg-secondary); white-space: nowrap; transition: all var(--sl-transition-fast); }
.quick-btn:hover { border-color: var(--sl-primary); color: var(--sl-primary); background: var(--sl-primary-bg); }
.console-output { flex: 1; background: #1e1e2e; border-radius: var(--sl-radius-md); padding: var(--sl-space-md); overflow-y: auto; font-family: var(--sl-font-mono); font-size: 0.8125rem; line-height: 1.7; color: #cdd6f4; min-height: 0; user-select: text; -webkit-user-select: text; cursor: text; }
.log-line { white-space: pre-wrap; word-break: break-all; }
.log-error { color: #f38ba8; }
.log-warn { color: #fab387; }
.log-command { color: #89b4fa; font-weight: 600; }
.log-system { color: #a6e3a1; font-style: italic; }
.log-empty { color: #585b70; font-style: italic; }
.scroll-btn { position: absolute; bottom: 70px; left: 50%; transform: translateX(-50%); padding: 6px 16px; background: var(--sl-primary); color: white; border-radius: var(--sl-radius-full); font-size: 0.75rem; cursor: pointer; box-shadow: var(--sl-shadow-md); z-index: 10; }
.console-input-wrapper { position: relative; flex-shrink: 0; }
.suggestions-popup { position: absolute; bottom: 100%; left: 0; right: 0; background: #313244; border: 1px solid #45475a; border-radius: var(--sl-radius-md); margin-bottom: 4px; max-height: 200px; overflow-y: auto; z-index: 20; }
.suggestion-item { padding: 6px 14px; font-family: var(--sl-font-mono); font-size: 0.8125rem; color: #cdd6f4; cursor: pointer; }
.suggestion-item:hover,.suggestion-item.active { background: #45475a; color: #89b4fa; }
.suggestion-hint { padding: 4px 14px; font-size: 0.6875rem; color: #585b70; border-top: 1px solid #45475a; }
.console-input-bar { display: flex; align-items: center; gap: var(--sl-space-sm); padding: var(--sl-space-sm) var(--sl-space-md); background: #1e1e2e; border-radius: var(--sl-radius-md); }
.input-prefix { color: #89b4fa; font-family: var(--sl-font-mono); font-weight: 700; }
.console-input { flex: 1; background: transparent; color: #cdd6f4; font-family: var(--sl-font-mono); font-size: 0.8125rem; padding: 6px 0; }
.console-input::placeholder { color: #585b70; }
</style>
