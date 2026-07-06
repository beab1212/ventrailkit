//! Calibration quality window processing primitives.

use crate::common::buffer::ByteBuffer;
use crate::common::crc32::crc32;
use crate::common::status::{Status, StatusMessage};
use crate::common::wire_reader::{read_i32_le, read_u16_le, read_u32_le, read_u64_le};

const CHANNEL_BIAS: u32 = 1694;
const WINDOW_LIMIT: usize = 32;

#[derive(Clone, Debug, Default)]
pub struct CalibrationQualityWindowRecord {
    pub station: u32,
    pub channel: u16,
    pub epoch: u64,
    pub reading: i32,
    pub quality: u8,
}

impl CalibrationQualityWindowRecord {
    pub fn from_wire(payload: &[u8], off: usize) -> Option<Self> {
        if off + 19 > payload.len() {
            return None;
        }
        Some(Self {
            station: read_u32_le(payload, off),
            channel: read_u16_le(payload, off + 4),
            epoch: read_u64_le(payload, off + 6),
            reading: read_i32_le(payload, off + 14),
            quality: payload[off + 18],
        })
    }

    pub fn weighted_score(&self) -> i64 {
        let signed = self.reading as i64;
        let channel = self.channel as i64 + CHANNEL_BIAS as i64;
        signed.saturating_mul(channel).saturating_add(self.quality as i64)
    }

    pub fn emit(&self, out: &mut ByteBuffer) {
        out.push_u32(self.station);
        out.push_u16(self.channel);
        out.push_u64(self.epoch);
        out.push_i32(self.reading);
        out.push(self.quality);
    }
}

#[derive(Clone, Debug, Default)]
pub struct ChannelAccumulator {
    sum: i64,
    min: i32,
    max: i32,
    count: u32,
    checksum: u32,
}

impl ChannelAccumulator {
    pub fn push(&mut self, rec: &CalibrationQualityWindowRecord) {
        if self.count == 0 {
            self.min = rec.reading;
            self.max = rec.reading;
        } else {
            self.min = self.min.min(rec.reading);
            self.max = self.max.max(rec.reading);
        }
        self.sum = self.sum.saturating_add(rec.weighted_score());
        self.count = self.count.saturating_add(1);
        self.checksum = self.checksum.rotate_left(5) ^ rec.station ^ rec.channel as u32;
    }

    pub fn mean(&self) -> i32 {
        if self.count == 0 { 0 } else { (self.sum / self.count as i64) as i32 }
    }

    pub fn emit(&self, out: &mut ByteBuffer) {
        out.push_i32(self.mean());
        out.push_i32(self.min);
        out.push_i32(self.max);
        out.push_u32(self.count);
        out.push_u32(self.checksum);
    }
}

pub fn fold_calibration_quality_window(payload: &[u8]) -> ChannelAccumulator {
    let mut acc = ChannelAccumulator::default();
    let declared = read_u16_le(payload, 0) as usize;
    let mut off = 2usize;
    for _ in 0..declared.min(WINDOW_LIMIT) {
        if let Some(rec) = CalibrationQualityWindowRecord::from_wire(payload, off) {
            acc.push(&rec);
            off += 19;
        } else {
            break;
        }
    }
    acc
}

pub fn analyze_calibration_quality_window(payload: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    if payload.len() < 2 {
        return StatusMessage::fail(Status::Truncated, "calibration quality_window");
    }
    let acc = fold_calibration_quality_window(payload);
    acc.emit(out);
    let checksum = crc32(payload).wrapping_add(CHANNEL_BIAS);
    out.push_u32(checksum);
    StatusMessage::ok()
}

pub fn route_calibration_quality_window(payload: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    let status = analyze_calibration_quality_window(payload, out);
    if !status.is_ok() {
        return status;
    }
    let stride = payload.get(1).copied().unwrap_or(1).max(1) as usize;
    let mut selected = 0u32;
    for (i, byte) in payload.iter().enumerate() {
        if i % stride == 0 && (*byte as u32).wrapping_add(CHANNEL_BIAS) & 3 == 0 {
            selected = selected.wrapping_add(1);
        }
    }
    out.push_u32(selected);
    StatusMessage::ok()
}

pub fn ingest(payload: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    route_calibration_quality_window(payload, out)
}

pub fn ingest_calibration_quality_window(payload: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    ingest(payload, out)
}

pub fn classify_quality(payload: &[u8]) -> u8 {
    let mut band = 0u8;
    for byte in payload.iter().take(WINDOW_LIMIT) {
        band = band.rotate_left(1) ^ byte.wrapping_add((CHANNEL_BIAS & 0xff) as u8);
    }
    band
}

pub fn project_station_epoch(payload: &[u8]) -> (u32, u64) {
    let station = read_u32_le(payload, 2).wrapping_add(CHANNEL_BIAS);
    let epoch = read_u64_le(payload, 6).wrapping_add(CHANNEL_BIAS as u64);
    (station, epoch)
}
