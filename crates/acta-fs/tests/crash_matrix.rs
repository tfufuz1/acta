//! Absturzmatrix der Persistenzoperationen (Spec 16.4, WP-5, WP-6, WP-7, I7).
//!
//! Invariante je Zelle: Nach dem Absturz liegt am Zielpfad der alte oder der volle neue
//! Inhalt vor, nie ein Teilstand. Jede Zelle hat die Positivkontrolle eines Laufs ohne
//! Störung, der den neuen Inhalt liefert.
#![allow(clippy::indexing_slicing)]

mod common;

use acta_fs::atomic::write_atomic;
use acta_fs::durable::DurableVfs;
use acta_fs::error::FsError;
use acta_fs::fault::{FaultAction, FaultPlan, FaultPoint, FaultVfs};
use acta_fs::vfs::Vfs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

const OLD: &[u8] = b"old-content-of-the-target-file";
const NEW: &[u8] = b"NEW";

fn setup(name: &str) -> io::Result<(PathBuf, PathBuf)> {
    let dir = common::scratch(name)?;
    let target = dir.join("target");
    let f = DurableVfs.create_new(&target)?;
    f.write_all_at(0, OLD)?;
    f.sync_data().map_err(io::Error::other)?;
    Ok((dir, target))
}

fn read(p: &Path) -> io::Result<Vec<u8>> {
    let f = DurableVfs.open_read(p)?;
    let mut buf = vec![0_u8; usize::try_from(f.len()?).map_err(io::Error::other)?];
    f.read_exact_at(0, &mut buf)?;
    Ok(buf)
}

fn plan(point: FaultPoint, skip: u64, action: FaultAction) -> FaultPlan {
    FaultPlan {
        point,
        skip,
        action,
    }
}

/// WP-5: Absturz oder zerrissener Schreibvorgang an jedem Punkt von `write_atomic`.
#[test]
fn write_atomic_is_all_or_nothing_in_every_cell() -> io::Result<()> {
    let points = [
        FaultPoint::Create,
        FaultPoint::Write,
        FaultPoint::Sync,
        FaultPoint::Rename,
    ];
    let actions = [
        FaultAction::Crash,
        FaultAction::TornWrite(1),
        FaultAction::Fail(ErrorKind::StorageFull),
    ];
    for (i, point) in points.into_iter().enumerate() {
        for (j, action) in actions.into_iter().enumerate() {
            let (dir, target) = setup(&format!("cm-{i}-{j}"))?;
            let vfs = FaultVfs::new(DurableVfs);

            // Positivkontrolle: derselbe Aufbau ohne Plan liefert den neuen Inhalt.
            let ctl_target = dir.join("control");
            let ctl = DurableVfs.create_new(&ctl_target)?;
            ctl.write_all_at(0, OLD)?;
            write_atomic(&vfs, &ctl_target, NEW).map_err(io::Error::other)?;
            assert_eq!(read(&ctl_target)?, NEW, "Kontrolle {point:?}");

            let base = vfs.count(point);
            vfs.arm(plan(point, base, action));
            let result = write_atomic(&vfs, &target, NEW);
            let content = read(&target)?;
            assert!(
                content == OLD || content == NEW,
                "Teilstand in Zelle {point:?}/{action:?}: {content:?}"
            );
            if matches!(point, FaultPoint::Rename) && matches!(action, FaultAction::Crash) {
                assert_eq!(content, OLD, "Absturz vor rename: alter Stand");
                assert!(result.is_err());
            }
        }
    }
    Ok(())
}

/// WP-7, I7: Nach `FsyncFailed` geht der Inhalt in eine neue Datei; es gibt Erfolg.
#[test]
fn single_fsync_failure_falls_back_to_a_new_file() -> io::Result<()> {
    let (dir, target) = setup("fsync-once")?;
    let vfs = FaultVfs::new(DurableVfs);
    vfs.arm(plan(
        FaultPoint::Sync,
        0,
        FaultAction::Fail(ErrorKind::Other),
    ));
    write_atomic(&vfs, &target, NEW).map_err(io::Error::other)?;
    assert_eq!(read(&target)?, NEW);
    // Positivkontrolle des Ersatzes: zwei Anlagen, zwei Synchronisationen.
    assert_eq!(vfs.count(FaultPoint::Create), 2);
    assert_eq!(vfs.count(FaultPoint::Sync), 2);
    // Keine Ersatzdatei bleibt zurück.
    let names: Vec<_> = DurableVfs.list(&dir)?;
    assert_eq!(names, [target]);
    Ok(())
}

/// 15.2: Scheitert auch der Ersatz, ist der Bereich zu vergiften; das Ziel bleibt alt.
#[test]
fn double_fsync_failure_poisons_and_keeps_old_content() -> io::Result<()> {
    let (dir, target) = setup("fsync-twice")?;
    let vfs = FaultVfs::new(DurableVfs);
    vfs.arm(plan(
        FaultPoint::Sync,
        0,
        FaultAction::Fail(ErrorKind::Other),
    ));
    vfs.arm_also(plan(
        FaultPoint::Sync,
        1,
        FaultAction::Fail(ErrorKind::Other),
    ));
    let r = write_atomic(&vfs, &target, NEW);
    assert!(matches!(r, Err(FsError::Poisoned)));
    assert_eq!(read(&target)?, OLD);
    assert_eq!(vfs.count(FaultPoint::Create), 2, "genau ein Ersatzversuch");
    assert_eq!(DurableVfs.list(&dir)?, [target], "Ersatzdateien entfernt");
    Ok(())
}

/// Andere Fehler (ENOSPC) werden nicht wiederholt und hinterlassen keine Ersatzdatei.
#[test]
fn io_error_is_not_retried_and_cleans_up() -> io::Result<()> {
    let (dir, target) = setup("enospc")?;
    let vfs = FaultVfs::new(DurableVfs);
    vfs.arm(plan(
        FaultPoint::Write,
        0,
        FaultAction::Fail(ErrorKind::StorageFull),
    ));
    let r = write_atomic(&vfs, &target, NEW);
    assert!(matches!(r, Err(FsError::Io(ref e)) if e.kind() == ErrorKind::StorageFull));
    assert_eq!(vfs.count(FaultPoint::Create), 1);
    assert_eq!(read(&target)?, OLD);
    assert_eq!(DurableVfs.list(&dir)?, [target]);
    Ok(())
}

/// WP-6: Absturz vor `remove_scrub` lässt die Datei unverändert; ohne Störung ist sie fort.
#[test]
fn remove_scrub_crash_leaves_file_intact() -> io::Result<()> {
    let (_dir, target) = setup("scrub-crash")?;
    let vfs = FaultVfs::new(DurableVfs);
    vfs.arm(plan(FaultPoint::Remove, 0, FaultAction::Crash));
    assert!(vfs.remove_scrub(&target).is_err());
    assert!(vfs.is_crashed());
    assert_eq!(read(&target)?, OLD);

    let healthy = FaultVfs::new(DurableVfs);
    healthy.remove_scrub(&target)?; // Positivkontrolle
    assert_eq!(
        read(&target).err().map(|e| e.kind()),
        Some(ErrorKind::NotFound)
    );
    Ok(())
}

/// Eine verwaiste Ersatzdatei aus einem früheren Absturz blockiert den nächsten Versuch nicht.
#[test]
fn stale_temporary_file_is_replaced() -> io::Result<()> {
    let (dir, target) = setup("stale-tmp")?;
    let stale = dir.join("target.tmp0");
    let f = DurableVfs.create_new(&stale)?;
    f.write_all_at(0, b"junk-from-crash")?;
    write_atomic(&DurableVfs, &target, NEW).map_err(io::Error::other)?;
    assert_eq!(read(&target)?, NEW);
    assert_eq!(DurableVfs.list(&dir)?, [target]);
    Ok(())
}
