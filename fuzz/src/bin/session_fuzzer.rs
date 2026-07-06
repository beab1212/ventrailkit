//! libFuzzer harness for HVWS session feeder.

#![no_main]

use libfuzzer_sys::fuzz_target;
use ventrail_core::wire::session_feeder::SessionFeeder;

fuzz_target!(|data: &[u8]| {
    let _ = SessionFeeder::push(data);
});
