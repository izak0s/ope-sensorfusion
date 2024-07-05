use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use chacha20poly1305::{ChaCha20Poly1305, KeyInit};
use chacha20poly1305::aead::Aead;
use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::consts::U32;

fn concat_arrays<T, const A: usize, const B: usize, const C: usize>(a: [T; A], b: [T; B]) -> [T; C] {
    assert_eq!(A + B, C);
    let mut iter = a.into_iter().chain(b);
    std::array::from_fn(|_| iter.next().unwrap())
}

pub fn combine_keys(key: [u8; 12], seed: [u8; 4]) -> [u8; 16] {
    let combined: [u8; 16] = concat_arrays(seed, key);
    combined
}

pub fn encrypt_u64(item: u64, key: &GenericArray<u8, U32>) -> Result<[u8; 24], String> {
    let cipher = ChaCha20Poly1305::new(&key);

    let nonce = vec![5u8; 12];
    let nonce_decoded = GenericArray::from_slice(&nonce);

    let mut buf = Vec::new();
    buf.write_u64::<BigEndian>(item).map_err(|e| format!("Failed to write u64 to buffer: {}", e))?;

    let encrypted = cipher.encrypt(&nonce_decoded, buf.as_ref())
        .map_err(|e| format!("Failed to encrypt u64 value {}: {}", item, e))?;

    let mut encrypted_arr = [0; 24];
    encrypted_arr.copy_from_slice(&encrypted[..]);
    Ok(encrypted_arr)
}

pub fn decrypt_u64(payload: [u8; 24], key: &GenericArray<u8, U32>) -> Result<u64, String> {
    let cipher = ChaCha20Poly1305::new(&key);

    let nonce = vec![5u8; 12];
    let nonce_decoded = GenericArray::from_slice(&nonce);

    if payload.len() != 24 {
        return Err("Invalid payload size, unable to decrypt u64".to_string());
    }

    let decrypted = cipher.decrypt(&nonce_decoded, payload.as_ref())
        .map_err(|e| format!("Failed to decrypt u64 value {:?}: {}", payload, e))?;
    let mut decrypted = Cursor::new(&decrypted);

    let decoded = decrypted.read_u64::<BigEndian>().map_err(|e| format!("Failed to decode u64 {:?}: {}", decrypted, e))?;

    Ok(decoded)
}