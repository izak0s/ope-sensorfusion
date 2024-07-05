use std::io::{Cursor, Read};

use chacha20poly1305::Key;

use crate::network::packet::Packet;

pub struct RequestSensorDataPacket {
    pub seed: [u8; 4],
}

impl Packet for RequestSensorDataPacket {
    fn packet_id() -> u8 {
        0x00
    }

    fn encode(&self, buf: &mut Vec<u8>, _key: Option<&Key>) {
        buf.extend_from_slice(&self.seed);
    }

    fn decode(&mut self, data: &mut Cursor<&[u8]>, _key: Option<&Key>) -> Result<(), String> {
        data.read_exact(&mut self.seed).map_err(|e| format!("Failed to read request sensor data packet: {}", e))?;

        Ok(())
    }

    fn new() -> Self {
        RequestSensorDataPacket { seed: [0, 0, 0, 0] }
    }

    fn size() -> usize {
        4
    }
}
