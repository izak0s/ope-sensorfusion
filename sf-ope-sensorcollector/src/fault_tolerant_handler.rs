use std::collections::HashMap;
use sf_ope_commons::data::encrypted_sensor_value::EncryptedSensorValue;

pub struct FaultTolerantHandler {
    pub(crate) results: Vec<EncryptedSensorValue>,
}

impl FaultTolerantHandler {
    // Calculate optimal sensor value
    pub(crate) fn calculate_marzullo(&self) -> Option<EncryptedSensorValue> {
        let (mut mapped, cache) = self._populate_map();

        // Sort by u64 value
        mapped.sort_by_key(|&(key, _)| key);
        println!("Sorted mapped sensors {:?}", mapped);

        let mut best = 0;
        let mut cnt = 0;
        let mut best_value: Option<EncryptedSensorValue> = None;

        for (i, (number, mode)) in mapped.iter().enumerate() {
            // Decrease current count with the type
            cnt -= mode;

            // Overwrite the best interval when cnt is more than the current best
            if cnt > best {
                best = cnt;
                best_value = Some(EncryptedSensorValue {
                    lower_bound: *number,
                    upper_bound: mapped[i + 1].0,
                    enc_lower_bound: [0; 24],
                    enc_upper_bound: [0; 24],
                })
            }
        }

        // If there is a best_value present, map encrypted value to the corresponding OPE encrypted value
        if best_value.is_some() {
            let mut unwrapped = best_value.unwrap();
            unwrapped.enc_lower_bound = *cache.get(&unwrapped.lower_bound).unwrap();
            unwrapped.enc_upper_bound = *cache.get(&unwrapped.upper_bound).unwrap();
            return Some(unwrapped);
        }

        best_value
    }

    // Convert intervals to a (value, type (-1/+1)) representation and cache the encrypted value
    fn _populate_map(&self) -> (Vec<(u64, i8)>, HashMap<u64, [u8; 24]>) {
        let mut vec: Vec<(u64, i8)> = Vec::new();
        let mut mapping: HashMap<u64, [u8; 24]> = HashMap::new();

        for sensor in &self.results {
            mapping.insert(sensor.lower_bound, sensor.enc_lower_bound);
            mapping.insert(sensor.upper_bound, sensor.enc_upper_bound);

            // Ensure that the bounds are correctly sorted (lower should really be the lower bound)
            if sensor.upper_bound > sensor.lower_bound {
                vec.push((sensor.lower_bound, -1));
                vec.push((sensor.upper_bound, 1));
            } else {
                vec.push((sensor.lower_bound, 1));
                vec.push((sensor.upper_bound, -1));
            }
        }

        (vec, mapping)
    }
}
