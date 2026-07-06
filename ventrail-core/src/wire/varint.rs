//! Varint helpers used by the HVWS framing layer.

pub use crate::common::wire_reader::{read_i32_le, read_u16_le, read_u32_le, read_u64_le};

pub fn decode_varint(data: &[u8], off: &mut usize) -> Option<u64> {
    let mut value = 0u64;
    let mut shift = 0u32;
    while *off < data.len() {
        let byte = data[*off];
        *off += 1;
        value |= ((byte & 0x7f) as u64) << shift;
        if byte & 0x80 == 0 {
            return Some(value);
        }
        shift += 7;
        if shift > 63 {
            return None;
        }
    }
    None
}

pub fn encode_varint(mut value: u64, out: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 { byte |= 0x80; }
        out.push(byte);
        if value == 0 { break; }
    }
}
