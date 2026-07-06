//! Deterministic PoC runner for local AddressSanitizer verification.

use std::env;
use std::fs;
use std::process;
use ventrail_core::wire::session_feeder::SessionFeeder;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: poc_runner <session.bin>");
        process::exit(2);
    });
    let data = fs::read(&path).unwrap_or_else(|e| {
        eprintln!("read {path}: {e}");
        process::exit(1);
    });
    let status = SessionFeeder::push(&data);
    if !status.is_ok() {
        eprintln!("ingest failed: {}", status.detail);
        process::exit(1);
    }
}
