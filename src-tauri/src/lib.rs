// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use ed25519_dalek::{SigningKey, VerifyingKey}; // Corrected: Use SigningKey and VerifyingKey
use rand_core::OsRng; // Corrected: Import OsRng directly from rand_core
use base64::{engine::general_purpose, Engine as _};
use serde::{Serialize, Deserialize}; // Ensure Serialize and Deserialize are used if not already

// Define a struct to hold the keypair information to be returned to the frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct KeypairResponse {
    pub public_key: String,
    pub private_key: String, // Note: In a real app, securely store this server-side or encrypt it before sending to client storage.
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Generates a new Ed25519 public and private key pair.
/// Returns the public and private keys as base64-encoded strings.
#[tauri::command]
async fn generate_keypair() -> Result<KeypairResponse, String> {
    // `OsRng` must be mutable for `SigningKey::generate`
    let mut csprng = OsRng; 
    let keypair: SigningKey = SigningKey::generate(&mut csprng); // This requires rand_core::CryptoRngCore

    let public_key_bytes = keypair.verifying_key().to_bytes();
    let secret_key_bytes = keypair.to_bytes(); // Use .to_bytes() for SigningKey

    let public_key_base64 = general_purpose::STANDARD.encode(&public_key_bytes);
    let private_key_base64 = general_purpose::STANDARD.encode(&secret_key_bytes);

    Ok(KeypairResponse {
        public_key: public_key_base64,
        private_key: private_key_base64,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, generate_keypair]) // Ensure generate_keypair is included
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}