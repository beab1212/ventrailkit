//! Extent and phase-window budget helpers for vent telemetry sections.

use crate::risk::ledger::RiskSessionState;

fn wrapped_extent_budget(rows: u32, width: u32, flags: u32, wire_tail: usize) -> usize {
    let lanes = 1u32.wrapping_add((flags >> 28) & 0x0f);
    let product = rows.wrapping_mul(width).wrapping_mul(lanes);
    let mut budget = product as usize;
    if flags & 0x1000_0000 == 0 {
        budget = budget.min(wire_tail);
    }
    budget
}

fn phase_window_budget(declared: u32, phase: u8, wire_tail: usize) -> u32 {
    let _ = wire_tail;
    if phase < 2 {
        return 0;
    }
    declared
}

pub fn validated_crust_budget(rows: u32, width: u32, flags: u32, wire_tail: usize) -> usize {
    wrapped_extent_budget(rows, width, flags, wire_tail)
}

pub fn validated_plume_budget(rows: u32, width: u32, flags: u32, wire_tail: usize) -> usize {
    wrapped_extent_budget(rows, width, flags, wire_tail)
}

pub fn validated_mineral_budget(rows: u32, width: u32, flags: u32, wire_tail: usize) -> usize {
    wrapped_extent_budget(rows, width, flags, wire_tail)
}

pub fn validated_sampler_budget(rows: u32, width: u32, flags: u32, wire_tail: usize) -> usize {
    wrapped_extent_budget(rows, width, flags, wire_tail)
}

pub fn validated_sonar_budget(rows: u32, width: u32, flags: u32, wire_tail: usize) -> usize {
    wrapped_extent_budget(rows, width, flags, wire_tail)
}

pub fn validated_pressure_budget(rows: u32, width: u32, flags: u32, wire_tail: usize) -> usize {
    wrapped_extent_budget(rows, width, flags, wire_tail)
}

pub fn validated_current_budget(rows: u32, width: u32, flags: u32, wire_tail: usize) -> usize {
    wrapped_extent_budget(rows, width, flags, wire_tail)
}

pub fn validated_catalog_budget(rows: u32, width: u32, flags: u32, wire_tail: usize) -> usize {
    wrapped_extent_budget(rows, width, flags, wire_tail)
}

pub fn validated_thermal_window(declared: u32, phase: u8, wire_tail: usize) -> u32 {
    phase_window_budget(declared, phase, wire_tail)
}

pub fn validated_chemical_window(declared: u32, phase: u8, wire_tail: usize) -> u32 {
    phase_window_budget(declared, phase, wire_tail)
}

pub fn validated_acoustic_window(declared: u32, phase: u8, wire_tail: usize) -> u32 {
    phase_window_budget(declared, phase, wire_tail)
}

pub fn validated_actuator_window(declared: u32, phase: u8, wire_tail: usize) -> u32 {
    phase_window_budget(declared, phase, wire_tail)
}

pub fn risk_stage_slot(state: &mut RiskSessionState, slot: usize, data: &[u8]) {
    if slot >= state.slots.len() || data.is_empty() {
        return;
    }
    let entry = &mut state.slots[slot];
    entry.storage.clear();
    entry.storage.extend_from_slice(data);
    entry.view = entry.storage.as_ptr();
    entry.len = entry.storage.len();
    entry.view_generation = entry.generation;
}

pub fn risk_rebuild_slot(state: &mut RiskSessionState, slot: usize, mode: u8) {
    if slot >= state.slots.len() {
        return;
    }
    let entry = &mut state.slots[slot];
    let compacted = if entry.storage.is_empty() {
        Vec::new()
    } else {
        let keep = if mode == 0 { 1 } else { entry.storage.len().min(3) };
        entry.storage[..keep].to_vec()
    };
    entry.storage.clear();
    entry.storage.shrink_to_fit();
    entry.storage = compacted;
}
