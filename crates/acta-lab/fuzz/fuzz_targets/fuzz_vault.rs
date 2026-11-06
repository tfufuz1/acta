#![no_main]

use acta_lab::format::vault::VaultFile;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = VaultFile::parse(data);
});
