use std::io::{Cursor, Read};
use std::mem::size_of;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::consts::U32;

use crate::data::sensor_value::SensorValue;
use crate::utils::crypt_utils::decrypt_u64;

#[derive(PartialEq, Debug)]
pub struct EncryptedSensorValue {
    pub lower_bound: u64,
    pub upper_bound: u64,
    pub enc_lower_bound: [u8; 24],
    pub enc_upper_bound: [u8; 24],
}

impl EncryptedSensorValue {
    pub(crate) fn encode(&self, buf: &mut Vec<u8>) {
        buf.write_u64::<BigEndian>(self.lower_bound)
            .expect("Failed to encode lower bound of packet");
        buf.write_u64::<BigEndian>(self.upper_bound)
            .expect("Failed to encode upper bound of packet");
        buf.append(&mut self.enc_lower_bound.to_vec());
        buf.append(&mut self.enc_upper_bound.to_vec());
    }

    pub(crate) fn decode(data: &mut Cursor<&[u8]>) -> Result<EncryptedSensorValue, String> {
        let readable_bytes = data.get_ref().len() - data.position() as usize;

        if readable_bytes != size_of::<EncryptedSensorValue>() {
            return Err(format!("Size of the packet does not match the expected size for EncryptedSensorValue: {}, expected {}",
                               readable_bytes, size_of::<EncryptedSensorValue>()));
        }

        // Read OPE key
        let lower_bound = data.read_u64::<BigEndian>().unwrap();
        let upper_bound = data.read_u64::<BigEndian>().unwrap();

        // Read symmetric encrypted values
        let mut enc_lower_bound = [0u8; 24];
        data.read_exact(&mut enc_lower_bound).map_err(|e| format!("Failed to read encrypted lower bound: {}", e))?;
        let mut enc_upper_bound = [0u8; 24];
        data.read_exact(&mut enc_upper_bound).map_err(|e| format!("Failed to read encrypted upper bound: {}", e))?;

        let sensor_value = EncryptedSensorValue {
            lower_bound,
            upper_bound,
            enc_lower_bound,
            enc_upper_bound,
        };

        Ok(sensor_value)
    }

    pub fn decrypt(&self, key: &GenericArray<u8, U32>) -> SensorValue {
        let lower_bound = decrypt_u64(self.enc_lower_bound, &key).expect("Failed to decrypt lower bound");
        let upper_bound = decrypt_u64(self.enc_upper_bound, &key).expect("Failed to decrypt upper bound");

        let decrypted = SensorValue {
            lower_bound,
            upper_bound,
        };

        println!(
            "Decrypting sensor value {}-{} to {}-{}",
            self.lower_bound, self.upper_bound, decrypted.lower_bound, decrypted.upper_bound
        );

        decrypted
    }
}