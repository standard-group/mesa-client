// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// commenting
use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use pbkdf2::pbkdf2_hmac;
use rand::{thread_rng, RngCore};
use reqwest;
use sha2::Sha256;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize)]
pub struct KeypairResponse {
    pub public_key: String,
    pub private_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedKeyResponse {
    pub encrypted_data_base64: String,
    pub salt_base64: String,
    pub nonce_base64: String, // IV for AES-GCM
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedAndEncryptedKeys {
    pub public_key: String,
    pub encrypted_key_package: EncryptedKeyResponse,
}

fn get_key_storage_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|_| "Failed to find application config directory.".to_string())?;
    let path = dir.join("mesa").join("private_key.json");
    Ok(path)
}

fn store_encrypted_key(app_handle: &AppHandle, data: &EncryptedKeyResponse) -> Result<(), String> {
    let json =
        serde_json::to_string(data).map_err(|_| "Failed to serialize key data.".to_string())?;
    let path = get_key_storage_path(app_handle)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|_| "Failed to create key storage directory.".to_string())?;
    }
    std::fs::write(path, json).map_err(|_| "Failed to write key file to disk.".to_string())
}

fn load_encrypted_key(app_handle: &AppHandle) -> Result<EncryptedKeyResponse, String> {
    let path = get_key_storage_path(app_handle)?;
    let content =
        std::fs::read_to_string(path).map_err(|_| "Failed to read key file.".to_string())?;
    let data: EncryptedKeyResponse =
        serde_json::from_str(&content).map_err(|_| "Failed to parse key file.".to_string())?;
    Ok(data)
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Generates a new keypair and immediately encrypts the private key with the user's password.
/// This prevents the plaintext private key from ever being sent to the frontend.
#[tauri::command]
async fn generate_and_encrypt_keypair(
    password: String,
) -> Result<GeneratedAndEncryptedKeys, String> {
    // 1. Generate keypair inside this function
    let mut csprng = OsRng;
    let keypair: SigningKey = SigningKey::generate(&mut csprng);
    let public_key_bytes = keypair.verifying_key().to_bytes();
    let secret_key_bytes = keypair.to_bytes(); // Kept in memory only for this function's scope

    // 2. Generate a random salt
    let mut salt = [0u8; 16];
    thread_rng().fill_bytes(&mut salt);

    // 3. Derive key from password with increased iterations
    const PBKDF2_ITERATIONS: u32 = 600_000;
    let mut key_bytes = [0u8; 32];
    pbkdf2_hmac::<Sha256>(
        &password.as_bytes(),
        &salt,
        PBKDF2_ITERATIONS,
        &mut key_bytes,
    );
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // 4. Generate a random nonce
    let mut nonce_bytes = [0u8; 12];
    thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 5. Encrypt the secret key bytes
    let ciphertext = cipher
        .encrypt(nonce, secret_key_bytes.as_ref())
        .map_err(|_| "Encryption failed".to_string())?;

    // 6. Return only the public key and the encrypted blob
    Ok(GeneratedAndEncryptedKeys {
        public_key: general_purpose::STANDARD.encode(&public_key_bytes),
        encrypted_key_package: EncryptedKeyResponse {
            encrypted_data_base64: general_purpose::STANDARD.encode(&ciphertext),
            salt_base64: general_purpose::STANDARD.encode(&salt),
            nonce_base64: general_purpose::STANDARD.encode(&nonce_bytes),
        },
    })
}

/// Decrypts an encrypted private key using AES-256 GCM with a key derived from a password via PBKDF2.
/// Returns the decrypted private key as a base64-encoded string.
#[tauri::command]
async fn decrypt_private_key(
    encrypted_response: EncryptedKeyResponse,
    password: String,
) -> Result<String, String> {
    // Decode the base64 components
    let encrypted_data_bytes = general_purpose::STANDARD
        .decode(&encrypted_response.encrypted_data_base64)
        .map_err(|_| "Failed to decode encrypted data.".to_string())?;
    let salt_bytes = general_purpose::STANDARD
        .decode(&encrypted_response.salt_base64)
        .map_err(|_| "Failed to decode salt.".to_string())?;
    let nonce_bytes = general_purpose::STANDARD
        .decode(&encrypted_response.nonce_base64)
        .map_err(|_| "Failed to decode nonce.".to_string())?;

    // Derive the key using the provided password and the stored salt
    const PBKDF2_ITERATIONS: u32 = 600_000;
    let mut key_bytes = [0u8; 32];
    pbkdf2_hmac::<Sha256>(
        password.as_bytes(),
        &salt_bytes,
        PBKDF2_ITERATIONS,
        &mut key_bytes,
    );
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(&nonce_bytes);

    // Decrypt the data
    let plaintext = cipher
        .decrypt(nonce, encrypted_data_bytes.as_ref())
        .map_err(|_| "Decryption failed. Please check your password.".to_string())?;

    // Return the decrypted private key as base64
    Ok(general_purpose::STANDARD.encode(&plaintext))
}

#[tauri::command]
async fn register(
    app_handle: AppHandle,
    username: String,
    password: String,
    server: String,
) -> Result<String, String> {
    let generated_keys = generate_and_encrypt_keypair(password.clone()).await?;

    store_encrypted_key(&app_handle, &generated_keys.encrypted_key_package)?;

    let payload = serde_json::json!({
        "username": username,
        "server_domain": server,
        "password": password,
        "pubkey": generated_keys.public_key,
    });

    // println!("{}", &payload.to_string());

    let url = format!("https://{}/api/v1/register", server);

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&payload)
        .send()
        .await
        .map_err(|_| "Failed to send registration request to the server.".to_string())?;

    let body = res
        .text()
        .await
        .map_err(|_| "Failed to read response from the server.".to_string())?;
    Ok(body)
}

#[tauri::command]
async fn login(username: String, password: String, server: String) -> Result<String, String> {
    let payload = serde_json::json!({
        "username": username,
        "server_domain": server,
        "password": password,
    });

    let url = format!("https://{}/api/v1/login", server);

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&payload)
        .send()
        .await
        .map_err(|_| "Failed to send login request to the server.".to_string())?;

    let body = res
        .text()
        .await
        .map_err(|_| "Failed to read response from the server.".to_string())?;
    Ok(body)
}

#[tauri::command]
async fn save_my_key(app_handle: AppHandle, encrypted: EncryptedKeyResponse) -> Result<(), String> {
    store_encrypted_key(&app_handle, &encrypted)
}

#[tauri::command]
async fn load_my_key(app_handle: AppHandle) -> Result<EncryptedKeyResponse, String> {
    load_encrypted_key(&app_handle)
}

#[tauri::command]
fn get_build_version() -> String {
    let base_version = env!("CARGO_PKG_VERSION");
    let build_date = env!("BUILD_DATE");
    let git_commit = env!("GIT_COMMIT");

    let build_type = option_env!("MESA_BUILD_TYPE").unwrap_or("release");

    match build_type {
        "release" => format!("v{}", base_version),
        "beta" => format!("v{}-beta+{}", base_version, build_date),
        "nightly" => format!("v{}+{}", base_version, build_date),
        "debug" => format!("v{}+git{}", base_version, git_commit),
        "internal" => format!("{}-dev", build_date),
        _ => format!("v{}+{}", base_version, build_date),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            generate_and_encrypt_keypair,
            decrypt_private_key,
            register,
            login,
            save_my_key,
            load_my_key,
            get_build_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
