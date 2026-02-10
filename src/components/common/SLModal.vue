<script setup lang="ts">
interface Props {
  visible: boolean;
  title?: string;
  width?: string;
}

withDefaults(defineProps<Props>(), {
  width: "480px",
});

defineEmits<{
  close: [];
}>();
</script>

<template>
  <Teleport to="body">
    <transition name="modal">
      <div v-if="visible" class="sl-modal-overlay" @click.self="$emit('close')">
        <div class="sl-modal glass-strong" :style="{ maxWidth: width }">
          <div class="sl-modal-header">
            <h3 v-if="title" class="sl-modal-title">{{ title }}</h3>
            <button class="sl-modal-close" @click="$emit('close')">
              <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                <path d="M18 6L6 18M6 6l12 12" />
              </svg>
            </button>
          </div>
          <div class="sl-modal-body">
            <slot />
          </div>
          <div v-if="$slots.footer" class="sl-modal-footer">
            <slot name="footer" />
          </div>
        </div>
      </div>
    </transition>
  </Teleport>
</template>

<style scoped>
.sl-modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.sl-modal {
  width: 90%;
  border-radius: var(--sl-radius-lg);
  box-shadow: var(--sl-shadow-xl);
}

.sl-modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--sl-space-md) var(--sl-space-lg);
  border-bottom: 1px solid var(--sl-border-light);
}

.sl-modal-title {
  font-size: 1.0625rem;
  font-weight: 600;
}

.sl-modal-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: var(--sl-radius-md);
  color: var(--sl-text-tertiary);
  transition: all var(--sl-transition-fast);
}

.sl-modal-close:hover {
  background: var(--sl-bg-tertiary);
  color: var(--sl-text-primary);
}

.sl-modal-body {
  padding: var(--sl-space-lg);
}

.sl-modal-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--sl-space-sm);
  padding: var(--sl-space-md) var(--sl-space-lg);
  border-top: 1px solid var(--sl-border-light);
}

.modal-enter-active,
.modal-leave-active {
  transition: all 0.25s ease;
}

.modal-enter-active .sl-modal,
.modal-leave-active .sl-modal {
  transition: all 0.25s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .sl-modal,
.modal-leave-to .sl-modal {
  transform: scale(0.95) translateY(10px);
  opacity: 0;
}
</style>
