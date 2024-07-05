use std::env;

use chacha20poly1305::Key;

use sf_ope_commons::utils::env_parser::parse_env_key;

use crate::collector_server::CollectorServer;
use crate::sensor_collector::SensorCollector;

pub mod fault_tolerant_handler;
pub mod sensor_collector;
mod collector_server;

fn main() {
    // Read the addresses from an environment variable
    let addresses = env::var("SENSOR_ADDRESSES").expect("SENSOR_ADDRESSES not set");
    let address_list: Vec<&str> = addresses.split(',').collect();

    // Read the addresses from an environment variable
    let keys = env::var("SENSOR_KEYS").expect("SENSOR_KEYS not set");
    let key_list: Vec<&str> = keys.split(',').collect();
    let mut parsed_key_list: Vec<Key> = Vec::new();
    for key_input in key_list {
        // Check key length
        if key_input.len() != 32 {
            panic!("Key length for should be exactly 32 bytes, {} given", key_input.len());
        }
        let key_casted: [u8; 32] = key_input.as_bytes().try_into().unwrap();
        let key_parsed = Key::from(key_casted);
        parsed_key_list.push(key_parsed);
    }

    // Ensure that the number of addresses and keys match
    if parsed_key_list.len() != address_list.len() {
        panic!("{} addresses found and {} keys. This number should match.", address_list.len(), parsed_key_list.len())
    }

    println!("Registered {} sensors with addresses: {:?}", address_list.len(), address_list);

    // Parse client key
    let client_key = parse_env_key("CLIENT_KEY");

    let collector = SensorCollector::new(address_list, parsed_key_list);
    CollectorServer { port: 12999 }.start_server(&collector, &client_key).expect("Failed to start collector server");
}
