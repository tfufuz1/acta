//! `SecureBuf`: einziger Speicher für Schlüsselmaterial (SPEC 4.6, 13.1).
//!
//! Jeder Puffer liegt in einer eigenen anonymen, seitenausgerichteten Abbildung. Dadurch
//! teilt kein anderes Objekt eine Seite mit dem Geheimnis, und `munlock` betrifft nur diesen
//! Puffer. Eigenschaften: feste Kapazität, `mlock` (fail-closed), `MADV_DONTDUMP`, Nullung
//! vor dem Entsperren und Freigeben. Kein `Default`, `Clone`, `Debug`, `Serialize`.
//!
//! Annahme [A-PAGE]: Seitengröße 4096 Byte (x86_64, aarch64 mit 4-KiB-Seiten). Der Kern rundet
//! Abbildungen auf seine Seitengröße auf; `rustix::param` (Feature `param`) ist nicht in der
//! Whitelist (SPEC 3.8) und wird daher nicht genutzt.
//!
//! Grenze [A-RAM]: `mlock` verhindert nur das Auslagern; Root-Zugriff auf den Prozessspeicher
//! bleibt ausgenommen (SPEC T8).

use std::ffi::c_void;
use std::ptr::{null_mut, NonNull};

use acta_base::CryptoRng;
use rustix::mm::{madvise, mlock, mmap_anonymous, munlock, munmap, Advice, MapFlags, ProtFlags};
use zeroize::Zeroize;

use crate::error::SysError;

/// Annahme [A-PAGE]: Seitengröße für die Ausrichtung der Abbildung.
const PAGE: usize = 4096;

/// Gesperrter, nullender Speicher fester Länge für Geheimnisse.
pub struct SecureBuf {
    /// Beginn der Abbildung; bei `len == 0` ein Dangling-Zeiger ohne Abbildung.
    ptr: NonNull<u8>,
    /// Nutzbare Länge in Bytes.
    len: usize,
    /// Länge der seitenausgerichteten Abbildung; 0, wenn keine Abbildung existiert.
    map_len: usize,
    /// `true`, wenn `mlock` erfolgreich war und `Drop` entsperren muss.
    locked: bool,
}

// SAFETY: `SecureBuf` besitzt seine Abbildung exklusiv, ohne geteilte Aliasse. Der Zugriff
// folgt den Regeln für `Box<[u8]>`, daher ist Senden und gemeinsamer Lesezugriff zulässig.
unsafe impl Send for SecureBuf {}
// SAFETY: siehe `Send`; gemeinsamer Zugriff gibt nur `&[u8]` frei, `&mut` erfordert `&mut self`.
unsafe impl Sync for SecureBuf {}

impl SecureBuf {
    /// Nullgefüllter Puffer der Länge `len`. Scheitert, wenn die Seiten nicht gesperrt werden können.
    ///
    /// # Errors
    /// `SysError::Map` bei fehlgeschlagener Abbildung, `SysError::Mlock` oder `SysError::DontDump`
    /// bei Verweigerung durch das Betriebssystem.
    pub fn zeroed(len: usize) -> Result<Self, SysError> {
        if len == 0 {
            return Ok(Self {
                ptr: NonNull::dangling(),
                len: 0,
                map_len: 0,
                locked: false,
            });
        }
        let map_len = len.checked_next_multiple_of(PAGE).ok_or(SysError::Map)?;
        // SAFETY: Anonyme Abbildung ohne feste Adresse; der Kernel liefert zeroed Seiten und
        // gibt sie nur über `munmap` frei, das in `Drop` mit derselben Länge aufgerufen wird.
        let raw = unsafe {
            mmap_anonymous(
                null_mut(),
                map_len,
                ProtFlags::READ | ProtFlags::WRITE,
                MapFlags::PRIVATE,
            )
        }
        .map_err(|_| SysError::Map)?;
        let ptr = NonNull::new(raw.cast::<u8>()).ok_or(SysError::Map)?;
        let mut out = Self {
            ptr,
            len,
            map_len,
            locked: false,
        };
        // Schlägt die Sperre fehl, räumt `Drop` die Abbildung nach Nullung auf.
        out.lock_pages()?;
        Ok(out)
    }

    /// Kopie von `src` in einen gesperrten Puffer. Der Quellpuffer wird nicht genullt; das ist
    /// Aufgabe des Aufrufers.
    ///
    /// # Errors
    /// Wie [`SecureBuf::zeroed`].
    pub fn from_slice(src: &[u8]) -> Result<Self, SysError> {
        let mut out = Self::zeroed(src.len())?;
        out.as_mut_slice().copy_from_slice(src);
        Ok(out)
    }

    /// Puffer mit Zufall aus `rng`. Ein Entropiefehler liefert `Err`, nie einen Puffer mit Ersatzwerten (Z10).
    ///
    /// # Errors
    /// `SysError::Entropy` bei Fehler der Quelle, sonst wie [`SecureBuf::zeroed`].
    pub fn random(len: usize, rng: &dyn CryptoRng) -> Result<Self, SysError> {
        let mut out = Self::zeroed(len)?;
        rng.try_fill(out.as_mut_slice())
            .map_err(SysError::Entropy)?;
        Ok(out)
    }

    /// Länge in Bytes (fest seit der Erzeugung).
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// `true`, wenn die Kapazität 0 ist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Lesezugriff auf das Geheimnis.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        // SAFETY: `ptr` ist für `len` initialisierte Bytes gültig (Abbildung oder Dangling bei len 0),
        // und `&self` schließt gleichzeitige Schreibzugriffe aus.
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    /// Schreibzugriff auf das Geheimnis.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        // SAFETY: wie bei `as_slice`; `&mut self` ist der einzige Zugriffspfad.
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }

    fn lock_pages(&mut self) -> Result<(), SysError> {
        if self.map_len == 0 {
            return Ok(());
        }
        let base: *mut c_void = self.ptr.as_ptr().cast();
        // SAFETY: `base..base+map_len` ist die eigene, seitenausgerichtete Abbildung dieses Objekts.
        unsafe { mlock(base, self.map_len) }.map_err(|_| SysError::Mlock)?;
        self.locked = true;
        // SAFETY: gleiche Region, seitenausgerichtet, eigene Abbildung; `MADV_DONTDUMP` ändert nur Core-Dumps.
        unsafe { madvise(base, self.map_len, Advice::LinuxDontDump) }
            .map_err(|_| SysError::DontDump)?;
        Ok(())
    }
}

impl Drop for SecureBuf {
    fn drop(&mut self) {
        self.as_mut_slice().zeroize();
        if self.map_len == 0 {
            return;
        }
        let base: *mut c_void = self.ptr.as_ptr().cast();
        if self.locked {
            // SAFETY: gleiche Region wie in `lock_pages`, seit der Sperre unverändert. Ein Fehler
            // beim Entsperren kann im Drop nicht gemeldet werden; die Nullung ist bereits erfolgt.
            unsafe { munlock(base, self.map_len) }.ok();
        }
        // SAFETY: gleiche Abbildung wie bei `mmap_anonymous`; danach wird `ptr` nie mehr benutzt.
        unsafe { munmap(base, self.map_len) }.ok();
    }
}
