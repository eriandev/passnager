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

#[cfg(test)]
mod tests {
    use super::*;

    /// Cheap enough to run many times. It changes how long a derivation takes, not
    /// what it returns, so the assertions below mean the same thing they would
    /// with production parameters.
    fn cheap() -> KdfParams {
        KdfParams {
            memory: 64,
            ..Default::default()
        }
    }

    /// `notes.encrypted_content CHECK (length BETWEEN 17 AND 1040)` is built on
    /// this: AES-GCM appends a 16 byte tag, so the stored value is the plaintext
    /// plus 16. A dependency bump that changed the tag size, or dropped the
    /// append, would make the constraint reject or wrongly admit bodies, and the
    /// failure would show up in a vault rather than in a test.
    const TAG_LEN: usize = 16;

    #[test]
    fn ciphertext_is_the_plaintext_plus_a_16_byte_tag() {
        let dek = generate_dek();
        for len in [0usize, 1, 16, 255, 256, 1024] {
            let plaintext = vec![7u8; len];
            let (ciphertext, _) = encrypt_password(&plaintext, &dek).unwrap();
            assert_eq!(
                ciphertext.len(),
                plaintext.len() + TAG_LEN,
                "a {len}-byte body should store as {} bytes",
                plaintext.len() + TAG_LEN
            );
        }
    }

    #[test]
    fn the_worst_case_body_exactly_meets_the_schema_bound() {
        // 256 characters at the 4 UTF-8 bytes a single character can take, plus
        // the tag. This is the value the constraint was written from.
        let dek = generate_dek();
        let worst = "👍".repeat(256);
        assert_eq!(worst.chars().count(), 256);
        let (ciphertext, _) = encrypt_password(worst.as_bytes(), &dek).unwrap();
        assert_eq!(ciphertext.len(), 1040);
    }

    #[test]
    fn encryption_round_trips_including_empty_and_unicode() {
        let dek = generate_dek();
        for case in [
            "",
            "hola",
            "contraseña con ñ y á",
            "日本語のテキスト",
            "👍🏽👎🏿",
        ] {
            let (ciphertext, nonce) = encrypt_password(case.as_bytes(), &dek).unwrap();
            let recovered = decrypt_password(&ciphertext, &nonce, &dek).unwrap();
            assert_eq!(recovered, case.as_bytes());
        }
    }

    /// Reusing a nonce with the same key is the one mistake that leaks the XOR of
    /// two plaintexts and forfeits authentication, so the nonce has to be fresh
    /// every time.
    #[test]
    fn encrypting_twice_produces_different_ciphertext() {
        let dek = generate_dek();
        let (first, first_nonce) = encrypt_password(b"same", &dek).unwrap();
        let (second, second_nonce) = encrypt_password(b"same", &dek).unwrap();
        assert_ne!(first, second, "identical ciphertext means a reused nonce");
        assert_ne!(first_nonce, second_nonce);
    }

    #[test]
    fn generated_nonces_and_salts_are_the_right_size_and_unique() {
        let a = generate_nonce();
        let b = generate_nonce();
        assert_eq!(a.len(), 12, "AES-GCM wants a 96-bit nonce");
        assert_ne!(a, b);

        let s = generate_salt();
        assert_eq!(s.len(), 16);
        assert_ne!(s, generate_salt());
    }

    #[test]
    fn the_wrong_dek_cannot_decrypt() {
        let dek = generate_dek();
        let other = generate_dek();
        let (ciphertext, nonce) = encrypt_password(b"secret", &dek).unwrap();
        assert!(decrypt_password(&ciphertext, &nonce, &other).is_err());
    }

    #[test]
    fn a_tampered_ciphertext_or_nonce_is_rejected() {
        let dek = generate_dek();
        let (mut ciphertext, mut nonce) = encrypt_password(b"secret value", &dek).unwrap();

        let mut forged = ciphertext.clone();
        forged[0] ^= 0x01;
        assert!(
            decrypt_password(&forged, &nonce, &dek).is_err(),
            "a flipped bit must fail authentication"
        );

        nonce[0] ^= 0x01;
        assert!(decrypt_password(&ciphertext, &nonce, &dek).is_err());

        ciphertext[0] ^= 0x01;
        assert!(decrypt_password(&ciphertext, &nonce, &dek).is_err());
    }

    #[test]
    fn truncating_the_ciphertext_is_rejected() {
        let dek = generate_dek();
        let (ciphertext, nonce) = encrypt_password(b"secret value", &dek).unwrap();
        assert!(decrypt_password(&ciphertext[..ciphertext.len() - 1], &nonce, &dek).is_err());
    }

    #[test]
    fn wrap_and_unwrap_round_trip_the_dek() {
        let dek = generate_dek();
        let salt = generate_salt();
        let kek = derive_kek_with_params(b"correct horse", &salt, cheap()).unwrap();

        let (wrapped, nonce) = wrap_dek(&dek, &kek).unwrap();
        assert_eq!(wrapped.len(), 32 + TAG_LEN);
        assert_eq!(unwrap_dek(&wrapped, &nonce, &kek).unwrap(), dek);
    }

    #[test]
    fn the_wrong_master_password_is_reported_as_such() {
        let dek = generate_dek();
        let salt = generate_salt();
        let kek = derive_kek_with_params(b"right", &salt, cheap()).unwrap();
        let (wrapped, nonce) = wrap_dek(&dek, &kek).unwrap();

        let wrong = derive_kek_with_params(b"wrong", &salt, cheap()).unwrap();
        let err = unwrap_dek(&wrapped, &nonce, &wrong).unwrap_err();
        assert_eq!(err, "Incorrect master password");
    }

    #[test]
    fn derivation_is_deterministic_and_sensitive_to_both_inputs() {
        let salt = generate_salt();
        let other_salt = generate_salt();

        let base = derive_kek_with_params(b"password", &salt, cheap()).unwrap();
        assert_eq!(
            base,
            derive_kek_with_params(b"password", &salt, cheap()).unwrap(),
            "same inputs must give the same key"
        );
        assert_ne!(
            base,
            derive_kek_with_params(b"passwore", &salt, cheap()).unwrap(),
            "a different password must give a different key"
        );
        assert_ne!(
            base,
            derive_kek_with_params(b"password", &other_salt, cheap()).unwrap(),
            "a different salt must give a different key"
        );
    }

    /// The cost parameters exist so tests do not have to spend 64MB and three
    /// iterations per assertion. That is only safe if they cannot change the
    /// result, so this compares the injectable path against the production one.
    /// Expensive by design: it is the one test that pays for real Argon2.
    #[test]
    fn injectable_parameters_do_not_change_the_derived_key() {
        let salt = generate_salt();
        let production = derive_kek(b"password", &salt).unwrap();
        let injected = derive_kek_with_params(b"password", &salt, KdfParams::default()).unwrap();
        assert_eq!(production, injected);
    }
}
