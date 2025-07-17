<script setup lang="ts">
import { useRouter } from 'vue-router';

const router = useRouter();

const startMessaging = () => {
    console.log("Starting messaging...");
    router.push('/login');
};
</script>

<template>
    <main class="app-container">
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

.header-section {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    background: linear-gradient(135deg, #3a3a3a 0%, #2c2c2c 100%);
    padding: clamp(1rem, 3vw, 2rem);
    background-image: url('/startbg.jpg');
    background-size: cover;
    background-position: center center;
    background-repeat: no-repeat;
    background-blend-mode: overlay;
    min-height: 40vh;
    overflow: hidden;
}

.logo-container {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    padding: clamp(1rem, 3vw, 2rem);
}

.main-logo {
    width: clamp(80px, 15vw, 180px);
    height: clamp(80px, 15vw, 180px);
    filter: brightness(0) invert(1);
    z-index: 2;
    position: relative;
    object-fit: contain;
    max-width: 180px;
    max-height: 180px;
}

.content-section {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: clamp(1rem, 4vw, 2rem);
    background: #1a1a1a;
    min-height: 40vh;
}

.welcome-container {
    text-align: center;
    max-width: min(400px, 90vw);
    width: 100%;
    padding: clamp(1rem, 2vw, 2rem);
}

.app-title {
    font-size: clamp(1.5rem, 4vw, 2.5rem);
    font-weight: 300;
    margin-bottom: clamp(0.8rem, 2vw, 1rem);
    color: #ffffff;
    letter-spacing: 0.5px;
    line-height: 1.2;
}

.app-subtitle {
    font-size: clamp(0.9rem, 2.5vw, 1.1rem);
    color: #cccccc;
    margin-bottom: clamp(0.4rem, 1vw, 0.5rem);
    font-weight: 400;
    line-height: 1.4;
}

.app-description {
    font-size: clamp(0.8rem, 2vw, 1rem);
    color: #999999;
    margin-bottom: clamp(2rem, 4vw, 2.5rem);
    font-weight: 300;
    line-height: 1.4;
}

.start-button {
    background: #ffffff;
    color: #1a1a1a;
    border: none;
    padding: clamp(0.8rem, 2vw, 1rem) clamp(1.5rem, 4vw, 2rem);
    font-size: clamp(0.85rem, 2vw, 1rem);
    font-weight: 600;
    border-radius: clamp(6px, 0.8vw, 8px);
    cursor: pointer;
    transition: all 0.3s ease;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    min-width: clamp(160px, 25vw, 200px);
    min-height: 44px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 0 auto;
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

@media (max-width: 768px) {
    .app-container {
        flex-direction: column;
    }

    .header-section {
        min-height: 35vh;
        flex: 0 0 auto;
    }

    .content-section {
        min-height: 35vh;
        flex: 1;
    }

    .main-logo {
        width: clamp(60px, 12vw, 120px);
        height: clamp(60px, 12vw, 120px);
    }
}

@media (max-width: 480px) {
    .header-section {
        min-height: 30vh;
        padding: clamp(0.5rem, 2vw, 1rem);
    }

    .content-section {
        min-height: 30vh;
        padding: clamp(0.5rem, 2vw, 1rem);
    }

    .main-logo {
        width: clamp(50px, 10vw, 100px);
        height: clamp(50px, 10vw, 100px);
    }
}

@media (max-width: 768px) and (orientation: landscape) {
    .app-container {
        flex-direction: row;
    }

    .header-section {
        flex: 1;
        min-height: 100vh;
        min-width: 40vw;
    }

    .content-section {
        flex: 1;
        min-height: 100vh;
        min-width: 40vw;
    }

    .main-logo {
        width: clamp(60px, 8vw, 120px);
        height: clamp(60px, 8vw, 120px);
    }
}

@media (-webkit-min-device-pixel-ratio: 2), (min-resolution: 192dpi) {
    .main-logo {
        image-rendering: -webkit-optimize-contrast;
        image-rendering: crisp-edges;
    }
}
</style>