<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

const emit = defineEmits<{ (e: "back"): void }>();

const directories = ref<string[]>([]);
const scanning = ref(false);

async function loadDirectories() {
  directories.value = await invoke<string[]>("get_directories");
}

async function addDirectory() {
  const selected = await open({ directory: true, multiple: false });
  if (selected) {
    directories.value = await invoke<string[]>("add_directory", {
      path: selected as string,
    });
  }
}

async function removeDirectory(path: string) {
  directories.value = await invoke<string[]>("remove_directory", { path });
}

async function rescan() {
  scanning.value = true;
  try {
    await invoke("scan_books");
  } finally {
    scanning.value = false;
  }
}

onMounted(loadDirectories);
</script>

<template>
  <div class="settings-view">
    <header class="toolbar">
      <button class="btn-back" @click="emit('back')">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <path d="M10 3L5 8L10 13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        返回
      </button>
      <span class="toolbar-title">设置</span>
    </header>

    <div class="settings-content">
      <section class="section">
        <h3 class="section-title">扫描目录</h3>
        <div v-if="directories.length === 0" class="empty-hint">
          尚未添加任何扫描目录
        </div>
        <div v-else class="dir-list">
          <div v-for="dir in directories" :key="dir" class="dir-item">
            <span class="dir-path">{{ dir }}</span>
            <button class="btn-delete" @click="removeDirectory(dir)">删除</button>
          </div>
        </div>
        <button class="btn-primary" @click="addDirectory">+ 添加目录</button>
      </section>

      <section class="section">
        <button class="btn-secondary" :disabled="scanning" @click="rescan">
          {{ scanning ? "扫描中..." : "重新扫描" }}
        </button>
      </section>
    </div>
  </div>
</template>

<style scoped>
.settings-view {
  display: flex;
  flex-direction: column;
  height: 100vh;
}
.toolbar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-color);
}
.btn-back {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 10px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  font-size: 14px;
  transition: background-color 0.15s;
}
.btn-back:hover {
  background-color: var(--hover-bg);
}
.toolbar-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}
.settings-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px 16px;
}
.section {
  margin-bottom: 24px;
}
.section-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 12px 0;
}
.empty-hint {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 12px;
}
.dir-list {
  margin-bottom: 12px;
}
.dir-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  margin-bottom: 6px;
}
.dir-path {
  font-size: 13px;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  min-width: 0;
}
.btn-delete {
  flex-shrink: 0;
  margin-left: 8px;
  padding: 4px 10px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: #e55;
  cursor: pointer;
  font-size: 12px;
  transition: background-color 0.15s;
}
.btn-delete:hover {
  background-color: rgba(238, 85, 85, 0.1);
}
.btn-primary {
  padding: 8px 16px;
  border: 1px dashed var(--border-color);
  border-radius: 6px;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  font-size: 13px;
  width: 100%;
  transition: border-color 0.15s, background-color 0.15s;
}
.btn-primary:hover {
  border-color: var(--accent-color);
  background-color: var(--hover-bg);
}
.btn-secondary {
  padding: 8px 16px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--accent-color);
  color: #fff;
  cursor: pointer;
  font-size: 13px;
  transition: opacity 0.15s;
}
.btn-secondary:hover {
  opacity: 0.9;
}
.btn-secondary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
