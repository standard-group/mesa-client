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
    pub private_key: String, // TODO: encrypt this server-side or encrypt it before sending to client storage.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedKeyResponse {
    pub encrypted_data_base64: String,
    pub salt_base64: String,
    pub nonce_base64: String, // IV for AES-GCM
}

fn get_key_storage_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|e| format!("Failed to find config dir: {}", e))?;
    let path = dir.join("mesa").join("private_key.json");
    Ok(path)
}

fn store_encrypted_key(app_handle: &AppHandle, data: &EncryptedKeyResponse) -> Result<(), String> {
    let json = serde_json::to_string(data).map_err(|e| e.to_string())?;
    let path = get_key_storage_path(app_handle)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(path, json).map_err(|e| e.to_string())
}

fn load_encrypted_key(app_handle: &AppHandle) -> Result<EncryptedKeyResponse, String> {
    let path = get_key_storage_path(app_handle)?;
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let data: EncryptedKeyResponse = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(data)
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Generates a new Ed25519 public and private key pair.
/// Returns the public and private keys as base64-encoded strings.
#[tauri::command]
async fn generate_keypair() -> Result<KeypairResponse, String> {
    let mut csprng = OsRng;
    let keypair: SigningKey = SigningKey::generate(&mut csprng);

    let public_key_bytes = keypair.verifying_key().to_bytes();
    let secret_key_bytes = keypair.to_bytes();

    let public_key_base64 = general_purpose::STANDARD.encode(&public_key_bytes);
    let private_key_base64 = general_purpose::STANDARD.encode(&secret_key_bytes);

    Ok(KeypairResponse {
        public_key: public_key_base64,
        private_key: private_key_base64,
    })
}

/// Encrypts a private key using AES-256 GCM with a key derived from a password via PBKDF2.
/// Returns the encrypted data, salt, and nonce (IV) as base64-encoded strings.
#[tauri::command]
async fn encrypt_private_key(
    private_key_base64: String,
    password: String,
) -> Result<EncryptedKeyResponse, String> {
    // Decode the private key from base64
    let private_key_bytes = general_purpose::STANDARD
        .decode(&private_key_base64)
        .map_err(|e| format!("Failed to decode private key: {}", e))?;

    // Generate a random salt
    let mut salt = [0u8; 16]; // 128-bit salt
    thread_rng().fill_bytes(&mut salt);

    // Derive a 32-byte (256-bit) AES key from the password and salt using PBKDF2
    const PBKDF2_ITERATIONS: u32 = 100_000;
    let mut key_bytes = [0u8; 32]; // 256-bit key
    pbkdf2_hmac::<Sha256>(
        &password.as_bytes(),
        &salt,
        PBKDF2_ITERATIONS,
        &mut key_bytes,
    );
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Generate a random nonce (Initialization Vector) for AES-GCM
    let mut nonce_bytes = [0u8; 12]; // 96-bit nonce for GCM
    thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt the private key
    let ciphertext = cipher
        .encrypt(nonce, private_key_bytes.as_ref())
        .map_err(|e| format!("Encryption failed: {}", e))?;

    Ok(EncryptedKeyResponse {
        encrypted_data_base64: general_purpose::STANDARD.encode(&ciphertext),
        salt_base64: general_purpose::STANDARD.encode(&salt),
        nonce_base64: general_purpose::STANDARD.encode(&nonce_bytes),
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
        .map_err(|e| format!("Failed to decode encrypted data: {}", e))?;
    let salt_bytes = general_purpose::STANDARD
        .decode(&encrypted_response.salt_base64)
        .map_err(|e| format!("Failed to decode salt: {}", e))?;
    let nonce_bytes = general_purpose::STANDARD
        .decode(&encrypted_response.nonce_base64)
        .map_err(|e| format!("Failed to decode nonce: {}", e))?;

    // Derive the key using the provided password and the stored salt
    const PBKDF2_ITERATIONS: u32 = 100_000;
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
        .map_err(|e| format!("Decryption failed: {}", e))?;

    // Return the decrypted private key as base64
    Ok(general_purpose::STANDARD.encode(&plaintext))
}

#[tauri::command]
async fn register(username: String, password: String, server: String) -> Result<String, String> {
    let keypair = generate_keypair().await?;
    
    // Clone the password before using it in encrypt_private_key
    let _encrypted = encrypt_private_key(keypair.private_key.clone(), password.clone()).await?;

    let payload = serde_json::json!({
        "username": username,
        "server_domain": server,
        "password": password,
        "pubkey": keypair.public_key,
    });

    println!("{}", &payload.to_string());

    // TODO: change to https (http only for debugging rn)
    let url = format!("http://{}/api/v1/register", server);

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let body = res.text().await.map_err(|e| e.to_string())?;
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
            generate_keypair,
            encrypt_private_key,
            decrypt_private_key,
            register,
            save_my_key,
            load_my_key,
            get_build_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}