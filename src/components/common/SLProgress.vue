<script setup lang="ts">
interface Props {
  value: number;
  max?: number;
  label?: string;
  showPercent?: boolean;
  variant?: "primary" | "success" | "warning" | "error";
}

withDefaults(defineProps<Props>(), {
  max: 100,
  showPercent: true,
  variant: "primary",
});
</script>

<template>
  <div class="sl-progress">
    <div v-if="label || showPercent" class="sl-progress-header">
      <span v-if="label" class="sl-progress-label">{{ label }}</span>
      <span v-if="showPercent" class="sl-progress-percent">
        {{ Math.round((value / max) * 100) }}%
      </span>
    </div>
    <div class="sl-progress-track">
      <div
        class="sl-progress-bar"
        :class="'sl-progress-bar--' + variant"
        :style="{ width: Math.min((value / max) * 100, 100) + '%' }"
      />
    </div>
  </div>
</template>

<style scoped>
.sl-progress {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.sl-progress-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.sl-progress-label {
  font-size: 0.8125rem;
  color: var(--sl-text-secondary);
}

.sl-progress-percent {
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--sl-text-primary);
  font-family: var(--sl-font-mono);
}

.sl-progress-track {
  height: 6px;
  background: var(--sl-bg-tertiary);
  border-radius: var(--sl-radius-full);
  overflow: hidden;
}

.sl-progress-bar {
  height: 100%;
  border-radius: var(--sl-radius-full);
  transition: width var(--sl-transition-slow);
}

.sl-progress-bar--primary { background: var(--sl-primary); }
.sl-progress-bar--success { background: var(--sl-success); }
.sl-progress-bar--warning { background: var(--sl-warning); }
.sl-progress-bar--error { background: var(--sl-error); }
</style>
