<script setup lang="ts">
interface Props {
  variant?: "primary" | "secondary" | "ghost" | "danger";
  size?: "sm" | "md" | "lg";
  disabled?: boolean;
  loading?: boolean;
}

withDefaults(defineProps<Props>(), {
  variant: "primary",
  size: "md",
  disabled: false,
  loading: false,
});
</script>

<template>
  <button
    class="sl-button"
    :class="[
      'sl-button--' + variant,
      'sl-button--' + size,
      { 'sl-button--disabled': disabled, 'sl-button--loading': loading }
    ]"
    :disabled="disabled || loading"
  >
    <svg
      v-if="loading"
      class="sl-button-spinner animate-spin"
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
    >
      <path d="M12 2v4m0 12v4m10-10h-4M6 12H2m15.07-5.07l-2.83 2.83M9.76 14.24l-2.83 2.83m11.14 0l-2.83-2.83M9.76 9.76L6.93 6.93" />
    </svg>
    <slot />
  </button>
</template>

<style scoped>
.sl-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--sl-space-xs);
  font-weight: 500;
  border-radius: var(--sl-radius-md);
  transition: all var(--sl-transition-fast);
  cursor: pointer;
  white-space: nowrap;
}

.sl-button--sm {
  padding: 6px 12px;
  font-size: 0.8125rem;
}

.sl-button--md {
  padding: 8px 20px;
  font-size: 0.875rem;
}

.sl-button--lg {
  padding: 12px 28px;
  font-size: 1rem;
}

.sl-button--primary {
  background: var(--sl-primary);
  color: var(--sl-text-inverse);
  box-shadow: 0 2px 8px rgba(14, 165, 233, 0.25);
}

.sl-button--primary:hover {
  background: var(--sl-primary-dark);
  box-shadow: 0 4px 12px rgba(14, 165, 233, 0.35);
  transform: translateY(-1px);
}

.sl-button--secondary {
  background: var(--sl-bg-secondary);
  color: var(--sl-text-primary);
  border: 1px solid var(--sl-border);
}

.sl-button--secondary:hover {
  background: var(--sl-bg-tertiary);
  border-color: var(--sl-primary-light);
}

.sl-button--ghost {
  background: transparent;
  color: var(--sl-text-secondary);
}

.sl-button--ghost:hover {
  background: var(--sl-primary-bg);
  color: var(--sl-primary);
}

.sl-button--danger {
  background: var(--sl-error);
  color: var(--sl-text-inverse);
}

.sl-button--danger:hover {
  opacity: 0.9;
  transform: translateY(-1px);
}

.sl-button--disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none !important;
}

.sl-button-spinner {
  flex-shrink: 0;
}
</style>
