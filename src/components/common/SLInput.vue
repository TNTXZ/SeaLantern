<script setup lang="ts">
interface Props {
  modelValue?: string;
  placeholder?: string;
  label?: string;
  type?: string;
  disabled?: boolean;
}

withDefaults(defineProps<Props>(), {
  modelValue: "",
  placeholder: "",
  type: "text",
  disabled: false,
});

defineEmits<{
  "update:modelValue": [value: string];
}>();
</script>

<template>
  <div class="sl-input-wrapper">
    <label v-if="label" class="sl-input-label">{{ label }}</label>
    <div class="sl-input-container">
      <div v-if="$slots.prefix" class="sl-input-prefix">
        <slot name="prefix" />
      </div>
      <input
        class="sl-input"
        :type="type"
        :value="modelValue"
        :placeholder="placeholder"
        :disabled="disabled"
        @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      />
      <div v-if="$slots.suffix" class="sl-input-suffix">
        <slot name="suffix" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.sl-input-wrapper {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-xs);
}

.sl-input-label {
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--sl-text-secondary);
}

.sl-input-container {
  display: flex;
  align-items: center;
  background: var(--sl-surface);
  border: 1px solid var(--sl-border);
  border-radius: var(--sl-radius-md);
  transition: all var(--sl-transition-fast);
  overflow: hidden;
}

.sl-input-container:focus-within {
  border-color: var(--sl-primary);
  box-shadow: 0 0 0 3px var(--sl-primary-bg);
}

.sl-input {
  flex: 1;
  padding: 8px 12px;
  font-size: 0.875rem;
  background: transparent;
  min-width: 0;
}

.sl-input::placeholder {
  color: var(--sl-text-tertiary);
}

.sl-input-prefix,
.sl-input-suffix {
  display: flex;
  align-items: center;
  padding: 0 var(--sl-space-sm);
  color: var(--sl-text-tertiary);
}
</style>
