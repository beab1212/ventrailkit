//! HVBK bundle validation for expedition archives.

use crate::common::buffer::ByteBuffer;
use crate::common::crc32::crc32;
use crate::common::status::{Status, StatusMessage};
use crate::common::wire_reader::{read_u16_le, read_u32_le};

pub const HVBK_MAGIC: u32 = 0x4B42_5648; // 'HVBK' little-endian

#[derive(Clone, Debug)]
pub struct BundleSection {
    pub tag: u16,
    pub offset: u32,
    pub length: u32,
}

pub fn parse_table(data: &[u8]) -> Result<Vec<BundleSection>, StatusMessage> {
    if data.len() < 12 {
        return Err(StatusMessage::fail(Status::Truncated, "hvbk header"));
    }
    let magic = read_u32_le(data, 0);
    if magic != HVBK_MAGIC {
        return Err(StatusMessage::fail(Status::InvalidArgument, "hvbk magic"));
    }
    let count = read_u16_le(data, 6) as usize;
    let mut off = 12usize;
    let mut sections = Vec::new();
    for _ in 0..count.min(128) {
        if off + 10 > data.len() {
            return Err(StatusMessage::fail(Status::Truncated, "hvbk section table"));
        }
        sections.push(BundleSection {
            tag: read_u16_le(data, off),
            offset: read_u32_le(data, off + 2),
            length: read_u32_le(data, off + 6),
        });
        off += 10;
    }
    Ok(sections)
}

pub fn validate_package(data: &[u8]) -> StatusMessage {
    let sections = match parse_table(data) {
        Ok(s) => s,
        Err(e) => return e,
    };
    for sec in sections {
        let start = sec.offset as usize;
        let end = start.saturating_add(sec.length as usize);
        if start > data.len() || end > data.len() {
            return StatusMessage::fail(Status::OutOfRange, "hvbk section extent");
        }
    }
    StatusMessage::ok()
}

pub fn ingest_container_validator(data: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    let sections = match parse_table(data) {
        Ok(s) => s,
        Err(e) => return e,
    };
    out.push_u32(sections.len() as u32);
    for sec in sections {
        let start = sec.offset as usize;
        let end = start.saturating_add(sec.length as usize).min(data.len());
        let checksum = if start < end { crc32(&data[start..end]) } else { 0 };
        out.push_u16(sec.tag);
        out.push_u32(sec.length);
        out.push_u32(checksum);
    }
    StatusMessage::ok()
}
