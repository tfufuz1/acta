#![no_main]

use libfuzzer_sys::fuzz_target;
use contextra_storage::run::Footer;

fuzz_target!(|data: &[u8]| {
    // Footer hat feste Länge von 28 Byte
    let _ = Footer::decode(data);
});

