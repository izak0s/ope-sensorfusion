use std::env;

use chacha20poly1305::Key;

pub fn parse_env_key(key: &str) -> Key {
    let key_input = match env::var(key) {
        Ok(item) => item.into_bytes(),
        Err(_) => Vec::with_capacity(32),
    };

    // Check key length
    if key_input.len() != 32 {
        panic!("Key length for {} should be exactly {} bytes, {} given", key, 32, key_input.len());
    }
    let key_casted: [u8; 32] = key_input.try_into().unwrap();
    Key::from(key_casted)
}
