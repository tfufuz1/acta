//! Geheime Schlüssel (Spec 4.2, 4.6, Z10).
//!
//! Geheimnisse haben kein `Default`, `Clone`, `Debug` und keinen `Serialize`. Das Material
//! liegt in `Zeroizing` und wird beim Verwerfen überschrieben. Jeder Schlüsseltyp ist ein
//! eigener Typ, damit etwa ein `SubjectKey` nie als `DocumentKey` übergeben werden kann.
//! Rohes Schlüsselmaterial verlässt das Crate nie. Außen gibt es nur den AEAD-Cipher
//! ([`crate::aead::SealKey`]) und die Ableitungen in [`crate::kdf`].

use crate::error::CryptoError;
use crate::mac::{hmac_sha256, verify_hmac_sha256};
use acta_base::CryptoRng;
use zeroize::Zeroizing;

/// Länge eines Schlüssels in Byte (256 Bit).
pub const KEY_LEN: usize = 32;

/// Versiegelter Supertrait: nur Schlüsseltypen dieses Crates implementieren [`crate::aead::SealKey`].
#[doc(hidden)]
pub mod private {
    /// Marker für die Verschlossenheit von `SealKey`.
    pub trait Sealed {}
}

macro_rules! secret_core {
    ($(#[$doc:meta])* $name:ident) => {
        $(#[$doc])*
        pub struct $name {
            bytes: Zeroizing<[u8; KEY_LEN]>,
        }

        impl $name {
            /// Erzeugt einen frischen Schlüssel aus dem Zufall des Ports.
            ///
            /// # Errors
            /// [`CryptoError::Entropy`], wenn die Quelle versagt. Dann entsteht kein Schlüssel.
            pub fn generate(rng: &dyn CryptoRng) -> Result<Self, CryptoError> {
                let mut bytes = Zeroizing::new([0_u8; KEY_LEN]);
                rng.try_fill(bytes.as_mut_slice())?;
                Ok(Self { bytes })
            }

            /// Übernimmt 32 Byte, etwa beim Laden aus dem Tresor. Der Aufrufer ist für die Herkunft verantwortlich.
            #[must_use]
            pub fn from_bytes(bytes: [u8; KEY_LEN]) -> Self {
                Self {
                    bytes: Zeroizing::new(bytes),
                }
            }

            /// Schlüsselmaterial für Ableitungen und Cipher in diesem Crate.
            pub(crate) fn expose(&self) -> &[u8] {
                self.bytes.as_slice()
            }
        }

        impl private::Sealed for $name {}

        // Das Material liegt in `Zeroizing`, der Typ wird beim Verwerfen überschrieben.
        impl zeroize::ZeroizeOnDrop for $name {}
    };
}

/// Schlüssel, der AEAD-Umschläge versiegeln darf.
macro_rules! secret_key {
    ($(#[$doc:meta])* $name:ident) => {
        secret_core!($(#[$doc])* $name);

        impl crate::aead::SealKey for $name {
            fn cipher(&self) -> Result<chacha20poly1305::XChaCha20Poly1305, CryptoError> {
                use chacha20poly1305::KeyInit;
                chacha20poly1305::XChaCha20Poly1305::new_from_slice(self.expose())
                    .map_err(|_| CryptoError::Internal)
            }
        }
    };
}

/// Schlüssel nur für MAC und Ableitungen. Kein `SealKey`: er kann keinen Umschlag versiegeln.
macro_rules! mac_key {
    ($(#[$doc:meta])* $name:ident) => {
        secret_core!($(#[$doc])* $name);
    };
}

secret_key!(
    /// Datenschlüssel (DEK) einer Dokument-Version, Umschlag-Schlüssel der Ebene P.
    DataKey
);
secret_key!(
    /// Schlüssel einer Betroffenen-Person (`SubjectKey`).
    SubjectKey
);
secret_key!(
    /// Schlüssel eines Dokuments über alle Versionen (`DocumentKey`).
    DocumentKey
);
secret_key!(
    /// Schlüssel eines Segments (`SegmentKey`).
    SegmentKey
);
secret_key!(
    /// Schlüssel eines Löschvorgangs (`ScopeKey`, Spec 4.2, 8.2). Kurzlebig: versiegelt die
    /// Arbeitsdaten (Keyspaces `0x0040`, `0x0060`) und bildet die Messmuster (8.5).
    ScopeKey
);
secret_key!(
    /// Generationsgeheimnis des Schlüsselankers (`MasterKey`, Spec 4.2).
    MasterKey
);
secret_key!(
    /// Geheimnis für Tags und Dedup-Salze (`k_idx`, Spec 2.4). Ohne Standardwert.
    IndexSecret
);

secret_key!(
    /// Zugriffsprotokoll-Schlüssel `LogKey(m)` je Monat (Spec 4.2, 6.5). Ablauf der Frist vernichtet ihn.
    LogKey
);
mac_key!(
    /// Commitment-Schlüssel `CommitKey(e)` je Epoche (Spec 2.7, 4.2). Nur für HMAC.
    CommitKey
);
mac_key!(
    /// Dedup-Schlüssel `K_dedup(t)` je Mandant (Spec 2.4, 4.2). Nur für das Blob-Salz.
    DedupKey
);

impl ScopeKey {
    /// HMAC-SHA256 über die konkatenierten `parts` unter dem `ScopeKey`.
    ///
    /// Die Messmuster nach Spec 8.5 sind `HMAC(ScopeKey, 32-Byte-Fenster)`, also
    /// `key.mac(&[fenster])`. Das Schlüsselmaterial verlässt das Crate dabei nicht.
    ///
    /// # Errors
    /// [`CryptoError::Internal`] nur bei internem Fehler der HMAC-Berechnung.
    pub fn mac(&self, parts: &[&[u8]]) -> Result<[u8; 32], CryptoError> {
        hmac_sha256(self.expose(), parts)
    }

    /// Prüft ein HMAC-Tag unter dem `ScopeKey` in konstanter Zeit. Ungleiche Längen ergeben `false`.
    ///
    /// # Errors
    /// Wie [`ScopeKey::mac`].
    pub fn verify_mac(&self, parts: &[&[u8]], tag: &[u8]) -> Result<bool, CryptoError> {
        verify_hmac_sha256(self.expose(), parts, tag)
    }
}

impl DedupKey {
    /// Abgeleiteter Dedup-Schlüssel aus einer Ableitung in diesem Crate.
    pub(crate) fn from_derived(bytes: Zeroizing<[u8; KEY_LEN]>) -> Self {
        Self { bytes }
    }
}

impl DataKey {
    /// Abgeleiteter Datenschlüssel aus einer Ableitung in diesem Crate.
    pub(crate) fn from_derived(bytes: Zeroizing<[u8; KEY_LEN]>) -> Self {
        Self { bytes }
    }
}
