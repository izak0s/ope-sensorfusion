use chacha20poly1305::Key;
use ope::get_ope;
use rand::Rng;

use crate::data::encrypted_sensor_value::EncryptedSensorValue;
use crate::utils::crypt_utils::encrypt_u64;

#[derive(Debug)]
pub struct SensorValue {
    pub lower_bound: u64,
    pub upper_bound: u64,
}

impl SensorValue {
    // Generate a sensor value
    pub fn generate(faulty: bool) -> SensorValue {
        if faulty {
            return Self::_generate_faulty();
        }

        let mut rnd = rand::thread_rng();

        // Have a constant value
        let value: u64 = 10430;
        // Generate a more precise interval size between 1 and 5000 to prevent cipher distribution attacks
        let interval_size = rnd.gen_range(1..5000);

        SensorValue {
            lower_bound: value - interval_size,
            upper_bound: value + interval_size,
        }
    }

    // Generate a faulty value
    fn _generate_faulty() -> SensorValue {
        let mut rnd = rand::thread_rng();

        // Generate random lower bound
        let lower_bound: u64 = rnd.gen::<u16>() as u64;
        // Generate random upper bound
        let upper_bound: u64 = rnd.gen::<u16>() as u64;

        SensorValue {
            lower_bound,
            upper_bound,
        }
    }

    pub fn encrypt(self, ope_key: [u8; 16], client_key: &Key) -> EncryptedSensorValue {
        let ope = get_ope(&ope_key);
        let encrypted = EncryptedSensorValue {
            lower_bound: ope.encrypt(self.lower_bound).unwrap(),
            upper_bound: ope.encrypt(self.upper_bound).unwrap(),
            enc_lower_bound: encrypt_u64(self.lower_bound, &client_key).unwrap(),
            enc_upper_bound: encrypt_u64(self.upper_bound, &client_key).unwrap(),
        };

        println!(
            "Encrypting generated sensor value {}-{} to {}-{}",
            self.lower_bound, self.upper_bound, encrypted.lower_bound, encrypted.upper_bound
        );

        encrypted
    }
}