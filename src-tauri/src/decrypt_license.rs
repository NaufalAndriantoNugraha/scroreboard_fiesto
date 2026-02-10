use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose, Engine as _};

pub fn get_date_from_hash(given_token: &str, information_number: &str) -> String {
    let raw_bytes = match general_purpose::STANDARD.decode(given_token) {
        Ok(bytes) => bytes,
        Err(_) => return "Invalid".to_string(),
    };

    if raw_bytes.len() < 28 {
        return "Invalid".to_string();
    }

    let iv = &raw_bytes[..12];
    let tag = &raw_bytes[12..28];
    let ciphertext = &raw_bytes[28..];

    let mut complete_ciphertext = Vec::new();
    complete_ciphertext.extend_from_slice(ciphertext);
    complete_ciphertext.extend_from_slice(tag);

    let key_bytes = information_number.as_bytes();
    if key_bytes.len() != 32 {
        return "Invalid".to_string();
    }

    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(iv);

    match cipher.decrypt(nonce, complete_ciphertext.as_ref()) {
        Ok(plaintext) => String::from_utf8(plaintext).unwrap_or_else(|_| "Invalid".to_string()),
        Err(_) => "Invalid".to_string(),
    }
}

pub fn get_original_hash(token: &str) -> String {
    if token.len() != 72 {
        return "Invalid".to_string();
    }

    token[2..66].to_string()
}
