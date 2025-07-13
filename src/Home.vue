<script setup lang="ts">
import { useRouter } from 'vue-router';
import { getCurrentWindow } from "@tauri-apps/api/window";

const win = getCurrentWindow();
const router = useRouter();

const startMessaging = () => {
    console.log("Starting messaging...");
    router.push('/login');
};

const minimizeWindow = async () => {
    try {
        await win.minimize();
    } catch (error) {
        console.error("minimize failed:", error);
    }
};

const maximizeWindow = async () => {
    try {
        const isMax = await win.isMaximized();
        if (isMax) {
            await win.unmaximize();
        } else {
            await win.maximize();
        }
    } catch (error) {
        console.error("toggleMaximize failed:", error);
    }
};

const closeWindow = async () => {
    try {
        await win.close();
    } catch (e) { console.error("close failed", e); }
}

</script>

<template>
    <main class="app-container">
        <div class="titlebar">
            <div class="titlebar-title">Project Mesa</div>
            <div class="titlebar-controls">
                <button class="titlebar-button minimize" @click.stop="minimizeWindow" type="button">
                    <svg width="16" height="16" viewBox="0 0 16 16">
                        <rect x="3" y="7" width="10" height="2" fill="currentColor" />
                    </svg>
                </button>
                <button class="titlebar-button maximize" @click.stop="maximizeWindow" type="button">
                    <svg width="16" height="16" viewBox="0 0 16 16">
                        <rect x="3" y="3" width="10" height="10" stroke="currentColor" stroke-width="1.5" fill="none" />
                    </svg>
                </button>
                <button class="titlebar-button close" @click.stop="closeWindow" type="button">
                    <svg width="16" height="16" viewBox="0 0 16 16">
                        <path d="M4 4L12 12M12 4L4 12" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
                    </svg>
                </button>
            </div>
        </div>

        <div class="header-section">
            <div class="logo-container">
                <img src="/logo.svg" alt="Project Mesa Logo" class="main-logo" />
            </div>
        </div>

        <div class="content-section">
            <div class="welcome-container">
                <h1 class="app-title">Project Mesa</h1>
                <p class="app-subtitle">Welcome to the official Project Mesa desktop app.</p>
                <p class="app-description">It's fast and secure.</p>

                <button class="start-button" @click="startMessaging">
                    START MESSAGING
                </button>
            </div>
        </div>
    </main>
</template>

<style scoped>
.app-container {
    height: 100vh;
    width: 100vw;
    display: flex;
    flex-direction: column;
    background: linear-gradient(135deg, #2c2c2c 0%, #1a1a1a 100%);
    color: #ffffff;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
    overflow: hidden;
}

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

.header-section {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    background: linear-gradient(135deg, #3a3a3a 0%, #2c2c2c 100%);
    padding: 2rem;
    background-image: url('/startbg.jpg');
    background-size: cover;
    background-position: center center;
    background-repeat: no-repeat;
    background-blend-mode: overlay;
    min-height: 0;
    overflow: hidden;
}

.logo-container {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    padding: 2rem;
}

.main-logo {
    width: clamp(80px, 20vw, 180px);
    height: clamp(80px, 20vw, 180px);
    filter: brightness(0) invert(1);
    z-index: 2;
    position: relative;
    object-fit: contain;
}

.content-section {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: clamp(1rem, 4vw, 2rem);
    background: #1a1a1a;
    min-height: 0;
}

.welcome-container {
    text-align: center;
    max-width: 400px;
}

.app-title {
    font-size: clamp(1.5rem, 5vw, 2.5rem);
    font-weight: 300;
    margin-bottom: 1rem;
    color: #ffffff;
    letter-spacing: 0.5px;
}

.app-subtitle {
    font-size: clamp(0.9rem, 3vw, 1.1rem);
    color: #cccccc;
    margin-bottom: 0.5rem;
    font-weight: 400;
}

.app-description {
    font-size: clamp(0.8rem, 2.5vw, 1rem);
    color: #999999;
    margin-bottom: 2.5rem;
    font-weight: 300;
}

.start-button {
    background: #ffffff;
    color: #1a1a1a;
    border: none;
    padding: clamp(0.8rem, 2vw, 1rem) clamp(1.5rem, 4vw, 2rem);
    font-size: clamp(0.85rem, 2.5vw, 1rem);
    font-weight: 600;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.3s ease;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    min-width: clamp(140px, 30vw, 180px);
}

.start-button:hover {
    background: #f0f0f0;
    transform: translateY(-2px);
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.3);
}

.start-button:active {
    transform: translateY(0);
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
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

@media (min-width: 400px) and (min-height: 500px) {
    .main-logo {
        width: 140px;
        height: 140px;
    }
}

@media (min-width: 600px) and (min-height: 700px) {
    .main-logo {
        width: 180px;
        height: 180px;
    }
}
</style>