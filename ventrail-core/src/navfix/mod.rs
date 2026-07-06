//! Navfix subsystem.

pub mod packet_decoder;
pub mod range_index;
pub mod merge_plan;
pub mod quality_window;

use crate::common::buffer::ByteBuffer;
use crate::common::status::StatusMessage;

pub fn ingest_primary(payload: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    let st = packet_decoder::ingest(payload, out);
    if !st.is_ok() { return st; }
    range_index::ingest(payload, out)
}

pub fn ingest_secondary(payload: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    let st = merge_plan::ingest(payload, out);
    if !st.is_ok() { return st; }
    quality_window::ingest(payload, out)
}

pub fn ingest_all(payload: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    let st = ingest_primary(payload, out);
    if !st.is_ok() { return st; }
    ingest_secondary(payload, out)
}
