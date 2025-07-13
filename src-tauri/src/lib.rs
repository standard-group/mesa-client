// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

// for encryption
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use pbkdf2::pbkdf2_hmac;
use rand::{rng, RngCore};
use sha2::{Digest, Sha256}; // for hashing passwords (though PBKDF2 is better for KDF) // for generating random salt and nonce

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
    let secret_key_bytes = keypair.to_bytes(); // Use .to_bytes() for SigningKey

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
    rng().fill_bytes(&mut salt);

    // Derive a 32-byte (256-bit) AES key from the password and salt using PBKDF2
    const PBKDF2_ITERATIONS: u32 = 100_000; // Recommended iterations
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
    rng().fill_bytes(&mut nonce_bytes);
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            generate_keypair,
            encrypt_private_key,
            decrypt_private_key
        ]) // Ensure generate_keypair is included
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
