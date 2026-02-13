<script setup lang="ts">
import { ref, onMounted, watch, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useVirtualizer } from "@tanstack/vue-virtual";
import type { BookInfo } from "../types";
import BookItem from "./BookItem.vue";

const emit = defineEmits<{ (e: "openSettings"): void }>();

const keyword = ref("");
const books = ref<BookInfo[]>([]);
const loading = ref(false);
const scrollRef = ref<HTMLDivElement | null>(null);

const bookCount = computed(() => books.value.length);

const virtualizer = useVirtualizer({
  get count() { return bookCount.value; },
  getScrollElement: () => scrollRef.value,
  estimateSize: () => 58,
  overscan: 5,
});

let searchTimer: ReturnType<typeof setTimeout> | null = null;

async function loadBooks() {
  loading.value = true;
  try {
    books.value = await invoke<BookInfo[]>("scan_books");
  } finally {
    loading.value = false;
  }
}

async function doSearch() {
  books.value = await invoke<BookInfo[]>("search_books", {
    keyword: keyword.value,
  });
}

watch(keyword, () => {
  if (searchTimer) clearTimeout(searchTimer);
  searchTimer = setTimeout(doSearch, 200);
});

onMounted(loadBooks);
</script>

<template>
  <div class="main-view">
    <header class="toolbar">
      <input
        v-model="keyword"
        class="search-input"
        type="text"
        placeholder="搜索书籍..."
      />
      <button class="btn-icon" @click="emit('openSettings')" title="设置">
        <svg width="18" height="18" viewBox="0 0 18 18" fill="none">
          <path d="M7.5 2.25L8.1 4.05C8.55 4.2 8.97 4.42 9.35 4.7L11.2 4.15L12.7 6.75L11.25 8.1C11.28 8.37 11.3 8.68 11.3 9C11.3 9.32 11.28 9.63 11.25 9.9L12.7 11.25L11.2 13.85L9.35 13.3C8.97 13.58 8.55 13.8 8.1 13.95L7.5 15.75H4.5L3.9 13.95C3.45 13.8 3.03 13.58 2.65 13.3L0.8 13.85L-0.7 11.25L0.75 9.9C0.72 9.63 0.7 9.32 0.7 9C0.7 8.68 0.72 8.37 0.75 8.1L-0.7 6.75L0.8 4.15L2.65 4.7C3.03 4.42 3.45 4.2 3.9 4.05L4.5 2.25H7.5ZM6 6.75C4.76 6.75 3.75 7.76 3.75 9C3.75 10.24 4.76 11.25 6 11.25C7.24 11.25 8.25 10.24 8.25 9C8.25 7.76 7.24 6.75 6 6.75Z" transform="translate(3,0)" stroke="currentColor" stroke-width="1.2" fill="none"/>
        </svg>
      </button>
    </header>

    <div v-if="loading" class="status-message">正在扫描...</div>
    <div v-else-if="books.length === 0 && keyword" class="status-message">
      未找到匹配的书籍
    </div>
    <div v-else-if="books.length === 0" class="status-message">
      暂无书籍，请先在设置中添加扫描目录
    </div>
    <div v-else ref="scrollRef" class="book-list">
      <div :style="{ height: `${virtualizer.getTotalSize()}px`, position: 'relative' }">
        <div
          v-for="row in virtualizer.getVirtualItems()"
          :key="books[row.index].path"
          :style="{
            position: 'absolute',
            top: 0,
            left: 0,
            width: '100%',
            transform: `translateY(${row.start}px)`,
          }"
        >
          <BookItem :book="books[row.index]" @renamed="doSearch" />
        </div>
      </div>
    </div>

    <footer class="footer">
      共 {{ books.length }} 本书籍
    </footer>
  </div>
</template>

<style scoped>
.main-view {
  display: flex;
  flex-direction: column;
  height: 100vh;
}
.toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  border-bottom: 1px solid var(--border-color);
}
.search-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--input-bg);
  color: var(--text-primary);
  font-size: 14px;
  outline: none;
  transition: border-color 0.15s;
}
.search-input:focus {
  border-color: var(--accent-color);
}
.search-input::placeholder {
  color: var(--text-secondary);
}
.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: background-color 0.15s, color 0.15s;
}
.btn-icon:hover {
  background-color: var(--hover-bg);
  color: var(--text-primary);
}
.book-list {
  flex: 1;
  overflow-y: auto;
  padding: 4px 8px;
}
.status-message {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary);
  font-size: 14px;
}
.footer {
  padding: 8px 16px;
  text-align: center;
  font-size: 12px;
  color: var(--text-secondary);
  border-top: 1px solid var(--border-color);
}
</style>
