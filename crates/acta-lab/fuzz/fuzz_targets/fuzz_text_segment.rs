#![no_main]

use acta_lab::format::text_segment::TextSegmentFile;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = TextSegmentFile::parse(data);
});
