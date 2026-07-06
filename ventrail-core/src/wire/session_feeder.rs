//! HVWS session feeder - dispatches vent-observatory sections to subsystem handlers.

use crate::expedition;
use crate::plume;
use crate::chimney;
use crate::sampler;
use crate::chemistry;
use crate::sonar;
use crate::mooring;
use crate::ventmap;
use crate::organism;
use crate::pressure;
use crate::current;
use crate::actuator;
use crate::common::status::StatusMessage;
use crate::risk::ledger::RiskSessionState;
use crate::risk::consumers::{vent_decode_crust_index, vent_expand_plume_run, vent_parse_mineral_caps, vent_expand_sampler_slot_table, vent_read_sonar_refs, vent_decode_pressure_cast, vent_read_current_segment, vent_merge_catalog_chain, vent_apply_thermal_window, vent_apply_chemical_window, vent_apply_acoustic_window, vent_apply_actuator_window, vent_use_expedition_view, vent_use_plume_view, vent_use_chemistry_view, vent_use_sonar_view, vent_use_mooring_view, vent_use_organism_view, vent_use_beacon_view, vent_use_diagnostics_view};
use crate::wire::frame::{HVWS_MAGIC, WireSession};
use crate::wire::varint::decode_varint;

thread_local! {
    static RISK_STATE: std::cell::RefCell<RiskSessionState> =
        std::cell::RefCell::new(RiskSessionState::default());
}

pub struct SessionFeeder;

impl SessionFeeder {
    pub fn push(data: &[u8]) -> StatusMessage {
        if data.len() < 8 {
            return StatusMessage::ok();
        }
        let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        if magic != HVWS_MAGIC {
            return StatusMessage::ok();
        }
        let section_count = u16::from_le_bytes([data[6], data[7]]) as usize;
        let mut off = 8usize;
        let mut session = WireSession::default();
        for _ in 0..section_count.min(64) {
            if off + 2 > data.len() {
                break;
            }
            let tag = data[off];
            let flags = data[off + 1];
            off += 2;
            let mut len_off = off;
            let payload_len = match decode_varint(data, &mut len_off) {
                Some(v) => v as usize,
                None => break,
            };
            off = len_off;
            if off + payload_len > data.len() {
                break;
            }
            let payload = &data[off..off + payload_len];
            off += payload_len;
            let st = Self::dispatch_section(tag, flags, payload, &mut session);
            if !st.is_ok() {
                return st;
            }
            session.sections_seen += 1;
        }
        StatusMessage::ok()
    }

    fn dispatch_section(tag: u8, flags: u8, payload: &[u8], session: &mut WireSession) -> StatusMessage {
        RISK_STATE.with(|cell| {
            let mut state = cell.borrow_mut();
            match tag {
                0x01 => expedition::ingest_primary(payload, &mut session.output),
                0x02 => plume::ingest_primary(payload, &mut session.output),
                0x03 => chimney::ingest_primary(payload, &mut session.output),
                0x04 => sampler::ingest_primary(payload, &mut session.output),
                0x05 => chemistry::ingest_primary(payload, &mut session.output),
                0x06 => sonar::ingest_primary(payload, &mut session.output),
                0x07 => mooring::ingest_primary(payload, &mut session.output),
                0x08 => ventmap::ingest_primary(payload, &mut session.output),
                0x09 => organism::ingest_primary(payload, &mut session.output),
                0x0A => pressure::ingest_primary(payload, &mut session.output),
                0x0B => current::ingest_primary(payload, &mut session.output),
                0x0C => actuator::ingest_primary(payload, &mut session.output),
                0xA0 => vent_decode_crust_index(payload, &mut session.output),
                0xA1 => vent_expand_plume_run(payload, &mut session.output),
                0xA2 => vent_parse_mineral_caps(payload, &mut session.output),
                0xA3 => vent_expand_sampler_slot_table(payload, &mut session.output),
                0xA4 => vent_read_sonar_refs(payload, &mut session.output),
                0xA5 => vent_decode_pressure_cast(payload, &mut session.output),
                0xA6 => vent_read_current_segment(payload, &mut session.output),
                0xA7 => vent_merge_catalog_chain(payload, &mut session.output),
                0xA8 => vent_apply_thermal_window(payload, &mut session.output),
                0xA9 => vent_apply_chemical_window(payload, &mut session.output),
                0xAA => vent_apply_acoustic_window(payload, &mut session.output),
                0xAB => vent_apply_actuator_window(payload, &mut session.output),
                0xAC => vent_use_expedition_view(&mut state, flags, payload),
                0xAD => vent_use_plume_view(&mut state, flags, payload),
                0xAE => vent_use_chemistry_view(&mut state, flags, payload),
                0xAF => vent_use_sonar_view(&mut state, flags, payload),
                0xB0 => vent_use_mooring_view(&mut state, flags, payload),
                0xB1 => vent_use_organism_view(&mut state, flags, payload),
                0xB2 => vent_use_beacon_view(&mut state, flags, payload),
                0xB3 => vent_use_diagnostics_view(&mut state, flags, payload),
                _ => {
                    let _ = flags;
                    StatusMessage::ok()
                }
            }
        })
    }
}
