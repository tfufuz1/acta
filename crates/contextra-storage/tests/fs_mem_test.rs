//! # MemFs Konformitäts- und Crash-Tests

#![allow(clippy::unwrap_used, clippy::expect_used)]

use contextra_storage::testing::conformance::run_fs_conformance_suite;
use contextra_storage::testing::MemFs;
use contextra_storage_ports::{Fs, OpenMode, Perm};
use std::path::Path;

#[test]
fn test_mem_fs_conformance() {
    let mem_fs = MemFs::new();
    let root = Path::new("/test_root");
    mem_fs
        .create_dir_all(root, Perm::Private)
        .expect("create root dir");
    run_fs_conformance_suite(&mem_fs, root);
}

#[test]
fn test_mem_fs_crash_reverts_uncommitted_data() {
    let fs = MemFs::new();
    let root = Path::new("/data");
    fs.create_dir_all(root, Perm::Private).unwrap();

    let file_path = root.join("journal.log");
    let file = fs
        .open(&file_path, OpenMode::CreateNew(Perm::Private))
        .unwrap();

    file.write_all_at(b"COMMITTED", 0).unwrap();
    file.sync_data().unwrap();
    fs.sync_dir(root).unwrap();

    // Schreiben ohne Sync
    file.write_all_at(b"UNCOMMITTED_DATA", 9).unwrap();

    // Absturz simulieren
    fs.crash();

    let reopened = fs.open(&file_path, OpenMode::Read).unwrap();
    assert_eq!(reopened.len().unwrap(), 9);

    let mut buf = vec![0u8; 9];
    reopened.read_at(&mut buf, 0).unwrap();
    assert_eq!(&buf, b"COMMITTED");
}
