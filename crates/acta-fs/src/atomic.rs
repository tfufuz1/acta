//! Atomares, dauerhaftes Schreiben einer ganzen Datei (WP-5) mit der Regel nach
//! `FsyncFailed` (Spec 3.7, WP-7, 15.2).
//!
//! Ablauf je Versuch: Ersatzdatei neu anlegen, vollständig schreiben, `sync_data`,
//! `rename_atomic` (Verzeichnis-`fsync` inbegriffen). Scheitert `sync_data`, wird die
//! Datei nie wieder als stabil angenommen: der Inhalt geht in eine **neue** Datei
//! (zweiter Versuch). Scheitert auch dieser, ist der Bereich zu vergiften.
//! Alle anderen Fehler werden nicht wiederholt, sondern gemeldet.

use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};

use crate::error::FsError;
use crate::vfs::Vfs;

/// Anzahl der Versuche mit je einer neuen Ersatzdatei (erster Versuch und ein Ersatz).
const ATTEMPTS: u8 = 2;

enum Attempt {
    FsyncFailed,
    Io(io::Error),
}

/// Schreibt `bytes` so nach `target`, dass nach einem Absturz entweder der alte oder der
/// volle neue Inhalt vorliegt.
///
/// # Errors
/// - [`FsError::Io`] bei jedem Fehler außer `FsyncFailed`; die Ersatzdatei wird entfernt.
/// - [`FsError::Poisoned`], wenn `sync_data` auch auf der Ersatzdatei scheitert.
pub fn write_atomic(vfs: &dyn Vfs, target: &Path, bytes: &[u8]) -> Result<(), FsError> {
    for attempt in 0..ATTEMPTS {
        let tmp = tmp_path(target, attempt).map_err(FsError::Io)?;
        match try_once(vfs, &tmp, target, bytes) {
            Ok(()) => return Ok(()),
            Err(Attempt::FsyncFailed) => discard(vfs, &tmp),
            Err(Attempt::Io(e)) => {
                discard(vfs, &tmp);
                return Err(FsError::Io(e));
            }
        }
    }
    Err(FsError::Poisoned)
}

fn try_once(vfs: &dyn Vfs, tmp: &Path, target: &Path, bytes: &[u8]) -> Result<(), Attempt> {
    // Eine Ersatzdatei aus einem früheren Absturz ist verwaist (C03, C11) und wird ersetzt.
    match vfs.remove_scrub(tmp) {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => return Err(Attempt::Io(e)),
    }
    let file = vfs.create_new(tmp).map_err(Attempt::Io)?;
    file.write_all_at(0, bytes).map_err(Attempt::Io)?;
    file.sync_data().map_err(|_| Attempt::FsyncFailed)?;
    vfs.rename_atomic(tmp, target).map_err(Attempt::Io)
}

/// Entfernt eine gescheiterte Ersatzdatei. Gelingt das nicht, bleibt sie als verwaiste
/// Datei zurück und wird vom Bereinigungslauf entfernt (C03); der Zielpfad ist unberührt.
fn discard(vfs: &dyn Vfs, tmp: &Path) {
    if vfs.remove_scrub(tmp).is_err() {
        // Absichtlich ohne Folge: siehe oben.
    }
}

fn tmp_path(target: &Path, attempt: u8) -> io::Result<PathBuf> {
    let name = target
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "target has no file name"))?;
    let mut tmp: OsString = name.to_os_string();
    tmp.push(format!(".tmp{attempt}"));
    Ok(target.with_file_name(tmp))
}
