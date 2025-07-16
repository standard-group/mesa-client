<template>
  <div v-if="isDesktop" class="titlebar">
    <div class="titlebar-left">
      <button class="back-button" @click="goBack" type="button" v-if="$route.meta.showBack">
        <svg width="16" height="16" viewBox="0 0 16 16">
          <path d="M10 12L6 8L10 4" stroke="currentColor" stroke-width="2" stroke-linecap="round"
            stroke-linejoin="round" fill="none" />
        </svg>
      </button>
      <div class="titlebar-title">Project Mesa - {{ version }}</div>
    </div>
    <div class="titlebar-controls">
      <button class="titlebar-button minimize" @click.stop="minimize" type="button">
        <svg width="16" height="16" viewBox="0 0 16 16">
          <rect x="3" y="7" width="10" height="2" fill="currentColor" />
        </svg>
      </button>
      <button class="titlebar-button maximize" @click.stop="maximize" type="button">
        <svg width="16" height="16" viewBox="0 0 16 16">
          <rect x="3" y="3" width="10" height="10" stroke="currentColor" stroke-width="1.5" fill="none" />
        </svg>
      </button>
      <button class="titlebar-button close" @click.stop="close" type="button">
        <svg width="16" height="16" viewBox="0 0 16 16">
          <path d="M4 4L12 12M12 4L4 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
        </svg>
      </button>
    </div>
  </div>

  <router-view v-slot="{ Component }">
    <Transition name="slide-left" mode="out-in">
      <component :is="Component" class="router-view-content" />
    </Transition>
  </router-view>
</template>

<script setup lang="ts">
import { ref, provide, onMounted } from "vue";
import { useRouter } from 'vue-router';
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { invoke } from '@tauri-apps/api/core';
import { type as getOsType } from '@tauri-apps/plugin-os';

const router = useRouter();
const version = await invoke<string>('get_build_version');
const isDesktop = ref(false);
provide('isDesktop', isDesktop);

onMounted(async () => {
  // set minimum window size on desktop only
  try {
    const win = getCurrentWindow();
    await win.setMinSize(new LogicalSize(1040, 680));
  } catch {
    /* ignore on mobile */
  }

  // detect platform
  const os = await getOsType();
  isDesktop.value = ['windows', 'linux', 'darwin'].includes(os);
});

/* titlebar */

const win = getCurrentWindow();
const minimize = () => win.minimize();
const maximize = async () => {
  (await win.isMaximized()) ? win.unmaximize() : win.maximize();
};
const close = () => win.close();

const goBack = () => {
    router.push({ name: 'Start' });
};

/* build */

console.log('Mesa Version:', version);
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
  /* i hope it is better */
  transition: transform 0.4s cubic-bezier(0.36, 0.66, 0.04, 1),
    opacity 0.4s cubic-bezier(0.36, 0.66, 0.04, 1);
}

.slide-left-enter-from {
  transform: translateX(30px);
  /* start slightly closer for a quicker feel */
  opacity: 0;
}

.slide-left-leave-to {
  transform: translateX(-30px);
  /* exit slightly closer */
  opacity: 0;
}

/* ** TITLEBAR ** */

.titlebar {
    -webkit-app-region: drag;
    height: 40px;
    background: rgba(26, 26, 26, 0.95);
    backdrop-filter: blur(10px);
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 16px;
    user-select: none;
    flex-shrink: 0;
    position: relative;
    z-index: 1000;
    cursor: move;
}

.titlebar-left {
    display: flex;
    align-items: center;
    gap: 8px;
    -webkit-app-region: no-drag;
    cursor: default;
}

.titlebar-title {
    font-size: 14px;
    font-weight: 500;
    color: #cccccc;
    pointer-events: none;
}

.titlebar-controls {
    -webkit-app-region: no-drag;
    display: flex;
    align-items: center;
    gap: 4px;
    height: 100%;
    cursor: default;
}

.titlebar-button {
    -webkit-app-region: no-drag;
    width: 40px;
    height: 32px;
    border: none;
    border-radius: 4px;
    background: transparent;
    color: #cccccc;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s ease;
    font-size: 12px;
}

.back-button {
    -webkit-app-region: no-drag;
    width: 40px;
    height: 32px;
    border: none;
    border-radius: 4px;
    background: transparent;
    color: #cccccc;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s ease;
    font-size: 12px;
}

.back-button:hover {
    background: rgba(255, 255, 255, 0.1);
}

.back-button:active {
    transform: scale(0.95);
}

.back-button svg {
    width: 16px;
    height: 16px;
}

.titlebar-button:hover {
    background: rgba(255, 255, 255, 0.1);
}

.titlebar-button.minimize:hover {
    background: rgba(255, 196, 0, 0.15);
    color: #ffc400;
}

.titlebar-button.maximize:hover {
    background: rgba(40, 167, 69, 0.15);
    color: #28a745;
}

.titlebar-button.close:hover {
    background: rgba(220, 53, 69, 0.15);
    color: #dc3545;
}

.titlebar-button:active {
    transform: scale(0.95);
}

.titlebar-button svg {
    width: 16px;
    height: 16px;
}

@media (max-width: 600px) {
    .titlebar {
        height: 36px;
    }

    .titlebar-title {
        font-size: 13px;
    }

    .titlebar-button {
        width: 36px;
        height: 28px;
    }

    .app-container {
        flex-direction: column;
    }

    .header-section {
        flex: 0.5;
        min-height: 35vh;
        padding: 1rem;
    }

    .content-section {
        flex: 0.5;
        min-height: 30vh;
        padding: 1rem;
    }
}

@media (max-width: 400px) {
    .titlebar {
        height: 32px;
        padding: 0 12px;
    }

    .titlebar-title {
        font-size: 12px;
    }

    .titlebar-button {
        width: 32px;
        height: 24px;
    }

    .header-section {
        min-height: 30vh;
        padding: 0.5rem;
    }

    .content-section {
        padding: 0.5rem;
    }
}

@media (max-width: 768px) and (orientation: landscape) {
    .app-container {
        flex-direction: row;
    }

    .titlebar {
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        z-index: 1000;
    }

    .header-section {
        flex: 0.5;
        min-height: calc(100vh - 40px);
        margin-top: 40px;
    }

    .content-section {
        flex: 0.5;
        min-height: calc(100vh - 40px);
        margin-top: 40px;
    }
}
</style>