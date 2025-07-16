<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from 'vue-router';
import { invoke } from "@tauri-apps/api/core";

const router = useRouter();

const username = ref("");
const password = ref("");
const serveraddress = ref("");
const rememberMe = ref(false);
const isLoading = ref(false);
const errorMessage = ref("");

async function handleLogin() {
    if (!username.value || !password.value) {
        errorMessage.value = "Please enter both username and password";
        return;
    }

    isLoading.value = true;
    errorMessage.value = "";

    try {
        const result = await invoke<string>("login", {
            username: username.value,
            password: password.value,
            server: serveraddress.value
        });

        if (result === "invalid credentials") {
            errorMessage.value = "Invalid username or password.";
        } else if (result === "success") {
            console.log("Login successful:", result);
            router.push({ name: 'Home' });
        } else {
            console.log("Login failed:", result);
            errorMessage.value = "An unknown error occurred. Please try again, or if the problem persists, share this problem with the developers: ", result;
        }

    } catch (error) {
        // this will now catch Rust panics or network errors
        console.error("Login invocation error:", error);
        errorMessage.value = "Could not connect to the login service.";
    } finally {
        isLoading.value = false;
    }
}

const goToRegister = () => {
    router.push('/register');
};
</script>

<template>
    <main class="app-container">
        <div class="login-container">
            <div class="login-content">
                <div class="logo-section">
                    <img src="/logo.svg" alt="Project Mesa Logo" class="login-logo" />
                    <h1 class="login-title">Project Mesa</h1>
                    <p class="login-subtitle">Sign in to your account</p>
                </div>

                <form @submit.prevent="handleLogin" class="login-form">
                    <div class="form-group">
                        <label for="username" class="form-label">Username</label>
                        <input id="username" v-model="username" type="text" class="form-input"
                            placeholder="Enter your username" :disabled="isLoading" autocomplete="username" />
                    </div>

                    <div class="form-group">
                        <label for="serveraddress" class="form-label">Server Address</label>
                        <input id="serveraddress" v-model="serveraddress" type="text" class="form-input"
                            placeholder="Enter server address (domain)" :disabled="isLoading" autocomplete="username" />
                    </div>

                    <div class="form-group">
                        <label for="password" class="form-label">Password</label>
                        <input id="password" v-model="password" type="password" class="form-input"
                            placeholder="Enter your password" :disabled="isLoading" autocomplete="current-password" />
                    </div>

                    <div class="form-options">
                        <label class="checkbox-container">
                            <input v-model="rememberMe" type="checkbox" class="checkbox-input" :disabled="isLoading" />
                            <span class="checkbox-custom"></span>
                            <span class="checkbox-label">Remember me</span>
                        </label>
                        <a href="#" class="forgot-password">Forgot password?</a>
                    </div>

                    <div v-if="errorMessage" class="error-message">
                        {{ errorMessage }}
                    </div>

                    <button type="submit" class="login-button" :disabled="isLoading" :class="{ 'loading': isLoading }">
                        <span v-if="!isLoading">SIGN IN</span>
                        <span v-else class="loading-spinner"></span>
                    </button>
                </form>

                <div class="login-footer">
                    <p class="signup-text">
                        Don't have an account?
                        <a href="#" class="signup-link" @click.prevent="goToRegister">Sign up</a>
                    </p>
                </div>
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
    overflow: hidden;
}

.login-container {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    background: #1a1a1a;
    background-image: url('/startbg.jpg');
    background-size: cover;
    background-position: center center;
    background-repeat: no-repeat;
    background-blend-mode: overlay;
}

.login-content {
    background: rgba(26, 26, 26, 0.95);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 3rem;
    width: 100%;
    max-width: 400px;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
}

.logo-section {
    text-align: center;
    margin-bottom: 2rem;
}

.login-logo {
    width: 80px;
    height: 80px;
    filter: brightness(0) invert(1);
    margin-bottom: 1rem;
    object-fit: contain;
}

.login-title {
    font-size: 2rem;
    font-weight: 300;
    margin-bottom: 0.5rem;
    color: #ffffff;
    letter-spacing: 0.5px;
}

.login-subtitle {
    font-size: 1rem;
    color: #cccccc;
    margin-bottom: 0;
    font-weight: 400;
}

.login-form {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.form-label {
    font-size: 0.9rem;
    color: #cccccc;
    font-weight: 500;
}

.form-input {
    padding: 0.75rem 1rem;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.05);
    color: #ffffff;
    font-size: 1rem;
    transition: all 0.3s ease;
}

.form-input:focus {
    outline: none;
    border-color: #ffffff;
    background: rgba(255, 255, 255, 0.1);
    box-shadow: 0 0 0 3px rgba(255, 255, 255, 0.1);
}

.form-input:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.form-input::placeholder {
    color: #999999;
}

.form-options {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.5rem;
}

.checkbox-container {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    user-select: none;
}

.checkbox-input {
    display: none;
}

.checkbox-custom {
    width: 18px;
    height: 18px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-radius: 3px;
    background: transparent;
    position: relative;
    transition: all 0.3s ease;
}

.checkbox-input:checked+.checkbox-custom {
    background: #ffffff;
    border-color: #ffffff;
}

.checkbox-input:checked+.checkbox-custom::after {
    content: '';
    position: absolute;
    left: 3px;
    top: 0px;
    width: 6px;
    height: 10px;
    border: solid #1a1a1a;
    border-width: 0 2px 2px 0;
    transform: rotate(45deg);
}

.checkbox-label {
    font-size: 0.9rem;
    color: #cccccc;
}

.forgot-password {
    font-size: 0.9rem;
    color: #cccccc;
    text-decoration: none;
    transition: color 0.3s ease;
}

.forgot-password:hover {
    color: #ffffff;
}

.error-message {
    background: rgba(220, 53, 69, 0.1);
    border: 1px solid rgba(220, 53, 69, 0.3);
    border-radius: 6px;
    padding: 0.75rem;
    font-size: 0.9rem;
    color: #ff6b6b;
    text-align: center;
}

.login-button {
    background: #ffffff;
    color: #1a1a1a;
    border: none;
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
    font-weight: 600;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.3s ease;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    min-height: 50px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.login-button:hover:not(:disabled) {
    background: #f0f0f0;
    transform: translateY(-2px);
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.3);
}

.login-button:active:not(:disabled) {
    transform: translateY(0);
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
}

.login-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none;
}

.loading-spinner {
    width: 20px;
    height: 20px;
    border: 2px solid #1a1a1a;
    border-top: 2px solid transparent;
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    0% {
        transform: rotate(0deg);
    }

    100% {
        transform: rotate(360deg);
    }
}

.login-footer {
    margin-top: 2rem;
    text-align: center;
}

.signup-text {
    font-size: 0.9rem;
    color: #999999;
    margin: 0;
}

.signup-link {
    color: #ffffff;
    text-decoration: none;
    font-weight: 500;
    transition: color 0.3s ease;
}

.signup-link:hover {
    color: #cccccc;
}

@media (max-width: 600px) {
    .login-container {
        padding: 1rem;
    }

    .login-content {
        padding: 2rem;
    }

    .login-logo {
        width: 60px;
        height: 60px;
    }

    .login-title {
        font-size: 1.5rem;
    }

    .form-options {
        flex-direction: column;
        gap: 1rem;
        align-items: flex-start;
    }
}

@media (max-width: 400px) {
    .login-content {
        padding: 1.5rem;
    }
}
</style>
