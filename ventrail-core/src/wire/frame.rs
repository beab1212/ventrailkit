//! HVWS session frame constants.

pub const HVWS_MAGIC: u32 = 0x5357_5648; // 'HVWS' little-endian
pub const HVWS_VERSION: u16 = 1;

#[derive(Clone, Debug)]
pub struct WireSection<'a> {
    pub tag: u8,
    pub flags: u8,
    pub payload: &'a [u8],
}

#[derive(Clone, Debug, Default)]
pub struct WireSession {
    pub sections_seen: u32,
    pub output: crate::common::buffer::ByteBuffer,
}
