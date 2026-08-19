use argon2::Argon2;
use chacha20poly1305::{
    ChaCha20Poly1305,
    aead::{
        KeyInit,
        stream::{DecryptorLE31, EncryptorLE31},
    },
};
use rand::Rng;
use std::io::{self, IsTerminal, Write};

use crate::error::{CfxError, Result};
use crate::format::{NONCE_LEN, SALT_LEN};

pub type StreamEncryptor = EncryptorLE31<ChaCha20Poly1305>;
pub type StreamDecryptor = DecryptorLE31<ChaCha20Poly1305>;

pub fn get_password(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush().map_err(CfxError::Io)?;
    if !io::stdin().is_terminal() {
        let mut pass = String::new();
        io::stdin().read_line(&mut pass).map_err(CfxError::Io)?;
        return Ok(pass.trim_end_matches(&['\r', '\n'][..]).to_string());
    }
    rpassword::read_password().map_err(|e| CfxError::PasswordPrompt(e.to_string()))
}

pub fn derive_key(password: &str, salt_bytes: &[u8]) -> Result<[u8; 32]> {
    let params = argon2::ParamsBuilder::default().build().unwrap();
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt_bytes, &mut key)
        .map_err(|e| CfxError::Crypto(format!("Key derivation failed: {}", e)))?;
    Ok(key)
}

pub fn generate_salt() -> [u8; SALT_LEN] {
    let mut salt = [0u8; SALT_LEN];
    rand::rng().fill_bytes(&mut salt);
    salt
}

pub fn generate_nonce() -> [u8; NONCE_LEN] {
    let mut nonce = [0u8; NONCE_LEN];
    rand::rng().fill_bytes(&mut nonce);
    nonce
}

pub fn create_encryptor(key: &[u8; 32], nonce: &[u8; NONCE_LEN]) -> StreamEncryptor {
    let aead = ChaCha20Poly1305::new(key.into());
    StreamEncryptor::from_aead(aead, nonce.as_ref().into())
}

pub fn create_decryptor(key: &[u8; 32], nonce: &[u8; NONCE_LEN]) -> StreamDecryptor {
    let aead = ChaCha20Poly1305::new(key.into());
    StreamDecryptor::from_aead(aead, nonce.as_ref().into())
}
