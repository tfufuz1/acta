#![no_main]

use acta_lab::format::vector_snapshot::VectorSnapshotFile;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = VectorSnapshotFile::parse(data);
});
