<template>
  <router-view v-slot="{ Component }">
    <Transition name="slide-left" mode="out-in">
      <component :is="Component" class="router-view-content" />
    </Transition>
  </router-view>
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { invoke } from '@tauri-apps/api/core';

const version = await invoke<string>('get_build_version');
console.log("Mesa Version:", version);

onMounted(async () => {
  try {
    const win = getCurrentWindow();
    await win.setMinSize(new LogicalSize(1040, 680));
  } catch (error) {
    console.error("Error setting minimum size for app:", error);
  }
});
</script>

<style>
:root {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 1.5;
  font-weight: 400;
  color: #ffffff;
  background-color: #1a1a1a;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

body {
  margin: 0;
  padding: 0;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

#app {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  position: relative;
}

* {
  box-sizing: border-box;
}

/* Original input, button, and anchor styles - Centralized here */
input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  background-color: #2a2a2a;
  color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

input:focus,
button:focus {
  outline: none;
  border-color: #666666;
}

a {
  font-weight: 500;
  color: #cccccc;
  text-decoration: inherit;
}

a:hover {
  color: #ffffff;
}

.router-view-content {
  width: 100%;
  height: 100%;
  position: absolute;
  top: 0;
  left: 0;
}

.slide-left-enter-active,
.slide-left-leave-active {
  transition: all 0.3s ease-out;
}

.slide-left-enter-from {
  transform: translateX(100%);
  opacity: 0;
}

.slide-left-leave-to {
  transform: translateX(-100%);
  opacity: 0;
}
</style>