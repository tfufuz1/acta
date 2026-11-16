//! Schreiber des Gates (SPEC 5.3 `Z4`, 3.7 Fsync-Regel).
//!
//! Reihenfolge einer Sperre:
//! 1. Stapel auf einer Kopie des Standes prüfen; fehlerhafte Stapel verändern nichts.
//! 2. Neuen Stand im Arbeitsspeicher veröffentlichen (Epoche erhöht, Lesesperre sofort wirksam).
//! 3. Rahmen anhängen und `fsync`. Erst danach meldet `commit` Erfolg.
//! 4. Schlägt Anhängen oder `fsync` fehl, bleibt die Sperre im Arbeitsspeicher bestehen,
//!    der Aufruf scheitert mit `Integrity`, und der Mandant ist vergiftet (15.2).
//!    Nach dem Neustart gilt der durable Stand; fehlt der Eintrag, ist der Antrag erneut
//!    zu stellen.

use std::io;
use std::sync::{Arc, RwLock};

use crate::error::GateError;
use crate::log::{self, GateKey, GateOp, TAG_LEN};
use crate::snapshot;
use crate::state::GateSnapshot;
use crate::view::{GateView, Shared};

/// Port für das Gate-Log. Implementierungen (Datei, Fault-Injektion) liegen außerhalb
/// dieses Crates; der Crate selbst führt keine E/A aus.
pub trait GateSink: Send {
    /// Hängt Bytes an das Log an. Ein Erfolg bedeutet noch keine Haltbarkeit.
    ///
    /// # Errors
    /// Jeder Fehler poisoniert den Mandanten.
    fn append(&mut self, bytes: &[u8]) -> io::Result<()>;

    /// Macht alle angehängten Bytes haltbar (Gruppen-`fsync`, SPEC 3.6).
    ///
    /// # Errors
    /// Ein Fehler ist endgültig für diese Datei (SPEC 3.7); der Mandant ist vergiftet.
    fn sync(&mut self) -> io::Result<()>;

    /// Verdichtung (C23): legt `snapshot` und ein neues Log mit dem Kopfrahmen `log_head`
    /// in neuen Dateien ab, macht beide haltbar, tauscht sie atomar (`rename`) gegen den
    /// alten Stand und archiviert das alte Log. Danach gehen `append`-Aufrufe in das
    /// neue Log. Scheitert der Austausch, gilt der alte Stand weiter; der Aufruf ist
    /// wiederholbar. Der Standard lehnt ab.
    ///
    /// # Errors
    /// Jeder Fehler lässt den alten Stand gültig.
    fn install_snapshot(&mut self, snapshot: &[u8], log_head: &[u8]) -> io::Result<()> {
        let _ = (snapshot, log_head);
        Err(io::Error::from(io::ErrorKind::Unsupported))
    }
}

/// Schreibender Zugriff auf den Sperrzustand eines Mandanten.
pub struct GateWriter {
    key: GateKey,
    shared: Shared,
    sink: Box<dyn GateSink>,
    last_tag: [u8; TAG_LEN],
    next_seq: u64,
    poisoned: bool,
}

impl GateWriter {
    /// Öffnet das Gate aus dem durablen Stand des Logs.
    ///
    /// Eine leere Logdatei ergibt den Anfangszustand. Jeder Kettenbruch ist ein Fehler.
    ///
    /// # Errors
    /// `ChainBroken`, `LogFormat` oder `Mac`, wenn der durable Stand nicht geprüft werden kann.
    pub fn recover(
        key: GateKey,
        durable_log: &[u8],
        sink: Box<dyn GateSink>,
    ) -> Result<(Self, GateView), GateError> {
        Self::recover_with_snapshot(key, None, durable_log, sink)
    }

    /// Öffnet das Gate aus Snapshot (nach einer Verdichtung) und dem anschließenden Log.
    ///
    /// # Errors
    /// Wie `recover`; ein Snapshot mit falschem MAC ist `ChainBroken`.
    pub fn recover_with_snapshot(
        key: GateKey,
        snapshot: Option<&[u8]>,
        durable_log: &[u8],
        sink: Box<dyn GateSink>,
    ) -> Result<(Self, GateView), GateError> {
        let start = snapshot
            .map(|bytes| snapshot::decode(&key, bytes))
            .transpose()?;
        let replayed = log::replay_from(&key, start, durable_log)?;
        let shared: Shared = Arc::new(RwLock::new(Arc::new(replayed.snapshot)));
        let view = GateView::new(Arc::clone(&shared));
        let writer = Self {
            key,
            shared,
            sink,
            last_tag: replayed.last_tag,
            next_seq: replayed.next_seq,
            poisoned: false,
        };
        Ok((writer, view))
    }

    /// Lesegriff auf denselben Stand.
    #[must_use]
    pub fn view(&self) -> GateView {
        GateView::new(Arc::clone(&self.shared))
    }

    /// Wendet einen Stapel atomar an und gibt die neue Epoche zurück.
    ///
    /// # Errors
    /// - `EmptyBatch` bei leerem Stapel; `EraseTerminal`, `NotRestricted`,
    ///   `HighWaterRegression` bei unzulässigen Operationen (ohne Veränderung).
    /// - `Poisoned` nach einem früheren Schreibfehler oder bei nicht lesbarer Sperre.
    /// - `LogWrite` oder `LogSync` bei Fehlern des Logs; der Mandant ist danach vergiftet.
    pub fn commit(&mut self, ops: &[GateOp]) -> Result<u64, GateError> {
        if self.poisoned {
            return Err(GateError::Poisoned);
        }
        if ops.is_empty() {
            return Err(GateError::EmptyBatch);
        }

        let current = self.load_current()?;
        let mut next: GateSnapshot = (*current).clone();
        let mut frames = Vec::new();
        let mut prev = self.last_tag;
        let mut seq = self.next_seq;

        for op in ops {
            next.apply(op, seq)?;
            prev = log::encode_frame(&self.key, &prev, seq, op, &mut frames)?;
            seq = seq.checked_add(1).ok_or(GateError::Poisoned)?;
        }
        let epoch = next.epoch();

        // Schritt 2: Sperre im Arbeitsspeicher, sofort wirksam (Z1, Z4).
        self.publish(next)?;

        // Schritt 3 und 4: haltbar machen. Jeder Fehler vergiftet den Mandanten.
        if self.sink.append(&frames).is_err() {
            self.poisoned = true;
            return Err(GateError::LogWrite);
        }
        if self.sink.sync().is_err() {
            self.poisoned = true;
            return Err(GateError::LogSync);
        }

        self.last_tag = prev;
        self.next_seq = seq;
        Ok(epoch)
    }

    /// Verdichtet das Log (SPEC 5.3, C23): neuer Snapshot, neues Log, atomarer Austausch.
    ///
    /// Gibt die Epoche des Snapshots zurück. Ein Fehler des Sinks lässt den alten Stand
    /// gültig und vergiftet den Mandanten nicht; die Verdichtung ist wiederholbar.
    ///
    /// # Errors
    /// `Poisoned` bei vergiftetem Mandanten, `LogWrite` wenn der Austausch scheitert.
    pub fn compact(&mut self) -> Result<u64, GateError> {
        if self.poisoned {
            return Err(GateError::Poisoned);
        }
        let current = self.load_current()?;
        let epoch = self.next_seq.checked_sub(1).ok_or(GateError::Poisoned)?;
        if current.epoch() != epoch {
            return Err(GateError::Poisoned);
        }
        let (snap_bytes, snap_tag) = snapshot::encode(&self.key, &current, &self.last_tag)?;
        let seq = self.next_seq;
        let mut head = Vec::new();
        let head_tag = log::encode_snapshot_frame(&self.key, &snap_tag, seq, epoch, &mut head)?;
        self.sink
            .install_snapshot(&snap_bytes, &head)
            .map_err(|_| GateError::LogWrite)?;
        self.last_tag = head_tag;
        self.next_seq = seq.checked_add(1).ok_or(GateError::Poisoned)?;
        self.publish_epoch(seq)?;
        Ok(epoch)
    }

    fn publish_epoch(&self, seq: u64) -> Result<(), GateError> {
        let mut next = (*self.load_current()?).clone();
        next.epoch = seq;
        self.publish(next)
    }

    /// Gibt zurück, ob der Mandant vergiftet ist.
    #[must_use]
    pub const fn is_poisoned(&self) -> bool {
        self.poisoned
    }

    fn load_current(&self) -> Result<Arc<GateSnapshot>, GateError> {
        let guard = self.shared.read().map_err(|_| GateError::Poisoned)?;
        Ok(Arc::clone(&*guard))
    }

    fn publish(&self, next: GateSnapshot) -> Result<(), GateError> {
        let mut guard = self.shared.write().map_err(|_| GateError::Poisoned)?;
        *guard = Arc::new(next);
        Ok(())
    }
}

impl std::fmt::Debug for GateWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GateWriter")
            .field("next_seq", &self.next_seq)
            .field("poisoned", &self.poisoned)
            .finish_non_exhaustive()
    }
}
