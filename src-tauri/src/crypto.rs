use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{Algorithm, Argon2, Version};
use rand::RngCore;

/// Argon2id cost parameters.
///
/// The defaults are what production vaults are protected with. Tests build a
/// cheaper variant with `KdfParams { memory: 64, ..Default::default() }`: that
/// changes how long derivation takes, not what it returns, so test assertions
/// about ciphertext stay meaningful.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KdfParams {
    pub memory: u32,
    pub iterations: u32,
    pub parallelism: u32,
    pub output_len: usize,
}

impl Default for KdfParams {
    fn default() -> Self {
        Self {
            memory: 65536,
            iterations: 3,
            parallelism: 4,
            output_len: 32,
        }
    }
}

pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

pub fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}

pub fn generate_dek() -> [u8; 32] {
    let mut dek = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut dek);
    dek
}

pub fn derive_kek(password: &[u8], salt: &[u8]) -> Result<[u8; 32], String> {
    derive_kek_with_params(password, salt, KdfParams::default())
}

pub fn derive_kek_with_params(
    password: &[u8],
    salt: &[u8],
    kdf: KdfParams,
) -> Result<[u8; 32], String> {
    let params = argon2::Params::new(
        kdf.memory,
        kdf.iterations,
        kdf.parallelism,
        Some(kdf.output_len),
    )
    .map_err(|e| e.to_string())?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut kek = [0u8; 32];
    argon2
        .hash_password_into(password, salt, &mut kek)
        .map_err(|e| e.to_string())?;
    Ok(kek)
}

pub fn wrap_dek(dek: &[u8; 32], kek: &[u8; 32]) -> Result<(Vec<u8>, Vec<u8>), String> {
    let cipher = Aes256Gcm::new_from_slice(kek).map_err(|e| e.to_string())?;
    let nonce_bytes = generate_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, dek.as_ref())
        .map_err(|e| e.to_string())?;

    Ok((ciphertext, nonce_bytes.to_vec()))
}

pub fn unwrap_dek(
    wrapped_dek: &[u8],
    dek_nonce: &[u8],
    kek: &[u8; 32],
) -> Result<[u8; 32], String> {
    let cipher = Aes256Gcm::new_from_slice(kek).map_err(|e| e.to_string())?;
    let nonce = Nonce::from_slice(dek_nonce);

    let plaintext = cipher
        .decrypt(nonce, wrapped_dek)
        .map_err(|_| "Incorrect master password".to_string())?;

    if plaintext.len() != 32 {
        return Err("Invalid DEK".to_string());
    }

    let mut dek = [0u8; 32];
    dek.copy_from_slice(&plaintext);
    Ok(dek)
}

pub fn encrypt_password(plaintext: &[u8], dek: &[u8; 32]) -> Result<(Vec<u8>, Vec<u8>), String> {
    let cipher = Aes256Gcm::new_from_slice(dek).map_err(|e| e.to_string())?;
    let nonce_bytes = generate_nonce();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| e.to_string())?;

    Ok((ciphertext, nonce_bytes.to_vec()))
}

pub fn decrypt_password(encrypted: &[u8], nonce: &[u8], dek: &[u8; 32]) -> Result<Vec<u8>, String> {
    let cipher = Aes256Gcm::new_from_slice(dek).map_err(|e| e.to_string())?;
    let nonce = Nonce::from_slice(nonce);

    let plaintext = cipher
        .decrypt(nonce, encrypted)
        .map_err(|e| e.to_string())?;

    Ok(plaintext)
}
