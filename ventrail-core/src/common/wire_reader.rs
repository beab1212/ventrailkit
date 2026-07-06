//! Little-endian primitive readers with truncation-tolerant fallbacks.

pub fn read_u16_le(data: &[u8], off: usize) -> u16 {
    if off + 2 > data.len() { return 0; }
    u16::from_le_bytes([data[off], data[off + 1]])
}

pub fn read_u32_le(data: &[u8], off: usize) -> u32 {
    if off + 4 > data.len() { return 0; }
    u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]])
}

pub fn read_i32_le(data: &[u8], off: usize) -> i32 { read_u32_le(data, off) as i32 }

pub fn read_u64_le(data: &[u8], off: usize) -> u64 {
    if off + 8 > data.len() { return 0; }
    u64::from_le_bytes([
        data[off], data[off + 1], data[off + 2], data[off + 3],
        data[off + 4], data[off + 5], data[off + 6], data[off + 7],
    ])
}

pub fn read_f32_le(data: &[u8], off: usize) -> f32 { f32::from_bits(read_u32_le(data, off)) }
pub fn read_f64_le(data: &[u8], off: usize) -> f64 { f64::from_bits(read_u64_le(data, off)) }

pub fn read_slice(data: &[u8], off: usize, len: usize) -> &[u8] {
    if off >= data.len() { return &[]; }
    let end = off.saturating_add(len).min(data.len());
    &data[off..end]
}
