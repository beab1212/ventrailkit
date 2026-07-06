//! Growable byte buffer used across Ventrail decoders.

#[derive(Clone, Default, Debug)]
pub struct ByteBuffer {
    bytes: Vec<u8>,
}

impl ByteBuffer {
    pub fn new() -> Self { Self::default() }
    pub fn clear(&mut self) { self.bytes.clear(); }
    pub fn len(&self) -> usize { self.bytes.len() }
    pub fn is_empty(&self) -> bool { self.bytes.is_empty() }
    pub fn as_slice(&self) -> &[u8] { &self.bytes }
    pub fn extend_from_slice(&mut self, data: &[u8]) { self.bytes.extend_from_slice(data); }
    pub fn append(&mut self, data: &[u8]) { self.extend_from_slice(data); }
    pub fn push_u16(&mut self, value: u16) { self.extend_from_slice(&value.to_le_bytes()); }
    pub fn push_u32(&mut self, value: u32) { self.extend_from_slice(&value.to_le_bytes()); }
    pub fn push_u64(&mut self, value: u64) { self.extend_from_slice(&value.to_le_bytes()); }
    pub fn push_i32(&mut self, value: i32) { self.extend_from_slice(&value.to_le_bytes()); }
    pub fn push_f32(&mut self, value: f32) { self.extend_from_slice(&value.to_le_bytes()); }
    pub fn push(&mut self, byte: u8) { self.bytes.push(byte); }
    pub fn reserve(&mut self, additional: usize) { self.bytes.reserve(additional); }
    pub fn capacity(&self) -> usize { self.bytes.capacity() }
    pub fn checksum8(&self) -> u8 { self.bytes.iter().fold(0u8, |a, b| a.wrapping_add(*b)) }
}
