<script setup lang="ts">
interface Props {
  modelValue?: boolean;
  label?: string;
  disabled?: boolean;
}

withDefaults(defineProps<Props>(), {
  modelValue: false,
  disabled: false,
});

defineEmits<{
  "update:modelValue": [value: boolean];
}>();
</script>

<template>
  <label class="sl-switch-wrapper" :class="{ disabled: disabled }">
    <div
      class="sl-switch"
      :class="{ active: modelValue }"
      @click="!disabled && $emit('update:modelValue', !modelValue)"
    >
      <div class="sl-switch-thumb" />
    </div>
    <span v-if="label" class="sl-switch-label">{{ label }}</span>
  </label>
</template>

<style scoped>
.sl-switch-wrapper {
  display: inline-flex;
  align-items: center;
  gap: var(--sl-space-sm);
  cursor: pointer;
}

.sl-switch-wrapper.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.sl-switch {
  position: relative;
  width: 40px;
  height: 22px;
  background: var(--sl-border);
  border-radius: var(--sl-radius-full);
  transition: background var(--sl-transition-fast);
}

.sl-switch.active {
  background: var(--sl-primary);
}

.sl-switch-thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 18px;
  height: 18px;
  background: white;
  border-radius: 50%;
  box-shadow: var(--sl-shadow-sm);
  transition: transform var(--sl-transition-fast);
}

.sl-switch.active .sl-switch-thumb {
  transform: translateX(18px);
}

.sl-switch-label {
  font-size: 0.875rem;
  color: var(--sl-text-secondary);
}
</style>
