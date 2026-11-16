//! Conformance-Suite des Ports `Vfs` (Spec 3.7, 16.2): dieselben Prüfungen laufen gegen
//! `DurableVfs` und gegen `FaultVfs<DurableVfs>` ohne aktiven Plan. Beide müssen sich
//! gleich verhalten; jede Abweichung ist ein Defekt des Wrappers.
#![allow(clippy::indexing_slicing)]

mod common;

use acta_fs::atomic::write_atomic;
use acta_fs::durable::DurableVfs;
use acta_fs::fault::FaultVfs;
use acta_fs::vfs::Vfs;
use std::io::{self, ErrorKind};
use std::path::Path;

fn put(vfs: &dyn Vfs, p: &Path, data: &[u8]) -> io::Result<()> {
    let f = vfs.create_new(p)?;
    f.write_all_at(0, data)?;
    f.sync_data().map_err(io::Error::other)
}

fn get(vfs: &dyn Vfs, p: &Path) -> io::Result<Vec<u8>> {
    let f = vfs.open_read(p)?;
    let mut buf = vec![0_u8; usize::try_from(f.len()?).map_err(io::Error::other)?];
    f.read_exact_at(0, &mut buf)?;
    Ok(buf)
}

fn kind<T>(r: io::Result<T>) -> Option<ErrorKind> {
    r.err().map(|e| e.kind())
}

/// Die gemeinsame Suite. Orakel sind die in Spec 3.7 festgelegten Fehlerbilder.
fn suite(vfs: &dyn Vfs, dir: &Path) -> io::Result<()> {
    // create_new: Positivkontrolle (Anlage gelingt), dann AlreadyExists.
    let a = dir.join("a");
    put(vfs, &a, b"alpha")?;
    assert_eq!(kind(vfs.create_new(&a)), Some(ErrorKind::AlreadyExists));
    assert_eq!(get(vfs, &a)?, b"alpha");

    // open_read auf fehlende Datei: NotFound.
    assert_eq!(
        kind(vfs.open_read(&dir.join("missing"))),
        Some(ErrorKind::NotFound)
    );

    // read_exact_at über das Ende: UnexpectedEof; Länge stimmt.
    let f = vfs.open_read(&a)?;
    assert_eq!(f.len()?, 5);
    let mut buf = [0_u8; 8];
    assert_eq!(
        kind(f.read_exact_at(0, &mut buf)),
        Some(ErrorKind::UnexpectedEof)
    );

    // Schreiben ab Offset, Lesen zurück.
    let b = dir.join("b");
    let w = vfs.create_new(&b)?;
    w.write_all_at(4, b"tail")?;
    w.write_all_at(0, b"head")?;
    w.sync_data().map_err(io::Error::other)?;
    assert_eq!(get(vfs, &b)?, b"headtail");

    // rename_atomic ersetzt das Ziel vollständig.
    let tmp = dir.join("c.tmp");
    let c = dir.join("c");
    put(vfs, &c, b"old-content-long")?;
    put(vfs, &tmp, b"new")?;
    vfs.rename_atomic(&tmp, &c)?;
    assert_eq!(get(vfs, &c)?, b"new");
    assert_eq!(kind(vfs.open_read(&tmp)), Some(ErrorKind::NotFound));

    // write_atomic: ersetzt, hinterlässt keine Ersatzdatei.
    write_atomic(vfs, &c, b"replaced").map_err(io::Error::other)?;
    assert_eq!(get(vfs, &c)?, b"replaced");

    // list: volle Pfade, aufsteigend.
    let names: Vec<_> = vfs
        .list(dir)?
        .iter()
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
        .collect();
    assert_eq!(names, ["a", "b", "c"]);

    // remove_scrub: Datei ist danach fort; erneut: NotFound.
    vfs.remove_scrub(&a)?;
    assert_eq!(kind(vfs.open_read(&a)), Some(ErrorKind::NotFound));
    assert_eq!(kind(vfs.remove_scrub(&a)), Some(ErrorKind::NotFound));

    // O_DIRECT-Rücklesen liefert dieselben Bytes wie der gepufferte Pfad.
    let d = dir.join("d");
    put(
        vfs,
        &d,
        &(0..9000_u32).map(|i| (i % 253) as u8).collect::<Vec<_>>(),
    )?;
    let direct = vfs.open_direct_read(&d)?;
    let mut got = vec![0_u8; 100];
    direct.read_exact_at(4090, &mut got)?;
    assert_eq!(got, get(vfs, &d)?[4090..4190]);
    Ok(())
}

#[test]
fn durable_vfs_conforms() -> io::Result<()> {
    let dir = common::scratch("conf-durable")?;
    suite(&DurableVfs, &dir)
}

#[test]
fn fault_vfs_without_plan_conforms() -> io::Result<()> {
    let dir = common::scratch("conf-fault")?;
    suite(&FaultVfs::new(DurableVfs), &dir)
}
