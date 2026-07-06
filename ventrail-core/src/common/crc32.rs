//! Small table-free CRC32 implementation for package health checks.

pub fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for byte in bytes {
        crc ^= *byte as u32;
        for _ in 0..8 {
            let mask = 0u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

pub fn crc32_extend(seed: u32, bytes: &[u8]) -> u32 {
    crc32(&[seed.to_le_bytes().as_slice(), bytes].concat())
}
