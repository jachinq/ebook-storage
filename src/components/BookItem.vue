<script setup lang="ts">
import { computed, ref, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import type { BookInfo } from "../types";
import { formatFileSize } from "../utils/format";

const props = defineProps<{ book: BookInfo }>();
const emit = defineEmits<{ (e: "renamed"): void }>();

const editing = ref(false);
const editName = ref("");
const editInput = ref<HTMLInputElement | null>(null);

const fileExt = computed(() => {
  const dot = props.book.name.lastIndexOf(".");
  return dot !== -1 ? props.book.name.slice(dot + 1).toLowerCase() : "";
});

const displayName = computed(() => {
  const dot = props.book.name.lastIndexOf(".");
  return dot !== -1 ? props.book.name.slice(0, dot) : props.book.name;
});

async function openFile() {
  await invoke("open_file", { path: props.book.path });
}

async function revealFile() {
  await invoke("reveal_file", { path: props.book.path });
}

function startRename() {
  editName.value = displayName.value;
  editing.value = true;
  nextTick(() => editInput.value?.select());
}

async function confirmRename() {
  const trimmed = editName.value.trim();
  if (!trimmed || trimmed === displayName.value) {
    editing.value = false;
    return;
  }
  const newFileName = fileExt.value ? `${trimmed}.${fileExt.value}` : trimmed;
  try {
    await invoke("rename_file", { path: props.book.path, newName: newFileName });
    editing.value = false;
    emit("renamed");
  } catch (e) {
    alert(`改名失败: ${e}`);
  }
}

function cancelRename() {
  editing.value = false;
}

function searchDouban() {
  openUrl(`https://search.douban.com/book/subject_search?search_text=${encodeURIComponent(displayName.value)}`);
}
</script>

<template>
  <div class="book-item" @click="openFile">
    <span class="format-badge" :class="'fmt-' + fileExt">{{ fileExt.toUpperCase() }}</span>
    <div class="book-info">
      <div v-if="editing" class="book-name-edit" @click.stop>
        <input
          ref="editInput"
          v-model="editName"
          class="rename-input"
          @keydown.enter="confirmRename"
          @keydown.escape="cancelRename"
          @blur="cancelRename"
        />
      </div>
      <div v-else class="book-name">{{ displayName }}</div>
      <div class="book-meta">
        <span class="book-path">{{ book.path }}</span>
        <span class="book-size">{{ formatFileSize(book.size) }}</span>
      </div>
    </div>
    <div class="book-actions">
      <button class="btn-icon" @click.stop="startRename" title="改名">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <path d="M11.5 1.5L14.5 4.5L5 14H2V11L11.5 1.5Z" stroke="currentColor" stroke-width="1.2" fill="none"/>
        </svg>
      </button>
      <button class="btn-icon" @click.stop="searchDouban" title="豆瓣搜索">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <circle cx="7" cy="7" r="5" stroke="currentColor" stroke-width="1.2"/>
          <path d="M11 11L14 14" stroke="currentColor" stroke-width="1.2" stroke-linecap="round"/>
        </svg>
      </button>
      <button class="btn-icon" @click.stop="revealFile" title="打开所在目录">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <path d="M1 3.5C1 2.67 1.67 2 2.5 2H6l1.5 1.5H13.5C14.33 3.5 15 4.17 15 5V12.5C15 13.33 14.33 14 13.5 14H2.5C1.67 14 1 13.33 1 12.5V3.5Z" stroke="currentColor" stroke-width="1.2" fill="none"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.book-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 14px;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.15s;
}
.book-item:hover {
  background-color: var(--hover-bg);
}
.format-badge {
  flex-shrink: 0;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.5px;
  margin-right: 10px;
}
.fmt-pdf {
  background-color: #fde8e8;
  color: #c0392b;
}
.fmt-epub {
  background-color: #e8f8e8;
  color: #27ae60;
}
.fmt-mobi {
  background-color: #e8f0fd;
  color: #2980b9;
}
.fmt-azw3 {
  background-color: #f0e8fd;
  color: #8e44ad;
}
.fmt-txt {
  background-color: #f0f0f0;
  color: #666;
}
@media (prefers-color-scheme: dark) {
  .fmt-pdf {
    background-color: #3d1f1f;
    color: #e74c3c;
  }
  .fmt-epub {
    background-color: #1f3d1f;
    color: #2ecc71;
  }
  .fmt-mobi {
    background-color: #1f2a3d;
    color: #3498db;
  }
  .fmt-azw3 {
    background-color: #2d1f3d;
    color: #a66bbe;
  }
  .fmt-txt {
    background-color: #2a2a2a;
    color: #999;
  }
}
.book-info {
  min-width: 0;
  flex: 1;
}
.book-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.book-meta {
  display: flex;
  gap: 12px;
  margin-top: 3px;
  font-size: 12px;
  color: var(--text-secondary);
}
.book-path {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  min-width: 0;
}
.book-size {
  flex-shrink: 0;
}
.book-actions {
  display: flex;
  gap: 2px;
  flex-shrink: 0;
  margin-left: 8px;
}
.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 30px;
  height: 30px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: background-color 0.15s, color 0.15s;
}
.btn-icon:hover {
  background-color: var(--hover-bg);
  color: var(--text-primary);
}
.book-name-edit {
  display: flex;
  align-items: center;
}
.rename-input {
  width: 100%;
  padding: 2px 6px;
  border: 1px solid var(--accent-color);
  border-radius: 4px;
  background: var(--input-bg);
  color: var(--text-primary);
  font-size: 14px;
  font-weight: 500;
  outline: none;
}
</style>
