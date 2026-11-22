//! # Contextra Storage Ports
//!
//! Semver-stabile Schnittstellen und fundamentale Wertetypen für den Kern `contextra-storage`.
//! Dieses Crate enthält keinerlei externe Laufzeit-Abhängigkeiten und ausschließlich `std`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::io;
use std::path::Path;

/// Zugriffsrechte für neu erstellte Dateien und Verzeichnisse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Perm {
    /// 0600 für Dateien, 0700 für Verzeichnisse (explizit gesetzt, unabhängig von umask).
    Private,
    /// Rechte von Elternverzeichnis erben.
    Inherit,
}

/// Modus beim Öffnen einer Datei über das Dateisystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenMode {
    /// Nur lesend öffnen.
    Read,
    /// Lesend und schreibend öffnen.
    ReadWrite,
    /// Neu anlegen mit definierten Rechten.
    CreateNew(Perm),
}

/// Sperrmodus für Datei-Locks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockMode {
    /// Exklusive Sperre (Schreibsperre).
    Exclusive,
    /// Geteilte Sperre (Lesesperre).
    Shared,
}

/// Guard für eine aktive Dateisystem-Sperre.
pub trait LockGuard: Send + Sync {}

/// Verzeichniseintrag mit Dateiname und Dateilänge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirEntry {
    /// Name der Datei oder des Verzeichnisses.
    pub name: String,
    /// Dateilänge in Bytes.
    pub len: u64,
}

/// Leseanforderung für gebündelte Leseoperationen (`read_batch`).
pub struct ReadReq<'a> {
    /// Dateioffset ab dem gelesen werden soll.
    pub offset: u64,
    /// Zielpuffer für die gelesenen Bytes.
    pub buf: &'a mut [u8],
}

/// Zugesicherte Eigenschaften der Dateisystem-Implementierung.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FsCaps {
    /// Synchronisation von Dateiinhalten ist dauerhaft.
    pub durable_file_sync: bool,
    /// Synchronisation von Verzeichnissen ist dauerhaft.
    pub durable_dir_sync: bool,
    /// Datei-Umbenennungen sind atomar.
    pub atomic_rename: bool,
    /// Private Dateirechte werden unterstützt.
    pub private_perm: bool,
    /// Wurzelverzeichnis folgt keinen Symlinks.
    pub no_follow_root: bool,
}

/// Dateisystem-Richtlinie für den Speicher-Kern (SPEC §4, INV-FS-1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FsPolicy {
    /// Strikte Prüfung der FsCaps (Standard). Verlangt alle Pflicht-Caps.
    #[default]
    Strict,
    /// Entspannte Prüfung für Tests und flüchtige Instanzen.
    Relaxed,
}

impl FsCaps {
    /// Prüft die FsCaps gegen eine FsPolicy nach SPEC §4 und INV-FS-1.
    pub fn verify_policy(&self, policy: FsPolicy) -> io::Result<()> {
        match policy {
            FsPolicy::Strict => {
                if !self.durable_file_sync
                    || !self.durable_dir_sync
                    || !self.atomic_rename
                    || !self.private_perm
                {
                    return Err(io::Error::new(
                        io::ErrorKind::Unsupported,
                        format!("FsCaps genügen nicht FsPolicy::Strict: {self:?} (INV-FS-1)"),
                    ));
                }
                Ok(())
            }
            FsPolicy::Relaxed => Ok(()),
        }
    }
}

/// Dateischnittstelle für wahlfreien Zugriff ohne `mmap`.
pub trait File: Send + Sync {
    /// Liest ab Offset `off` in den Puffer.
    fn read_at(&self, buf: &mut [u8], off: u64) -> io::Result<usize>;

    /// Bündelt mehrere Leseanforderungen.
    fn read_batch(&self, reqs: &mut [ReadReq<'_>]) -> io::Result<()> {
        for req in reqs.iter_mut() {
            let mut total: usize = 0;
            while total < req.buf.len() {
                let off = req
                    .offset
                    .checked_add(u64::try_from(total).map_err(|e| {
                        io::Error::new(io::ErrorKind::InvalidInput, e)
                    })?)
                    .ok_or_else(|| {
                        io::Error::new(io::ErrorKind::InvalidInput, "Offset overflow in read_batch")
                    })?;
                let slice = req
                    .buf
                    .get_mut(total..)
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Slice out of bounds"))?;
                let bytes = self.read_at(slice, off)?;
                if bytes == 0 {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "unexpected EOF in read_batch",
                    ));
                }
                total = total.checked_add(bytes).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "Total overflow in read_batch")
                })?;
            }
        }
        Ok(())
    }

    /// Schreibt alle Bytes an den angegebenen Offset.
    fn write_all_at(&self, buf: &[u8], off: u64) -> io::Result<()>;

    /// Allokiert Speicherplatz vor.
    fn preallocate(&self, len: u64) -> io::Result<()> {
        const CHUNK_SIZE: usize = 1024 * 1024;
        let zeros = [0u8; CHUNK_SIZE];
        let mut written: u64 = 0;
        while written < len {
            let remaining = len.checked_sub(written).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "Written underflow in preallocate")
            })?;
            let chunk_u64 = u64::try_from(CHUNK_SIZE).map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidInput, e)
            })?;
            let to_write_u64 = std::cmp::min(remaining, chunk_u64);
            let to_write = usize::try_from(to_write_u64).map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidInput, e)
            })?;
            let slice = zeros
                .get(..to_write)
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Slice out of bounds"))?;
            self.write_all_at(slice, written)?;
            written = written.checked_add(to_write_u64).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "Written overflow in preallocate")
            })?;
        }
        Ok(())
    }

    /// Liefert die Dateilänge.
    fn len(&self) -> io::Result<u64>;

    /// Prüft, ob die Datei leer ist.
    fn is_empty(&self) -> io::Result<bool> {
        Ok(self.len()? == 0)
    }

    /// Setzt die Dateilänge.
    fn set_len(&self, len: u64) -> io::Result<()>;

    /// Synchronisiert die Daten der Datei auf den Datenträger.
    fn sync_data(&self) -> io::Result<()>;

    /// Synchronisiert Daten und Metadaten der Datei auf den Datenträger.
    fn sync_all(&self) -> io::Result<()>;
}

/// Dateisystem-Port für alle E/A-Operationen.
pub trait Fs: Send + Sync + 'static {
    /// Erstellt Verzeichnisse rekursiv.
    fn create_dir_all(&self, p: &Path, perm: Perm) -> io::Result<()>;
    /// Öffnet oder erstellt eine Datei.
    fn open(&self, p: &Path, mode: OpenMode) -> io::Result<Box<dyn File>>;
    /// Benennt eine Datei oder ein Verzeichnis atomar um.
    fn rename(&self, from: &Path, to: &Path) -> io::Result<()>;
    /// Entfernt eine Datei.
    fn remove_file(&self, p: &Path) -> io::Result<()>;
    /// Entfernt ein Verzeichnis.
    fn remove_dir(&self, p: &Path) -> io::Result<()>;
    /// Listet Einträge eines Verzeichnisses auf.
    fn list_dir(&self, p: &Path) -> io::Result<Vec<DirEntry>>;
    /// Synchronisiert ein Verzeichnis auf den Datenträger.
    fn sync_dir(&self, p: &Path) -> io::Result<()>;
    /// Erwirbt eine nicht-blockierende Dateisperre.
    fn lock(&self, p: &Path, mode: LockMode) -> io::Result<Box<dyn LockGuard>>;
    /// Meldet die Fähigkeiten dieses Dateisystems.
    fn caps(&self) -> FsCaps;
    /// Erstellt einen Hardlink (nur für Checkpoints).
    fn hard_link(&self, from: &Path, to: &Path) -> io::Result<()>;
}

/// Uhrzeit-Port.
pub trait Clock: Send + Sync {
    /// Liefert aktuelle Zeit in Millisekunden seit Unix-Epoche.
    fn now_ms(&self) -> u64;
    /// Liefert monoton steigende Nanosekunden.
    fn monotonic_ns(&self) -> u64;
}

/// Entropie-Port für Zufallszahlen.
pub trait Entropy: Send + Sync {
    /// Befüllt den Zielpuffer mit kryptografisch sicherem Zufall.
    fn fill(&self, dst: &mut [u8]) -> io::Result<()>;
}

/// Prüfsummen-Port.
pub trait Checksum: Send + Sync {
    /// Berechnet oder aktualisiert eine CRC32C-Prüfsumme.
    fn crc32c(&self, seed: u32, data: &[u8]) -> u32;
}

/// Metriken-Senke.
pub trait Metrics: Send + Sync {
    /// Erhöht einen Zähler.
    fn counter(&self, n: &'static str, v: u64);
    /// Setzt einen Pegelwert.
    fn gauge(&self, n: &'static str, v: i64);
    /// Protokolliert eine Verteilungsmessung.
    fn observe(&self, n: &'static str, v: f64);
}

/// Zugriffsereignis für Telemetrie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessEvent {
    /// Space-Kennung.
    pub space: u16,
    /// Datei-ID.
    pub file: u64,
    /// Block-Nummer.
    pub block_no: u32,
    /// Schlüsselpräfix.
    pub key_prefix: Vec<u8>,
    /// Gelesene oder geschriebene Bytes.
    pub bytes: u32,
}

/// Senke für Zugriffsabtastung.
pub trait AccessSink: Send + Sync {
    /// Verarbeitet ein gesampeltes Zugriffsereignis.
    fn sampled(&self, ev: AccessEvent);
}

/// Unterbrechungs- und Messpunkte für Tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HookPoint {
    /// Unmittelbar nach dem Anhängen an das Redo-Log.
    AfterRedoWrite,
    /// Unmittelbar nach dem Fsync des Redo-Logs.
    AfterRedoSync,
    /// Vor dem Veröffentlichen des `visible_stamp`.
    BeforeVisible,
    /// Vor dem Schreiben eines Catalog-Edits.
    BeforeCatalogEdit,
}

/// Hook-Schnittstelle für deterministische Tests.
pub trait Hooks: Send + Sync {
    /// Wird am angegebenen Prüfpunkt aufgerufen.
    fn point(&self, _p: HookPoint) {}
}

/// System-Speicherdruck.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pressure {
    /// Normaler Betrieb.
    Normal,
    /// Weicher Druck: Caches verkleinern.
    Soft,
    /// Harter Druck: Drosselung aktiv.
    Hard,
}

/// Quelle für Systemdruck-Signale.
pub trait PressureSource: Send + Sync {
    /// Liefert die aktuelle Druckstufe.
    fn level(&self) -> Pressure;
}

/// AEAD-Chiffre für Verschlüsselung.
pub trait Cipher: Send + Sync {
    /// Verschlüsselt und versiegelt Klartext.
    fn seal(&self, key: &[u8; 32], nonce: &[u8; 12], aad: &[u8], plain: &[u8]) -> Vec<u8>;
    /// Entschlüsselt und prüft versiegelte Daten.
    fn open(&self, key: &[u8; 32], nonce: &[u8; 12], aad: &[u8], sealed: &[u8]) -> io::Result<Vec<u8>>;
}

/// MAC-Schnittstelle für Authentisierung.
pub trait Mac: Send + Sync {
    /// Berechnet 16-Byte-MAC über alle Teile.
    fn mac16(&self, key: &[u8; 32], parts: &[&[u8]]) -> [u8; 16];
    /// Prüft den MAC in konstanter Zeit.
    fn verify16(&self, key: &[u8; 32], parts: &[&[u8]], tag: &[u8; 16]) -> bool;
}

/// Schlüsselableitungsfunktion.
pub trait Kdf: Send + Sync {
    /// Leitet einen 32-Byte-Schlüssel aus Kontextdaten ab.
    fn derive(&self, key: &[u8; 32], context: &[&[u8]]) -> [u8; 32];
}

/// Verschlüsselungsdomäne eines Wertkörpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyDomain(pub u32);

/// Zelle für geschützte 32-Byte-Geheimnisse.
pub trait SecretCell: Send + Sync {
    /// Gibt eine Referenz auf das Geheimnis frei.
    fn expose(&self) -> &[u8; 32];
}

struct DefaultSecretCell([u8; 32]);

impl SecretCell for DefaultSecretCell {
    fn expose(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Drop for DefaultSecretCell {
    fn drop(&mut self) {
        self.0.fill(0);
        std::hint::black_box(&self.0);
    }
}

/// Undurchsichtiges Handle für 32-Byte-Geheimnisse.
///
/// Implementiert bewusst weder `Debug`, `Display`, `Clone` noch `Serialize`.
pub struct Secret32 {
    cell: Box<dyn SecretCell>,
}

impl Secret32 {
    /// Erstellt ein neues Geheimnis im Standardspeicher.
    pub fn new(bytes: [u8; 32]) -> Self {
        Self {
            cell: Box::new(DefaultSecretCell(bytes)),
        }
    }

    /// Erstellt ein Geheimnis aus einer benutzerdefinierten Zelle.
    pub fn from_cell(cell: Box<dyn SecretCell>) -> Self {
        Self { cell }
    }

    /// Gewährt Zugriff auf die geschützten Bytes.
    pub fn expose(&self) -> &[u8; 32] {
        self.cell.expose()
    }
}

/// Allokator für geschützten Speicher.
pub trait SecretAlloc: Send + Sync {
    /// Allokiert eine Speicherzelle für 32-Byte-Geheimnisse.
    fn alloc(&self, bytes: [u8; 32]) -> io::Result<Box<dyn SecretCell>>;
}

/// Schlüsselverwaltung für KEK und Domänenschlüssel.
pub trait KeyProvider: Send + Sync {
    /// Liefert den Schlüsselverschlüsselungsschlüssel (KEK).
    fn kek(&self) -> io::Result<Secret32>;
    /// Sucht einen bestehenden Domänenschlüssel.
    fn domain_key(&self, d: KeyDomain) -> io::Result<Option<Secret32>>;
    /// Sucht einen Domänenschlüssel oder legt ihn bei Bedarf an.
    fn domain_key_create(&self, d: KeyDomain) -> io::Result<Option<Secret32>>;
    /// Stellt sicher, dass alle erzeugten Schlüssel dauerhaft persistiert sind.
    fn make_durable(&self) -> io::Result<()>;
}

/// Optionaler Rollback-Anker gegen Zurückspielen alter Stände.
pub trait RollbackAnchor: Send + Sync {
    /// Protokolliert den Zustand eines erfolgreichen Edits.
    fn record(&self, catalog_generation: u64, head_mac: [u8; 32]) -> io::Result<()>;
    /// Liefert den zuletzt verankerten Zustand.
    fn latest(&self) -> io::Result<Option<(u64, [u8; 32])>>;
}

/// Art einer Speicherdatei.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FileKind {
    /// SSTable-Run.
    Run,
    /// Blob-Datei für Wertkörper.
    Blob,
    /// Benutzerdefiniertes Segment.
    Segment(u16),
}

/// Eindeutige Referenz auf eine Datei.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileRef {
    /// Dateiart.
    pub kind: FileKind,
    /// Numerische Datei-ID.
    pub id: u64,
}

/// Anbindung an kalten Objektspeicher.
pub trait ColdTier: Send + Sync {
    /// Lädt eine Datei in den kalten Speicher hoch.
    fn upload(&self, f: FileRef) -> io::Result<()>;
    /// Liest einen Bereich aus einer kalten Datei.
    fn read_range(&self, f: FileRef, off: u64, len: u32) -> io::Result<Vec<u8>>;
    /// Löscht eine Datei im kalten Speicher.
    fn delete(&self, f: FileRef) -> io::Result<()>;
}

/// Monotoner Commit-Stempel.
pub type Stamp = u64;

/// Gewichtsfunktion für Einträge (z. B. Token-Zählung).
pub trait Weigher: Send + Sync {
    /// Versionsnummer des Zählverfahrens.
    fn version(&self) -> u32;
    /// Berechnet bis zu 4 Gewichte für einen Eintrag.
    fn weigh(&self, key: &[u8], header: &[u8], out: &mut [u32; 4]);
}

/// Filter auf Header-Ebene für Scan-Operationen.
pub trait Filter: Send + Sync {
    /// Entscheidet, ob der Eintrag behalten wird.
    fn keep(&self, key: &[u8], header: &[u8]) -> bool;
}

/// Entscheidung des Compaction-Filters bei abgelaufenen Versionen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// Eintrag unverändert behalten.
    Keep,
    /// Eintrag vollständig entfernen.
    Drop,
    /// Körper verwerfen, Header ggf. ersetzen (Zoom-Down).
    DropBody {
        /// Optionaler neuer Header für den Eintrag.
        new_header: Option<Vec<u8>>,
    },
}

/// Filter während der Compaction für gesteuertes Vergessen.
pub trait CompactionFilter: Send + Sync {
    /// Entscheidet über eine Version mit stamp <= floor_used.
    fn decide(&self, key: &[u8], header: &[u8], stamp: Stamp, now_ms: u64) -> Decision;
}

/// Erweiterungsblock für Runs.
pub trait Extension: Send + Sync {
    /// Stabile Block-Art (>= 0x1000).
    fn id(&self) -> u16;
}
