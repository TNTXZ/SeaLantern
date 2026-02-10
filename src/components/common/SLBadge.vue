<script setup lang="ts">
interface Props {
  text: string;
  variant?: "primary" | "success" | "warning" | "error" | "neutral";
}

withDefaults(defineProps<Props>(), {
  variant: "primary",
});
</script>

<template>
  <span class="sl-badge" :class="'sl-badge--' + variant">
    {{ text }}
  </span>
</template>

<style scoped>
.sl-badge {
  display: inline-flex;
  align-items: center;
  padding: 2px 8px;
  font-size: 0.75rem;
  font-weight: 500;
  border-radius: var(--sl-radius-full);
}

.sl-badge--primary {
  background: var(--sl-primary-bg);
  color: var(--sl-primary);
}

.sl-badge--success {
  background: rgba(34, 197, 94, 0.1);
  color: var(--sl-success);
}

.sl-badge--warning {
  background: rgba(245, 158, 11, 0.1);
  color: var(--sl-warning);
}

.sl-badge--error {
  background: rgba(239, 68, 68, 0.1);
  color: var(--sl-error);
}

.sl-badge--neutral {
  background: var(--sl-bg-tertiary);
  color: var(--sl-text-secondary);
}
</style>
