//! Wire consumers for extent batches, phase windows, and cached views.

use crate::common::buffer::ByteBuffer;
use crate::common::status::{Status, StatusMessage};
use crate::risk::budget::{
    risk_rebuild_slot, risk_stage_slot, validated_acoustic_window, validated_actuator_window,
    validated_catalog_budget, validated_chemical_window, validated_crust_budget,
    validated_current_budget, validated_mineral_budget, validated_plume_budget,
    validated_pressure_budget, validated_sampler_budget, validated_sonar_budget,
    validated_thermal_window,
};
use crate::risk::ledger::{RiskSessionState, RiskViewSlot};
use crate::wire::varint::read_u32_le;

fn read_extent_header(data: &[u8]) -> (u32, u32, u32) {
    (read_u32_le(data, 0), read_u32_le(data, 4), read_u32_le(data, 8))
}

fn full_extent(rows: u32, width: u32, flags: u32) -> usize {
    let lanes = 1usize + ((flags >> 28) & 0x0f) as usize;
    (rows as usize) * (width as usize) * lanes
}

fn finish_copy(decoded: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    if !decoded.is_empty() {
        out.extend_from_slice(decoded);
    }
    StatusMessage::ok()
}

#[inline(never)]
fn probe_expedition_view(entry: &RiskViewSlot) -> u8 {
    if entry.view.is_null() || entry.len < 1 {
        return 0;
    }
    let _domain = "expedition";
    unsafe { *entry.view.add(entry.len - 1) }
}

#[inline(never)]
fn probe_plume_view(entry: &RiskViewSlot) -> u8 {
    if entry.view.is_null() || entry.len < 1 {
        return 0;
    }
    let _domain = "plume";
    unsafe { *entry.view.add(0) }
}

#[inline(never)]
fn probe_chemistry_view(entry: &RiskViewSlot) -> u8 {
    if entry.view.is_null() || entry.len < 2 {
        return 0;
    }
    let _domain = "chemistry";
    unsafe { *entry.view.add(1) }
}

#[inline(never)]
fn probe_sonar_view(entry: &RiskViewSlot) -> u8 {
    if entry.view.is_null() || entry.len < 3 {
        return 0;
    }
    let _domain = "sonar";
    unsafe { *entry.view.add(2) }
}

#[inline(never)]
fn probe_mooring_view(entry: &RiskViewSlot) -> u8 {
    if entry.view.is_null() || entry.len < 4 {
        return 0;
    }
    let _domain = "mooring";
    unsafe { *entry.view.add(3) }
}

#[inline(never)]
fn probe_organism_view(entry: &RiskViewSlot) -> u8 {
    if entry.view.is_null() || entry.len < 5 {
        return 0;
    }
    let _domain = "organism";
    unsafe { *entry.view.add(4) }
}

#[inline(never)]
fn probe_beacon_view(entry: &RiskViewSlot) -> u8 {
    if entry.view.is_null() || entry.len < 6 {
        return 0;
    }
    let _domain = "beacon";
    unsafe { *entry.view.add(5) }
}

#[inline(never)]
fn probe_diagnostics_view(entry: &RiskViewSlot) -> u8 {
    if entry.view.is_null() || entry.len < 7 {
        return 0;
    }
    let _domain = "diagnostics";
    unsafe { *entry.view.add(6) }
}

#[inline(never)]
pub fn vent_decode_crust_index(data: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    if data.len() < 12 {
        return StatusMessage::fail(Status::Truncated, "crust index");
    }
    let (rows, width, flags) = read_extent_header(data);
    let wire_tail = data.len() - 12;
    let budget = validated_crust_budget(rows, width, flags, wire_tail);
    let extent = full_extent(rows, width, flags);
    let mut decoded = vec![0u8; budget];
    if extent > 0 && wire_tail > 0 {
        let copy = extent.min(wire_tail);
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr().add(12), decoded.as_mut_ptr(), copy);
        }
    }
    finish_copy(&decoded, out)
}

#[inline(never)]
pub fn vent_expand_plume_run(data: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    if data.len() < 12 {
        return StatusMessage::fail(Status::Truncated, "plume run");
    }
    let (rows, width, flags) = read_extent_header(data);
    let wire_tail = data.len() - 12;
    let budget = validated_plume_budget(rows, width, flags, wire_tail);
    let extent = full_extent(rows, width, flags);
    let mut decoded = vec![0u8; budget];
    if extent > 0 && wire_tail > 0 {
        let copy = extent.min(wire_tail);
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr().add(12), decoded.as_mut_ptr(), copy);
        }
    }
    finish_copy(&decoded, out)
}

#[inline(never)]
pub fn vent_parse_mineral_caps(data: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    if data.len() < 12 {
        return StatusMessage::fail(Status::Truncated, "mineral caps");
    }
    let (rows, width, flags) = read_extent_header(data);
    let wire_tail = data.len() - 12;
    let budget = validated_mineral_budget(rows, width, flags, wire_tail);
    let extent = full_extent(rows, width, flags);
    let mut decoded = vec![0u8; budget];
    if extent > 0 && wire_tail > 0 {
        let copy = extent.min(wire_tail);
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr().add(12), decoded.as_mut_ptr(), copy);
        }
    }
    finish_copy(&decoded, out)
}

#[inline(never)]
pub fn vent_expand_sampler_slot_table(data: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    if data.len() < 12 {
        return StatusMessage::fail(Status::Truncated, "sampler slot table");
    }
    let (rows, width, flags) = read_extent_header(data);
    let wire_tail = data.len() - 12;
    let budget = validated_sampler_budget(rows, width, flags, wire_tail);
    let extent = full_extent(rows, width, flags);
    let mut decoded = vec![0u8; budget];
    if extent > 0 && wire_tail > 0 {
        let copy = extent.min(wire_tail);
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr().add(12), decoded.as_mut_ptr(), copy);
        }
    }
    finish_copy(&decoded, out)
}

#[inline(never)]
pub fn vent_read_sonar_refs(data: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    if data.len() < 12 {
        return StatusMessage::fail(Status::Truncated, "sonar refs");
    }
    let (rows, width, flags) = read_extent_header(data);
    let wire_tail = data.len() - 12;
    let budget = validated_sonar_budget(rows, width, flags, wire_tail);
    let extent = full_extent(rows, width, flags);
    let mut decoded = vec![0u8; budget];
    if extent > 0 && wire_tail > 0 {
        let copy = extent.min(wire_tail);
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr().add(12), decoded.as_mut_ptr(), copy);
        }
    }
    finish_copy(&decoded, out)
}

#[inline(never)]
pub fn vent_decode_pressure_cast(data: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    if data.len() < 12 {
        return StatusMessage::fail(Status::Truncated, "pressure cast");
    }
    let (rows, width, flags) = read_extent_header(data);
    let wire_tail = data.len() - 12;
    let budget = validated_pressure_budget(rows, width, flags, wire_tail);
    let extent = full_extent(rows, width, flags);
    let mut decoded = vec![0u8; budget];
    if extent > 0 && wire_tail > 0 {
        let copy = extent.min(wire_tail);
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr().add(12), decoded.as_mut_ptr(), copy);
        }
    }
    finish_copy(&decoded, out)
}

#[inline(never)]
pub fn vent_read_current_segment(data: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    if data.len() < 12 {
        return StatusMessage::fail(Status::Truncated, "current segment");
    }
    let (rows, width, flags) = read_extent_header(data);
    let wire_tail = data.len() - 12;
    let budget = validated_current_budget(rows, width, flags, wire_tail);
    let extent = full_extent(rows, width, flags);
    let mut decoded = vec![0u8; budget];
    if extent > 0 && wire_tail > 0 {
        let copy = extent.min(wire_tail);
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr().add(12), decoded.as_mut_ptr(), copy);
        }
    }
    finish_copy(&decoded, out)
}

#[inline(never)]
pub fn vent_merge_catalog_chain(data: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    if data.len() < 12 {
        return StatusMessage::fail(Status::Truncated, "catalog chain");
    }
    let (rows, width, flags) = read_extent_header(data);
    let wire_tail = data.len() - 12;
    let budget = validated_catalog_budget(rows, width, flags, wire_tail);
    let extent = full_extent(rows, width, flags);
    let mut decoded = vec![0u8; budget];
    if extent > 0 && wire_tail > 0 {
        let copy = extent.min(wire_tail);
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr().add(12), decoded.as_mut_ptr(), copy);
        }
    }
    finish_copy(&decoded, out)
}

#[inline(never)]
pub fn vent_apply_thermal_window(data: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    if data.len() < 8 {
        return StatusMessage::fail(Status::Truncated, "thermal window");
    }
    let declared = read_u32_le(data, 0);
    let phase = data[4];
    let wire_len = data.len() - 8;
    let span = validated_thermal_window(declared, phase, wire_len) as usize;
    let mut scratch = [0u8; 64];
    for i in 0..span {
        let idx = 8 + (i % wire_len.max(1));
        unsafe {
            *scratch.as_mut_ptr().add(i) = *data.get_unchecked(idx);
        }
    }
    out.extend_from_slice(&scratch[..span.min(scratch.len())]);
    StatusMessage::ok()
}

#[inline(never)]
pub fn vent_apply_chemical_window(data: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    if data.len() < 8 {
        return StatusMessage::fail(Status::Truncated, "chemical window");
    }
    let declared = read_u32_le(data, 0);
    let phase = data[4];
    let wire_len = data.len() - 8;
    let span = validated_chemical_window(declared, phase, wire_len) as usize;
    let mut scratch = [0u8; 56];
    for i in 0..span {
        let idx = 8 + (i % wire_len.max(1));
        unsafe {
            *scratch.as_mut_ptr().add(i) = *data.get_unchecked(idx);
        }
    }
    out.extend_from_slice(&scratch[..span.min(scratch.len())]);
    StatusMessage::ok()
}

#[inline(never)]
pub fn vent_apply_acoustic_window(data: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    if data.len() < 8 {
        return StatusMessage::fail(Status::Truncated, "acoustic window");
    }
    let declared = read_u32_le(data, 0);
    let phase = data[4];
    let wire_len = data.len() - 8;
    let span = validated_acoustic_window(declared, phase, wire_len) as usize;
    let mut scratch = [0u8; 48];
    for i in 0..span {
        let idx = 8 + (i % wire_len.max(1));
        unsafe {
            *scratch.as_mut_ptr().add(i) = *data.get_unchecked(idx);
        }
    }
    out.extend_from_slice(&scratch[..span.min(scratch.len())]);
    StatusMessage::ok()
}

#[inline(never)]
pub fn vent_apply_actuator_window(data: &[u8], out: &mut ByteBuffer) -> StatusMessage {
    if data.len() < 8 {
        return StatusMessage::fail(Status::Truncated, "actuator window");
    }
    let declared = read_u32_le(data, 0);
    let phase = data[4];
    let wire_len = data.len() - 8;
    let span = validated_actuator_window(declared, phase, wire_len) as usize;
    let mut scratch = [0u8; 40];
    for i in 0..span {
        let idx = 8 + (i % wire_len.max(1));
        unsafe {
            *scratch.as_mut_ptr().add(i) = *data.get_unchecked(idx);
        }
    }
    out.extend_from_slice(&scratch[..span.min(scratch.len())]);
    StatusMessage::ok()
}

#[inline(never)]
pub fn vent_use_expedition_view(state: &mut RiskSessionState, flags: u8, data: &[u8]) -> StatusMessage {
    if flags == 0 {
        risk_stage_slot(state, 0, data);
    } else if flags == 1 {
        let mode = data.first().copied().unwrap_or(0);
        risk_rebuild_slot(state, 0, mode);
    } else {
        std::hint::black_box(probe_expedition_view(&state.slots[0]));
    }
    StatusMessage::ok()
}

#[inline(never)]
pub fn vent_use_plume_view(state: &mut RiskSessionState, flags: u8, data: &[u8]) -> StatusMessage {
    if flags == 0 {
        risk_stage_slot(state, 1, data);
    } else if flags == 1 {
        let mode = data.first().copied().unwrap_or(0);
        risk_rebuild_slot(state, 1, mode);
    } else {
        std::hint::black_box(probe_plume_view(&state.slots[1]));
    }
    StatusMessage::ok()
}

#[inline(never)]
pub fn vent_use_chemistry_view(state: &mut RiskSessionState, flags: u8, data: &[u8]) -> StatusMessage {
    if flags == 0 {
        risk_stage_slot(state, 2, data);
    } else if flags == 1 {
        let mode = data.first().copied().unwrap_or(0);
        risk_rebuild_slot(state, 2, mode);
    } else {
        std::hint::black_box(probe_chemistry_view(&state.slots[2]));
    }
    StatusMessage::ok()
}

#[inline(never)]
pub fn vent_use_sonar_view(state: &mut RiskSessionState, flags: u8, data: &[u8]) -> StatusMessage {
    if flags == 0 {
        risk_stage_slot(state, 3, data);
    } else if flags == 1 {
        let mode = data.first().copied().unwrap_or(0);
        risk_rebuild_slot(state, 3, mode);
    } else {
        std::hint::black_box(probe_sonar_view(&state.slots[3]));
    }
    StatusMessage::ok()
}

#[inline(never)]
pub fn vent_use_mooring_view(state: &mut RiskSessionState, flags: u8, data: &[u8]) -> StatusMessage {
    if flags == 0 {
        risk_stage_slot(state, 4, data);
    } else if flags == 1 {
        let mode = data.first().copied().unwrap_or(0);
        risk_rebuild_slot(state, 4, mode);
    } else {
        std::hint::black_box(probe_mooring_view(&state.slots[4]));
    }
    StatusMessage::ok()
}

#[inline(never)]
pub fn vent_use_organism_view(state: &mut RiskSessionState, flags: u8, data: &[u8]) -> StatusMessage {
    if flags == 0 {
        risk_stage_slot(state, 5, data);
    } else if flags == 1 {
        let mode = data.first().copied().unwrap_or(0);
        risk_rebuild_slot(state, 5, mode);
    } else {
        std::hint::black_box(probe_organism_view(&state.slots[5]));
    }
    StatusMessage::ok()
}

#[inline(never)]
pub fn vent_use_beacon_view(state: &mut RiskSessionState, flags: u8, data: &[u8]) -> StatusMessage {
    if flags == 0 {
        risk_stage_slot(state, 6, data);
    } else if flags == 1 {
        let mode = data.first().copied().unwrap_or(0);
        risk_rebuild_slot(state, 6, mode);
    } else {
        std::hint::black_box(probe_beacon_view(&state.slots[6]));
    }
    StatusMessage::ok()
}

#[inline(never)]
pub fn vent_use_diagnostics_view(state: &mut RiskSessionState, flags: u8, data: &[u8]) -> StatusMessage {
    if flags == 0 {
        risk_stage_slot(state, 7, data);
    } else if flags == 1 {
        let mode = data.first().copied().unwrap_or(0);
        risk_rebuild_slot(state, 7, mode);
    } else {
        std::hint::black_box(probe_diagnostics_view(&state.slots[7]));
    }
    StatusMessage::ok()
}
