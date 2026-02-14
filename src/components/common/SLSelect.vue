<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from "vue";

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
  searchable?: boolean;
  loading?: boolean;
  maxHeight?: string;
  previewFont?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: "请选择",
  disabled: false,
  searchable: false,
  loading: false,
  maxHeight: "280px",
  previewFont: false,
});

function getFontStyle(value: string | number) {
  if (!props.previewFont || !value) return {};
  return { fontFamily: String(value) };
}

const emit = defineEmits<{
  "update:modelValue": [value: string | number];
}>();

const isOpen = ref(false);
const searchQuery = ref("");
const containerRef = ref<HTMLElement | null>(null);
const dropdownRef = ref<HTMLElement | null>(null);
const inputRef = ref<HTMLInputElement | null>(null);
const highlightedIndex = ref(-1);

const selectedOption = computed(() => {
  return props.options.find(opt => opt.value === props.modelValue);
});

const filteredOptions = computed(() => {
  if (!props.searchable || !searchQuery.value) return props.options;
  const query = searchQuery.value.toLowerCase();
  return props.options.filter(opt => 
    opt.label.toLowerCase().includes(query)
  );
});

function toggleDropdown() {
  if (props.disabled) return;
  isOpen.value = !isOpen.value;
  if (isOpen.value) {
    searchQuery.value = "";
    highlightedIndex.value = -1;
    if (props.searchable) {
      setTimeout(() => inputRef.value?.focus(), 0);
    }
  }
}

function selectOption(option: Option) {
  emit("update:modelValue", option.value);
  isOpen.value = false;
  searchQuery.value = "";
}

function handleKeydown(e: KeyboardEvent) {
  if (!isOpen.value) {
    if (e.key === "Enter" || e.key === " " || e.key === "ArrowDown") {
      e.preventDefault();
      toggleDropdown();
    }
    return;
  }

  switch (e.key) {
    case "ArrowDown":
      e.preventDefault();
      highlightedIndex.value = Math.min(
        highlightedIndex.value + 1,
        filteredOptions.value.length - 1
      );
      scrollToHighlighted();
      break;
    case "ArrowUp":
      e.preventDefault();
      highlightedIndex.value = Math.max(highlightedIndex.value - 1, 0);
      scrollToHighlighted();
      break;
    case "Enter":
      e.preventDefault();
      if (highlightedIndex.value >= 0 && filteredOptions.value[highlightedIndex.value]) {
        selectOption(filteredOptions.value[highlightedIndex.value]);
      }
      break;
    case "Escape":
      isOpen.value = false;
      break;
  }
}

function scrollToHighlighted() {
  setTimeout(() => {
    const highlighted = dropdownRef.value?.querySelector(".highlighted");
    highlighted?.scrollIntoView({ block: "nearest" });
  }, 0);
}

function handleClickOutside(e: MouseEvent) {
  if (containerRef.value && !containerRef.value.contains(e.target as Node)) {
    isOpen.value = false;
  }
}

watch(searchQuery, () => {
  highlightedIndex.value = filteredOptions.value.length > 0 ? 0 : -1;
});

onMounted(() => {
  document.addEventListener("click", handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener("click", handleClickOutside);
});
</script>

<template>
  <div class="sl-select" ref="containerRef">
    <label v-if="label" class="sl-select-label">{{ label }}</label>
    
    <div 
      class="sl-select-trigger"
      :class="{ open: isOpen, disabled }"
      @click="toggleDropdown"
      @keydown="handleKeydown"
      tabindex="0"
    >
      <span v-if="loading" class="sl-select-loading">
        <svg class="spinner" viewBox="0 0 24 24" width="16" height="16">
          <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4" />
        </svg>
        加载中...
      </span>
      <span v-else-if="selectedOption" class="sl-select-value" :style="getFontStyle(selectedOption.value)">
        {{ selectedOption.label }}
      </span>
      <span v-else class="sl-select-placeholder">{{ placeholder }}</span>
      
      <svg class="sl-select-arrow" :class="{ open: isOpen }" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
        <path d="M6 9l6 6 6-6" />
      </svg>
    </div>

    <Transition name="dropdown">
      <div v-if="isOpen" class="sl-select-dropdown" ref="dropdownRef">
        <div v-if="searchable" class="sl-select-search">
          <svg class="search-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="11" cy="11" r="8" />
            <path d="M21 21l-4.35-4.35" />
          </svg>
          <input
            ref="inputRef"
            v-model="searchQuery"
            type="text"
            placeholder="搜索..."
            class="sl-select-input"
            @keydown="handleKeydown"
          />
        </div>
        
        <div class="sl-select-options" :style="{ maxHeight }">
          <div v-if="filteredOptions.length === 0" class="sl-select-empty">
            未找到匹配项
          </div>
          <div
            v-for="(option, index) in filteredOptions"
            :key="option.value"
            class="sl-select-option"
            :class="{
              selected: option.value === modelValue,
              highlighted: index === highlightedIndex
            }"
            :style="getFontStyle(option.value)"
            @click="selectOption(option)"
            @mouseenter="highlightedIndex = index"
          >
            <span class="option-label">{{ option.label }}</span>
            <svg v-if="option.value === modelValue" class="check-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M20 6L9 17l-5-5" />
            </svg>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.sl-select {
  position: relative;
  width: 100%;
}

.sl-select-label {
  display: block;
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--sl-text-secondary);
  margin-bottom: var(--sl-space-xs);
}

.sl-select-trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 8px 12px;
  font-size: 0.875rem;
  background: var(--sl-surface);
  border: 1px solid var(--sl-border);
  border-radius: var(--sl-radius-md);
  cursor: pointer;
  transition: all var(--sl-transition-fast);
  color: var(--sl-text-primary);
  min-height: 38px;
}

.sl-select-trigger:hover:not(.disabled) {
  border-color: var(--sl-border-hover);
}

.sl-select-trigger:focus {
  border-color: var(--sl-primary);
  box-shadow: 0 0 0 3px var(--sl-primary-bg);
  outline: none;
}

.sl-select-trigger.open {
  border-color: var(--sl-primary);
  box-shadow: 0 0 0 3px var(--sl-primary-bg);
}

.sl-select-trigger.disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.sl-select-value {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.sl-select-placeholder {
  color: var(--sl-text-tertiary);
  flex: 1;
}

.sl-select-loading {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--sl-text-tertiary);
  flex: 1;
}

.sl-select-loading .spinner {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.sl-select-arrow {
  color: var(--sl-text-tertiary);
  transition: transform var(--sl-transition-fast);
  flex-shrink: 0;
  margin-left: 8px;
}

.sl-select-arrow.open {
  transform: rotate(180deg);
}

.sl-select-dropdown {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  right: 0;
  background: var(--sl-surface);
  border: 1px solid var(--sl-border);
  border-radius: var(--sl-radius-md);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
  z-index: 1000;
  overflow: hidden;
}

.sl-select-search {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-bottom: 1px solid var(--sl-border);
}

.sl-select-search .search-icon {
  color: var(--sl-text-tertiary);
  flex-shrink: 0;
}

.sl-select-input {
  flex: 1;
  border: none;
  background: transparent;
  font-size: 0.875rem;
  color: var(--sl-text-primary);
  outline: none;
}

.sl-select-input::placeholder {
  color: var(--sl-text-tertiary);
}

.sl-select-options {
  overflow-y: auto;
  overscroll-behavior: contain;
}

.sl-select-options::-webkit-scrollbar {
  width: 6px;
}

.sl-select-options::-webkit-scrollbar-track {
  background: transparent;
}

.sl-select-options::-webkit-scrollbar-thumb {
  background: var(--sl-border);
  border-radius: 3px;
}

.sl-select-options::-webkit-scrollbar-thumb:hover {
  background: var(--sl-text-tertiary);
}

.sl-select-empty {
  padding: 16px;
  text-align: center;
  color: var(--sl-text-tertiary);
  font-size: 0.875rem;
}

.sl-select-option {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 12px;
  cursor: pointer;
  transition: background var(--sl-transition-fast);
}

.sl-select-option:hover,
.sl-select-option.highlighted {
  background: var(--sl-surface-hover);
}

.sl-select-option.selected {
  color: var(--sl-primary);
  background: var(--sl-primary-bg);
}

.sl-select-option .option-label {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.sl-select-option .check-icon {
  color: var(--sl-primary);
  flex-shrink: 0;
  margin-left: 8px;
}

.dropdown-enter-active,
.dropdown-leave-active {
  transition: all 0.15s ease;
}

.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
