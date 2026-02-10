<script setup lang="ts">
interface Props {
  title?: string;
  subtitle?: string;
  hoverable?: boolean;
  padding?: "none" | "sm" | "md" | "lg";
}

withDefaults(defineProps<Props>(), {
  hoverable: false,
  padding: "md",
});
</script>

<template>
  <div
    class="sl-card glass-card"
    :class="[
      'sl-card--pad-' + padding,
      { 'sl-card--hoverable': hoverable }
    ]"
  >
    <div v-if="title || $slots.header" class="sl-card-header">
      <div v-if="title" class="sl-card-header-text">
        <h3 class="sl-card-title">{{ title }}</h3>
        <p v-if="subtitle" class="sl-card-subtitle">{{ subtitle }}</p>
      </div>
      <slot name="header" />
      <div v-if="$slots.actions" class="sl-card-actions">
        <slot name="actions" />
      </div>
    </div>
    <div class="sl-card-body">
      <slot />
    </div>
    <div v-if="$slots.footer" class="sl-card-footer">
      <slot name="footer" />
    </div>
  </div>
</template>

<style scoped>
.sl-card {
  display: flex;
  flex-direction: column;
}

.sl-card--pad-none { padding: 0; }
.sl-card--pad-sm { padding: var(--sl-space-sm); }
.sl-card--pad-md { padding: var(--sl-space-md); }
.sl-card--pad-lg { padding: var(--sl-space-lg); }

.sl-card--hoverable {
  cursor: pointer;
}

.sl-card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--sl-space-md);
}

.sl-card-title {
  font-size: 1rem;
  font-weight: 600;
  color: var(--sl-text-primary);
}

.sl-card-subtitle {
  font-size: 0.8125rem;
  color: var(--sl-text-tertiary);
  margin-top: 2px;
}

.sl-card-footer {
  margin-top: var(--sl-space-md);
  padding-top: var(--sl-space-md);
  border-top: 1px solid var(--sl-border-light);
}
</style>
