<script setup lang="ts">
import { ref } from "vue";
import MainView from "./components/MainView.vue";
import SettingsView from "./components/SettingsView.vue";

const page = ref<"main" | "settings">("main");
</script>

<template>
  <MainView v-if="page === 'main'" @open-settings="page = 'settings'" />
  <SettingsView v-else @back="page = 'main'" />
</template>

<style>
:root {
  --text-primary: #1a1a1a;
  --text-secondary: #888;
  --border-color: #e0e0e0;
  --hover-bg: #f0f0f0;
  --input-bg: #fff;
  --accent-color: #4a90d9;
  --bg-color: #fff;
}

@media (prefers-color-scheme: dark) {
  :root {
    --text-primary: #e8e8e8;
    --text-secondary: #999;
    --border-color: #3a3a3a;
    --hover-bg: #2a2a2a;
    --input-bg: #1e1e1e;
    --accent-color: #5b9bd5;
    --bg-color: #1a1a1a;
  }
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  font-size: 14px;
  color: var(--text-primary);
  background-color: var(--bg-color);
  -webkit-font-smoothing: antialiased;
}
</style>
