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