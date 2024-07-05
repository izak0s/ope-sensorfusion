use std::io::Cursor;

use chacha20poly1305::Key;

pub mod request_sensor_data_packet;
pub mod sensor_data_reply_packet;

pub trait Packet {
    fn packet_id() -> u8;

    fn encode(&self, buf: &mut Vec<u8>, key: Option<&Key>);

    fn decode(&mut self, data: &mut Cursor<&[u8]>, key: Option<&Key>) -> Result<(), String>;

    fn new() -> Self;

    fn size() -> usize;
}
