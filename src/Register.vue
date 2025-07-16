<script setup lang="ts">
import { ref, watch } from "vue"; // Added watch
import { useRouter } from 'vue-router';
import { invoke } from "@tauri-apps/api/core";

// TODO: Implement this all later and better (localstorage encryption)

const router = useRouter();

const username = ref("");
const password = ref("");
const confirmPassword = ref("");
const isLoading = ref(false);
const errorMessage = ref("");

const defaultServerAddress = "standard.qzz.io";
const useDefaultServer = ref(true);
const customServerAddress = ref("");

watch(useDefaultServer, (newValue) => {
    if (newValue) {
        customServerAddress.value = "";
    }
});

async function handleRegister() {
    if (!username.value || !password.value || !confirmPassword.value) {
        errorMessage.value = "Please fill in all required fields.";
        return;
    }
    if (password.value !== confirmPassword.value) {
        errorMessage.value = "Passwords do not match.";
        return;
    }

    let serverToUse = "";
    if (useDefaultServer.value) {
        serverToUse = defaultServerAddress;
    } else {
        if (!customServerAddress.value) {
            errorMessage.value = "Please enter a custom server address.";
            return;
        }
        serverToUse = customServerAddress.value;
    }

    try {
        isLoading.value = true;
        errorMessage.value = "";

        // Step 1: Generate keypair
        const keypair = await invoke("generate_keypair");
        console.log("Generated keypair:", keypair);

        // Step 2: Encrypt the private key with the user's password
        // Type guard for keypair
        if (
            typeof keypair !== "object" ||
            keypair === null ||
            !("private_key" in keypair) ||
            typeof (keypair as any).private_key !== "string"
        ) {
            throw new Error("Invalid keypair returned from generate_keypair");
        }
        const encryptedKey = await invoke("encrypt_private_key", {
            privateKeyBase64: (keypair as any).private_key,
            password: password.value
        });
        console.log("Encrypted private key:", encryptedKey);

        // Step 3: Register with the server
        const registerResult = await invoke("register", {
            username: username.value,
            password: password.value,
            server: serverToUse
        });
        console.log("Registration successful:", registerResult);

        // Step 4: Store the encrypted private key locally
        await invoke("save_my_key", {
            encrypted: encryptedKey
        });
        console.log("Private key saved locally");

        // TODO: implement navigation
        router.push('/login');

    } catch (error) {
        console.error("Registration error:", error);
        errorMessage.value = error instanceof Error ? error.message : "Registration failed.";
    } finally {
        isLoading.value = false;
    }
}

const goBack = () => {
    router.push('/login');
};
</script>

<template>
    <main class="app-container">
        <div class="register-container">
            <div class="register-content">
                <div class="logo-section">
                    <img src="/logo.svg" alt="Project Mesa Logo" class="register-logo" />
                    <h1 class="register-title">Project Mesa</h1>
                    <p class="register-subtitle">Create your account</p>
                </div>

                <form @submit.prevent="handleRegister" class="register-form">
                    <div class="form-group">
                        <label for="username" class="form-label">Username</label>
                        <input id="username" v-model="username" type="text" class="form-input"
                            placeholder="Choose a username" :disabled="isLoading" autocomplete="username" />
                    </div>

                    <div class="form-group">
                        <label for="password" class="form-label">Password</label>
                        <input id="password" v-model="password" type="password" class="form-input"
                            placeholder="Create a password" :disabled="isLoading" autocomplete="new-password" />
                    </div>

                    <div class="form-group">
                        <label for="confirmPassword" class="form-label">Confirm Password</label>
                        <input id="confirmPassword" v-model="confirmPassword" type="password" class="form-input"
                            placeholder="Confirm your password" :disabled="isLoading" autocomplete="new-password" />
                    </div>

                    <div class="form-group">
                        <label class="checkbox-container">
                            <input type="checkbox" v-model="useDefaultServer" class="checkbox-input"
                                :disabled="isLoading" />
                            <span class="checkbox-custom"></span>
                            <span class="checkbox-label">Use default server ({{ defaultServerAddress }})</span>
                        </label>
                    </div>

                    <div class="form-group" v-if="!useDefaultServer">
                        <label for="customServer" class="form-label">Custom Server Address</label>
                        <input id="customServer" v-model="customServerAddress" type="text" class="form-input"
                            placeholder="e.g., my.custom.server.com" :disabled="isLoading" />
                    </div>

                    <div v-if="errorMessage" class="error-message">
                        {{ errorMessage }}
                    </div>

                    <button type="submit" class="register-button" :disabled="isLoading"
                        :class="{ 'loading': isLoading }">
                        <span v-if="!isLoading">REGISTER</span>
                        <span v-else class="loading-spinner"></span>
                    </button>
                </form>

                <div class="register-footer">
                    <p class="login-text">
                        Already have an account?
                        <a href="#" class="login-link" @click.prevent="goBack">Sign in</a>
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



.register-container {
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

.register-content {
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

.register-logo {
    width: 80px;
    height: 80px;
    filter: brightness(0) invert(1);
    margin-bottom: 1rem;
    object-fit: contain;
}

.register-title {
    font-size: 2rem;
    font-weight: 300;
    margin-bottom: 0.5rem;
    color: #ffffff;
    letter-spacing: 0.5px;
}

.register-subtitle {
    font-size: 1rem;
    color: #cccccc;
    margin-bottom: 0;
    font-weight: 400;
}

.register-form {
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

.error-message {
    background: rgba(220, 53, 69, 0.1);
    border: 1px solid rgba(220, 53, 69, 0.3);
    border-radius: 6px;
    padding: 0.75rem;
    font-size: 0.9rem;
    color: #ff6b6b;
    text-align: center;
}

.register-button {
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

.register-button:hover:not(:disabled) {
    background: #f0f0f0;
    transform: translateY(-2px);
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.3);
}

.register-button:active:not(:disabled) {
    transform: translateY(0);
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
}

.register-button:disabled {
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

.register-footer {
    margin-top: 2rem;
    text-align: center;
}

.login-text {
    font-size: 0.9rem;
    color: #999999;
    margin: 0;
}

.login-link {
    color: #ffffff;
    text-decoration: none;
    font-weight: 500;
    transition: color 0.3s ease;
}

.login-link:hover {
    color: #cccccc;
}

@media (max-width: 600px) {
    .register-container {
        padding: 1rem;
    }

    .register-content {
        padding: 2rem;
    }

    .register-logo {
        width: 60px;
        height: 60px;
    }

    .register-title {
        font-size: 1.5rem;
    }
}

@media (max-width: 400px) {
    .register-content {
        padding: 1.5rem;
    }
}
</style>