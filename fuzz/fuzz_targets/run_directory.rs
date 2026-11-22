#![no_main]

use libfuzzer_sys::fuzz_target;
use contextra_storage::run::DirectoryBlock;

fuzz_target!(|data: &[u8]| {
    // Directory-Parsing gegen beliebige Eingaben
    let file_len = 1024 * 1024;
    let _ = DirectoryBlock::decode(data, file_len);
});


