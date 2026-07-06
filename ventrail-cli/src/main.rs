//! Ventrail command-line tool for HVWS sessions and HVBK packages.

use std::env;
use std::fs;
use std::path::Path;
use std::process;

use ventrail_core::common::buffer::ByteBuffer;
use ventrail_core::package::container_validator;
use ventrail_core::wire::session_feeder::SessionFeeder;

fn usage() -> ! {
    eprintln!("usage:");
    eprintln!("  ventrail ingest <session.bin>    Process an HVWS wire session");
    eprintln!("  ventrail validate <package.bin>  Validate an HVBK expedition bundle");
    eprintln!("  ventrail stats <session.bin>     Summarize HVWS section tags");
    process::exit(2);
}

fn cmd_ingest(path: &str) {
    let data = fs::read(path).unwrap_or_else(|e| {
        eprintln!("read {}: {e}", path);
        process::exit(1);
    });
    let status = SessionFeeder::push(&data);
    if !status.is_ok() {
        eprintln!("ingest failed: {}", status.detail);
        process::exit(1);
    }
    println!("ingested {} bytes", data.len());
}

fn cmd_validate(path: &str) {
    let data = fs::read(path).unwrap_or_else(|e| {
        eprintln!("read {}: {e}", path);
        process::exit(1);
    });
    let status = container_validator::validate_package(&data);
    if !status.is_ok() {
        eprintln!("validate failed: {}", status.detail);
        process::exit(1);
    }
    let mut out = ByteBuffer::new();
    let _ = container_validator::ingest_container_validator(&data, &mut out);
    println!("valid HVBK package ({} bytes, {} health bytes)", data.len(), out.len());
}

fn decode_varint_local(data: &[u8], off: &mut usize) -> Option<u64> {
    let mut value = 0u64;
    let mut shift = 0u32;
    while *off < data.len() {
        let byte = data[*off];
        *off += 1;
        value |= ((byte & 0x7f) as u64) << shift;
        if byte & 0x80 == 0 { return Some(value); }
        shift += 7;
        if shift > 63 { return None; }
    }
    None
}

fn cmd_stats(path: &str) {
    let data = fs::read(path).unwrap_or_else(|e| {
        eprintln!("read {}: {e}", path);
        process::exit(1);
    });
    if data.len() < 8 {
        eprintln!("stats: file too short for HVWS header");
        process::exit(1);
    }
    let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    if magic != 0x5357_5648 {
        eprintln!("stats: not an HVWS session (magic 0x{:08X})", magic);
        process::exit(1);
    }
    let version = u16::from_le_bytes([data[4], data[5]]);
    let section_count = u16::from_le_bytes([data[6], data[7]]) as usize;
    println!("HVWS version {} sections {}", version, section_count);
    let mut off = 8usize;
    let mut tag_counts = [0u32; 256];
    for _ in 0..section_count.min(64) {
        if off + 2 > data.len() { break; }
        let tag = data[off];
        off += 2;
        let mut len_off = off;
        let payload_len = match decode_varint_local(&data, &mut len_off) {
            Some(v) => v as usize,
            None => break,
        };
        off = len_off.saturating_add(payload_len);
        tag_counts[tag as usize] += 1;
    }
    for tag in 0..=255u8 {
        let count = tag_counts[tag as usize];
        if count > 0 {
            println!("  tag 0x{:02X}: {}", tag, count);
        }
        if tag == 255 { break; }
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let cmd = args.next().unwrap_or_else(|| usage());
    let path = args.next().unwrap_or_else(|| usage());
    if args.next().is_some() { usage(); }
    if !Path::new(&path).exists() {
        eprintln!("file not found: {}", path);
        process::exit(1);
    }
    match cmd.as_str() {
        "ingest" => cmd_ingest(&path),
        "validate" => cmd_validate(&path),
        "stats" => cmd_stats(&path),
        _ => usage(),
    }
}
