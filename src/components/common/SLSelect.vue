<script setup lang="ts">
interface Option {
  label: string;
  value: string | number;
}

interface Props {
  modelValue?: string | number;
  options: Option[];
  label?: string;
  placeholder?: string;
  disabled?: boolean;
}

withDefaults(defineProps<Props>(), {
  placeholder: "请选择",
  disabled: false,
});

defineEmits<{
  "update:modelValue": [value: string | number];
}>();
</script>

<template>
  <div class="sl-select-wrapper">
    <label v-if="label" class="sl-select-label">{{ label }}</label>
    <div class="sl-select-container">
      <select
        class="sl-select"
        :value="modelValue"
        :disabled="disabled"
        @change="$emit('update:modelValue', ($event.target as HTMLSelectElement).value)"
      >
        <option value="" disabled>{{ placeholder }}</option>
        <option
          v-for="opt in options"
          :key="opt.value"
          :value="opt.value"
        >
          {{ opt.label }}
        </option>
      </select>
      <svg class="sl-select-arrow" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
        <path d="M6 9l6 6 6-6" />
      </svg>
    </div>
  </div>
</template>

<style scoped>
.sl-select-wrapper {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-xs);
}

.sl-select-label {
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--sl-text-secondary);
}

.sl-select-container {
  position: relative;
}

.sl-select {
  width: 100%;
  padding: 8px 36px 8px 12px;
  font-size: 0.875rem;
  background: var(--sl-surface);
  border: 1px solid var(--sl-border);
  border-radius: var(--sl-radius-md);
  appearance: none;
  cursor: pointer;
  transition: all var(--sl-transition-fast);
  color: var(--sl-text-primary);
}

.sl-select:focus {
  border-color: var(--sl-primary);
  box-shadow: 0 0 0 3px var(--sl-primary-bg);
  outline: none;
}

.sl-select-arrow {
  position: absolute;
  right: 10px;
  top: 50%;
  transform: translateY(-50%);
  color: var(--sl-text-tertiary);
  pointer-events: none;
}
</style>
