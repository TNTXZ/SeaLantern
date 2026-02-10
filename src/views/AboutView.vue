<script setup lang="ts">
import { ref } from "vue";
import SLCard from "../components/common/SLCard.vue";
import SLButton from "../components/common/SLButton.vue";

interface Contributor {
  name: string;
  role: string;
  avatar: string;
  url?: string;
}

const version = "0.1.0";
const buildDate = "2026";

const contributors = ref<Contributor[]>([
  {
    name: "FPS_Z",
    role: "创始人 / 主要开发者",
    avatar: "https://mc-heads.net/avatar/FPS_Z/64",
    url: "",
  },
  // ============================================
  // 在这里添加更多贡献者！
  // 格式：{ name: "ID", role: "角色", avatar: "头像URL", url: "主页链接" }
  //
  // 每一个参与贡献的人都会被展示在这里
  // 无论是代码、设计、建议还是文档，你的名字值得被记住
  // ============================================
]);

function openUrl(url: string) {
  if (url) window.open(url, "_blank");
}
</script>

<template>
  <div class="about-view animate-fade-in-up">
    <!-- Hero Section -->
    <div class="hero-section">
      <div class="hero-logo">
        <svg width="72" height="72" viewBox="0 0 72 72" fill="none">
          <rect x="4" y="4" width="64" height="64" rx="16" fill="var(--sl-primary)" opacity="0.1"/>
          <rect x="12" y="12" width="48" height="48" rx="12" fill="var(--sl-primary)" opacity="0.2"/>
          <rect x="22" y="22" width="28" height="28" rx="6" fill="var(--sl-primary)" opacity="0.4"/>
          <rect x="28" y="28" width="16" height="16" rx="3" fill="var(--sl-primary)"/>
        </svg>
      </div>
      <h1 class="hero-title">Sea Lantern</h1>
      <p class="hero-subtitle">Minecraft 服务器管理工具</p>
      <div class="hero-badges">
        <span class="version-badge">v{{ version }}</span>
        <span class="tech-badge">Tauri 2 + Vue 3</span>
        <span class="license-badge">GPLv3</span>
      </div>
      <p class="hero-desc">
        一个由社区共同打造的 Minecraft 开服器。<br/>
        不仅代码开源，连灵魂都由你们定义。
      </p>
    </div>

    <!-- Manifesto -->
    <SLCard>
      <div class="manifesto">
        <h3 class="manifesto-title">为什么叫 Sea Lantern？</h3>
        <p class="manifesto-text">
          海晶灯（Sea Lantern）是 Minecraft 中一种发光方块——它由无数碎片组合而成，却能发出柔和而持久的光。
        </p>
        <p class="manifesto-text">
          就像这个项目一样，每一位贡献者都是一片海晶碎片。<br/>
          当我们聚在一起，就能照亮整个社区。
        </p>
      </div>
    </SLCard>

    <!-- Contributor Wall -->
    <div class="contributor-section">
      <div class="section-header">
        <h2 class="section-title">贡献者墙</h2>
        <p class="section-desc">每一个让这个项目变得更好的人</p>
      </div>

      <div class="contributor-grid">
        <div
          v-for="c in contributors"
          :key="c.name"
          class="contributor-card glass-card"
          :class="{ clickable: !!c.url }"
          @click="c.url && openUrl(c.url)"
        >
          <img :src="c.avatar" :alt="c.name" class="contributor-avatar" />
          <div class="contributor-info">
            <span class="contributor-name">{{ c.name }}</span>
            <span class="contributor-role">{{ c.role }}</span>
          </div>
        </div>

        <!-- Join Card -->
        <div class="contributor-card glass-card join-card">
          <div class="join-icon">
            <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="var(--sl-primary)" stroke-width="1.5" stroke-linecap="round">
              <path d="M12 4v16m8-8H4" />
            </svg>
          </div>
          <div class="contributor-info">
            <span class="contributor-name join-text">你的名字</span>
            <span class="contributor-role">参与贡献，加入我们</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Project Info -->
    <div class="info-grid">
      <SLCard title="项目信息">
        <div class="info-list">
          <div class="info-item">
            <span class="info-label">版本</span>
            <span class="info-value">{{ version }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">构建年份</span>
            <span class="info-value">{{ buildDate }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">前端</span>
            <span class="info-value">Vue 3 + TypeScript + Vite</span>
          </div>
          <div class="info-item">
            <span class="info-label">后端</span>
            <span class="info-value">Rust + Tauri 2</span>
          </div>
          <div class="info-item">
            <span class="info-label">许可证</span>
            <span class="info-value">GNU GPLv3</span>
          </div>
        </div>
      </SLCard>

      <SLCard title="参与方式">
        <div class="contribute-ways">
          <div class="way-item">
            <div class="way-icon">💻</div>
            <div class="way-info">
              <span class="way-title">写代码</span>
              <span class="way-desc">提交 PR，修 Bug 或加新功能</span>
            </div>
          </div>
          <div class="way-item">
            <div class="way-icon">🎨</div>
            <div class="way-info">
              <span class="way-title">做设计</span>
              <span class="way-desc">设计 UI、图标、主题皮肤</span>
            </div>
          </div>
          <div class="way-item">
            <div class="way-icon">💡</div>
            <div class="way-info">
              <span class="way-title">提建议</span>
              <span class="way-desc">在 Issues 里提出你的想法</span>
            </div>
          </div>
          <div class="way-item">
            <div class="way-icon">📖</div>
            <div class="way-info">
              <span class="way-title">写文档</span>
              <span class="way-desc">完善教程和使用说明</span>
            </div>
          </div>
          <div class="way-item">
            <div class="way-icon">🌍</div>
            <div class="way-info">
              <span class="way-title">翻译</span>
              <span class="way-desc">帮助翻译成其他语言</span>
            </div>
          </div>
          <div class="way-item">
            <div class="way-icon">📢</div>
            <div class="way-info">
              <span class="way-title">推广</span>
              <span class="way-desc">分享给更多 MC 服主</span>
            </div>
          </div>
        </div>
      </SLCard>
    </div>

    <!-- Links -->
    <div class="links-section">
      <SLButton variant="primary" size="lg" @click="openUrl('https://gitee.com/')">
        Gitee 仓库
      </SLButton>
      <SLButton variant="secondary" size="lg" @click="openUrl('https://space.bilibili.com/409620362?spm_id_from=333.1007.0.0')">
        B站主页
      </SLButton>
      <SLButton variant="ghost" size="lg" @click="openUrl('')">
        加入QQ群
      </SLButton>
    </div>

    <!-- Footer -->
    <div class="about-footer">
      <p class="footer-text">
        Sea Lantern 是一个开源项目，遵循 GPLv3 协议。
      </p>
      <p class="footer-text">
        Minecraft 是 Mojang Studios 的注册商标。本项目与 Mojang 无关。
      </p>
      <p class="footer-quote">
        "我们搭建了骨架，而灵魂，交给你们。"
      </p>
    </div>
  </div>
</template>

<style scoped>
.about-view {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-xl);
  max-width: 900px;
  margin: 0 auto;
  padding-bottom: var(--sl-space-2xl);
}

/* Hero */
.hero-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: var(--sl-space-2xl) 0;
}

.hero-logo {
  margin-bottom: var(--sl-space-md);
  animation: sl-fade-in-up 0.6s ease forwards;
}

.hero-title {
  font-size: 2.5rem;
  font-weight: 800;
  color: var(--sl-text-primary);
  letter-spacing: -0.03em;
  margin-bottom: var(--sl-space-xs);
  animation: sl-fade-in-up 0.6s ease 0.1s both;
}

.hero-subtitle {
  font-size: 1.125rem;
  color: var(--sl-text-secondary);
  margin-bottom: var(--sl-space-md);
  animation: sl-fade-in-up 0.6s ease 0.2s both;
}

.hero-badges {
  display: flex;
  gap: var(--sl-space-sm);
  margin-bottom: var(--sl-space-lg);
  animation: sl-fade-in-up 0.6s ease 0.3s both;
}

.version-badge, .tech-badge, .license-badge {
  padding: 4px 14px;
  border-radius: var(--sl-radius-full);
  font-size: 0.8125rem;
  font-weight: 500;
}

.version-badge {
  background: var(--sl-primary-bg);
  color: var(--sl-primary);
}

.tech-badge {
  background: rgba(34, 197, 94, 0.1);
  color: var(--sl-success);
}

.license-badge {
  background: rgba(168, 85, 247, 0.1);
  color: #a855f7;
}

.hero-desc {
  font-size: 1rem;
  color: var(--sl-text-secondary);
  line-height: 1.8;
  animation: sl-fade-in-up 0.6s ease 0.4s both;
}

/* Manifesto */
.manifesto {
  text-align: center;
  padding: var(--sl-space-lg);
}

.manifesto-title {
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--sl-text-primary);
  margin-bottom: var(--sl-space-md);
}

.manifesto-text {
  font-size: 0.9375rem;
  color: var(--sl-text-secondary);
  line-height: 1.8;
  margin-bottom: var(--sl-space-sm);
}

/* Contributor Wall */
.contributor-section {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-md);
}

.section-header {
  text-align: center;
}

.section-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--sl-text-primary);
  margin-bottom: var(--sl-space-xs);
}

.section-desc {
  font-size: 0.9375rem;
  color: var(--sl-text-tertiary);
}

.contributor-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: var(--sl-space-md);
}

.contributor-card {
  display: flex;
  align-items: center;
  gap: var(--sl-space-md);
  padding: var(--sl-space-md);
  transition: all var(--sl-transition-normal);
}

.contributor-card.clickable {
  cursor: pointer;
}

.contributor-card.clickable:hover {
  transform: translateY(-4px);
  box-shadow: var(--sl-shadow-lg);
}

.contributor-avatar {
  width: 48px;
  height: 48px;
  border-radius: var(--sl-radius-md);
  flex-shrink: 0;
  background: var(--sl-bg-tertiary);
  image-rendering: pixelated;
}

.contributor-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.contributor-name {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--sl-text-primary);
}

.contributor-role {
  font-size: 0.75rem;
  color: var(--sl-text-tertiary);
}

.join-card {
  border: 2px dashed var(--sl-border);
  background: transparent;
  cursor: default;
}

.join-card:hover {
  border-color: var(--sl-primary-light);
  background: var(--sl-primary-bg);
  transform: none;
  box-shadow: none;
}

.join-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.join-text {
  color: var(--sl-primary);
}

/* Info Grid */
.info-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--sl-space-md);
}

.info-list {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 0;
  border-bottom: 1px solid var(--sl-border-light);
}

.info-item:last-child { border-bottom: none; }

.info-label {
  font-size: 0.875rem;
  color: var(--sl-text-tertiary);
}

.info-value {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--sl-text-primary);
  font-family: var(--sl-font-mono);
}

/* Contribute Ways */
.contribute-ways {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--sl-space-sm);
}

.way-item {
  display: flex;
  align-items: center;
  gap: var(--sl-space-sm);
  padding: var(--sl-space-sm);
  border-radius: var(--sl-radius-md);
  transition: background var(--sl-transition-fast);
}

.way-item:hover {
  background: var(--sl-bg-secondary);
}

.way-icon {
  font-size: 1.5rem;
  flex-shrink: 0;
  width: 36px;
  text-align: center;
}

.way-info {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.way-title {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--sl-text-primary);
}

.way-desc {
  font-size: 0.75rem;
  color: var(--sl-text-tertiary);
}

/* Links */
.links-section {
  display: flex;
  justify-content: center;
  gap: var(--sl-space-md);
}

/* Footer */
.about-footer {
  text-align: center;
  padding: var(--sl-space-xl) 0;
  border-top: 1px solid var(--sl-border-light);
}

.footer-text {
  font-size: 0.8125rem;
  color: var(--sl-text-tertiary);
  margin-bottom: var(--sl-space-xs);
}

.footer-quote {
  font-size: 1rem;
  font-weight: 500;
  color: var(--sl-primary);
  font-style: italic;
  margin-top: var(--sl-space-md);
}

@media (max-width: 768px) {
  .info-grid { grid-template-columns: 1fr; }
  .contribute-ways { grid-template-columns: 1fr; }
  .contributor-grid { grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); }
}
</style>
