use std::io::Cursor;
use std::mem::size_of;
use std::net::SocketAddr;

use byteorder::ReadBytesExt;
use chacha20poly1305::Key;

use crate::network::packet::Packet;
use crate::network::packet::request_sensor_data_packet::RequestSensorDataPacket;
use crate::network::packet::sensor_data_reply_packet::SensorDataReplyPacket;

pub enum Packets {
    RequestSensorData(RequestSensorDataPacket),
    SensorDataReply(SensorDataReplyPacket),
}

pub struct PacketManager {}

impl PacketManager {

    // Transform packet to byte-level representation
    pub fn construct_packet<T: Packet>(packet: T, key: Option<&Key>) -> Vec<u8> {
        let size = size_of::<T>();
        let mut buf = Vec::with_capacity(size + 1);

        buf.push(T::packet_id());
        packet.encode(&mut buf, key);

        return buf;
    }

    // Decode packet from byte-level representation
    fn decode_packet<T: Packet>(rdr: &mut Cursor<&[u8]>, key: Option<&Key>) -> Result<T, String> {
        let expected_size = T::size();
        let readable_bytes = rdr.get_ref().len() - rdr.position() as usize;

        // Check whether the buffer size matches the expected packet size
        if readable_bytes != expected_size {
            return Err(format!("Packet {} expects {} readable bytes, actually {}", T::packet_id(), expected_size, readable_bytes));
        }

        // Create new packet and decode
        let mut packet = T::new();
        packet.decode(rdr, key).map_err(|e| format!("Failed to decode packet: {}", e))?;

        return Ok(packet);
    }

    // Handle incoming packets
    pub fn handle_packet(rdr: &mut Cursor<&[u8]>, addr: SocketAddr, key: Option<&Key>) -> Result<Packets, String> {
        let packet_id = rdr.read_u8().map_err(|e| format!("Failed to decode packet id: {}", e))?;

        println!("Received packet with id {} from {}", packet_id, addr);

        return match packet_id {
            0x00 => {
                let packet = Self::decode_packet::<RequestSensorDataPacket>(rdr, key)?;
                Ok(Packets::RequestSensorData(packet))
            }
            0x01 => {
                let packet = Self::decode_packet::<SensorDataReplyPacket>(rdr, key)?;
                Ok(Packets::SensorDataReply(packet))
            }
            _ => {
                Err(format!(
                    "Received an unhandled packet {} from {}",
                    packet_id, addr
                ))
            }
        };
    }
}
