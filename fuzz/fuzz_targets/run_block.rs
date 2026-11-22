#![no_main]

use libfuzzer_sys::fuzz_target;
use contextra_storage::run::DataBlockReader;

fuzz_target!(|data: &[u8]| {
    // Versucht beliebige Byteströme als DataBlock zu parsen und zu iterieren
    if let Ok(reader) = DataBlockReader::new(data) {
        let _ = reader.iter().collect::<Vec<_>>();
        let _ = reader.get(b"test_key");
    }
});
