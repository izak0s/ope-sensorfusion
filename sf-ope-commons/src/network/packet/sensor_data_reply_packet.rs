use std::io::{Cursor, Read};
use std::mem::size_of;

use chacha20poly1305::{AeadCore, ChaCha20Poly1305, Key, KeyInit};
use chacha20poly1305::aead::{Aead, OsRng};
use chacha20poly1305::aead::generic_array::GenericArray;
use crate::data::encrypted_sensor_value::EncryptedSensorValue;

use crate::network::packet::Packet;

pub struct SensorDataReplyPacket {
    pub encrypted_value: EncryptedSensorValue,
}

impl Packet for SensorDataReplyPacket {
    fn packet_id() -> u8 {
        0x01
    }


    fn encode(&self, buf: &mut Vec<u8>, key: Option<&Key>) {
        let mut ope_values = Vec::with_capacity(size_of::<EncryptedSensorValue>());
        self.encrypted_value.encode(&mut ope_values);

        let cipher = ChaCha20Poly1305::new(key.unwrap());

        // 96-bits; unique per message
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let encrypted = cipher.encrypt(&nonce, ope_values.as_ref()).unwrap();

        // Encode nonce
        buf.append(&mut nonce.to_vec());
        // Encode encrypted payload
        buf.append(&mut encrypted.to_vec());

    }

    fn decode(&mut self, data: &mut Cursor<&[u8]>, key: Option<&Key>) -> Result<(), String> {
        let mut nonce = vec![0u8; 12];
        data.read_exact(&mut nonce).map_err(|e| format!("Failed to read nonce: {}", e))?;
        let nonce_decoded = GenericArray::from_slice(&nonce);

        let mut encrypted_payload = vec![0u8; 80];
        data.read_exact(&mut encrypted_payload).map_err(|e| format!("Failed to set read encrypted payload: {}", e))?;

        let cipher = ChaCha20Poly1305::new(key.unwrap());
        let decrypted_payload = cipher.decrypt(&nonce_decoded, encrypted_payload.as_ref())
            .map_err(|e| format!("Failed to decrypt encrypted payload (Wrong key?) {}", e))?;

        let mut payload_cursor = Cursor::new(decrypted_payload.as_ref());

        self.encrypted_value = EncryptedSensorValue::decode(&mut payload_cursor)
            .map_err(|e| format!("Failed to decode packet: {}", e))?;

        Ok(())
    }

    fn new() -> Self {
        SensorDataReplyPacket {
            encrypted_value: EncryptedSensorValue {
                lower_bound: 0,
                upper_bound: 0,
                enc_lower_bound: [0u8; 24],
                enc_upper_bound: [0u8; 24]
            },
        }
    }

    fn size() -> usize {
        32 + 12 + 24 + 24
    }
}
