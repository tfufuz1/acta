//! Identifikatoren (Spec 2.4, 6.2). Ordinale sind dicht, nie wiederverwendet und nur intern sichtbar.

use crate::ports::{CryptoRng, EntropyError};

macro_rules! handle16 {
    ($(#[$m:meta])* $name:ident) => {
        $(#[$m])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(pub [u8; 16]);
        impl $name {
            /// Aus Rohbytes.
            #[must_use]
            pub const fn from_bytes(b: [u8; 16]) -> Self {
                Self(b)
            }

            /// Rohbytes.
            #[must_use]
            pub fn as_bytes(&self) -> &[u8; 16] {
                &self.0
            }
        }
        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                // Kein Inhalt im Log (Z15).
                f.write_str(concat!(stringify!($name), "(..)"))
            }
        }
    };
}

// Zufaellige Bildung (SPEC 2.4) nur fuer Handles, deren Bildung "zufaellig" lautet.
macro_rules! random_handle {
    ($name:ident) => {
        impl $name {
            /// Zufaelliges Handle ohne Inhaltsbezug (Z10: ein Entropiefehler liefert nie einen
            /// Ersatzwert).
            ///
            /// # Errors
            /// `EntropyError`, wenn die Quelle nicht liefern kann.
            pub fn random(rng: &dyn CryptoRng) -> Result<Self, EntropyError> {
                let mut b = [0_u8; 16];
                rng.try_fill(&mut b)?;
                Ok(Self(b))
            }
        }
    };
}

macro_rules! handle32 {
    ($(#[$m:meta])* $name:ident) => {
        $(#[$m])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(pub [u8; 32]);
        impl $name {
            /// Aus Rohbytes.
            #[must_use]
            pub const fn from_bytes(b: [u8; 32]) -> Self {
                Self(b)
            }

            /// Rohbytes.
            #[must_use]
            pub fn as_bytes(&self) -> &[u8; 32] {
                &self.0
            }
        }
        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                // Kein Inhalt im Log (Z15).
                f.write_str(concat!(stringify!($name), "(..)"))
            }
        }
    };
}

macro_rules! ordinal {
    ($(#[$m:meta])* $name:ident, $t:ty) => {
        $(#[$m])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
        pub struct $name(pub $t);
        impl $name {
            /// Naechstes Ordinal; `None` bei Ueberlauf (Vergiftung `Node`, Spec 2.4).
            #[must_use]
            pub fn next(self) -> Option<Self> {
                self.0.checked_add(1).map(Self)
            }
        }
    };
}

handle16!(/// Mandantenkennung (zufaellig, nicht geheim).
    TenantId);
handle16!(/// Dokumentidentitaet in P (stabil ueber Versionen).
    DocRef);
handle16!(/// Version eines P-Dokuments (Tresor-ID des `DocumentKey`).
    VerRef);
handle16!(/// Zufaelliges Handle aus K auf P.
    PersonalRef);
handle16!(/// Vorgang.
    CaseId);
handle16!(/// Zufaelliges Handle auf eine Identitaet in P.
    ActorRef);
handle16!(/// Loeschvorgang (UUIDv7, Bildung ueber `from_v7`).
    ErasureId);
handle16!(/// Schluessel-Tag eines Betroffenen (SPEC 2.4): HMAC, nur als versiegelte Tresor-ID und im Verzeichnis. Bildung in `acta-crypto`.
    SubjectTag);
handle16!(/// Lokalitaetsgruppe in Segmenten (SPEC 2.4): nicht gespeichert, nur mit `SubjectKey` berechenbar. Bildung in `acta-crypto`.
    ClusterId);
handle16!(/// Index-Generation (SPEC 2.4, 9.2): Grundlage eines Indexsatzes. Bildung bei der Index-Generation (11.4).
    IndexGen);

random_handle!(TenantId);
random_handle!(DocRef);
random_handle!(VerRef);
random_handle!(PersonalRef);
random_handle!(CaseId);
random_handle!(ActorRef);

handle32!(/// Objekt-Identitaet der Ebene K (SPEC 2.4): SHA-256 ueber kanonische Bytes, 32 Byte.
    ObjectId);
handle32!(/// Kettenkennung (SPEC 6.2): 32 Byte. Die Bildung `SHA-256(TenantId || Kennbuchstabe)` liegt in `acta-ledger`.
    ChainId);

ordinal!(/// Dichte Mandantennummer; 0 ist fuer Systeminterna reserviert.
    TenantOrd, u32);

impl TenantOrd {
    /// Reserviertes Ordinal fuer Systeminterna.
    pub const SYSTEM: Self = Self(0);

    /// `true` fuer das reservierte Systemordinal.
    #[must_use]
    pub const fn is_system(self) -> bool {
        self.0 == 0
    }
}

impl ErasureId {
    /// UUIDv7 (RFC 9562) aus Millisekundenzeit und 74 Zufallsbits (`rand` liefert 10 Bytes,
    /// genutzt werden 74 Bit). `None`, wenn `ms` nicht in 48 Bit passt.
    #[must_use]
    pub fn from_v7(ms: u64, rand: [u8; 10]) -> Option<Self> {
        let [hi0, hi1, t0, t1, t2, t3, t4, t5] = ms.to_be_bytes();
        if hi0 != 0 || hi1 != 0 {
            return None;
        }
        let [r0, r1, r2, r3, r4, r5, r6, r7, r8, r9] = rand;
        Some(Self([
            t0,
            t1,
            t2,
            t3,
            t4,
            t5,
            0x70 | (r0 & 0x0f),
            r1,
            0x80 | (r2 & 0x3f),
            r3,
            r4,
            r5,
            r6,
            r7,
            r8,
            r9,
        ]))
    }

    /// Zeitanteil in Millisekunden (aus einer UUIDv7).
    #[must_use]
    pub fn v7_ms(&self) -> u64 {
        let [t0, t1, t2, t3, t4, t5, ..] = self.0;
        u64::from_be_bytes([0, 0, t0, t1, t2, t3, t4, t5])
    }
}
ordinal!(/// Dichte Betroffenennummer je Mandant.
    SubjOrd, u32);
ordinal!(/// Dichte Versionsnummer je Mandant.
    VerOrd, u32);
ordinal!(/// Dichte K-Objektnummer je Mandant.
    KOrd, u64);

#[cfg(test)]
mod tests {
    use super::*;

    // SPEC: 2.4 (Ordinale)
    #[test]
    fn ordinal_overflow_is_none() {
        assert_eq!(VerOrd(u32::MAX).next(), None);
        assert_eq!(VerOrd(1).next(), Some(VerOrd(2)));
    }

    // SPEC: 2.4 (TenantOrd)
    #[test]
    fn tenant_ord_zero_is_system() {
        assert!(TenantOrd::SYSTEM.is_system());
        assert!(!TenantOrd(1).is_system());
    }

    // SPEC: 2.4 (ErasureId, UUIDv7)
    #[test]
    fn uuid_v7_layout() {
        let id = ErasureId::from_v7(0x0123_4567_89ab, [0xff; 10]);
        let id = id.map(|i| i.0);
        assert_eq!(
            id,
            Some([
                0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0x7f, 0xff, 0xbf, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff
            ])
        );
        assert_eq!(ErasureId::from_v7(1 << 48, [0; 10]), None);
        let e = ErasureId::from_v7(1_700_000_000_000, [1; 10]);
        assert_eq!(e.map(|i| i.v7_ms()), Some(1_700_000_000_000));
    }

    // SPEC: 2.4 (ErasureId)
    #[test]
    fn uuid_v7_is_time_ordered() {
        let a = ErasureId::from_v7(10, [0xff; 10]);
        let b = ErasureId::from_v7(11, [0; 10]);
        assert!(a < b);
    }

    // SPEC: 2.4, Z10
    #[test]
    fn random_handle_uses_port_and_fails_closed() {
        struct Fixed(u8);
        impl crate::ports::CryptoRng for Fixed {
            fn try_fill(&self, d: &mut [u8]) -> Result<(), crate::ports::EntropyError> {
                d.fill(self.0);
                Ok(())
            }
        }
        struct Broken;
        impl crate::ports::CryptoRng for Broken {
            fn try_fill(&self, _: &mut [u8]) -> Result<(), crate::ports::EntropyError> {
                Err(crate::ports::EntropyError)
            }
        }
        assert_eq!(DocRef::random(&Fixed(9)), Ok(DocRef([9; 16])));
        assert_eq!(TenantId::random(&Fixed(1)), Ok(TenantId([1; 16])));
        assert_eq!(VerRef::random(&Fixed(2)), Ok(VerRef([2; 16])));
        assert_eq!(PersonalRef::random(&Fixed(3)), Ok(PersonalRef([3; 16])));
        assert_eq!(CaseId::random(&Fixed(4)), Ok(CaseId([4; 16])));
        assert_eq!(ActorRef::random(&Fixed(5)), Ok(ActorRef([5; 16])));
        assert_eq!(TenantId::random(&Broken), Err(crate::ports::EntropyError));
        assert_eq!(ActorRef::random(&Broken), Err(crate::ports::EntropyError));
        assert_eq!(DocRef::random(&Broken), Err(crate::ports::EntropyError));
        assert_eq!(DocRef::from_bytes([3; 16]).as_bytes(), &[3; 16]);
    }

    // SPEC: Z15
    #[test]
    fn debug_hides_content() {
        let s = format!("{:?}", TenantId([7; 16]));
        assert!(!s.contains('7'));
    }
}

// SPEC: 2.4 (Identifikatoren, die acta-base als Newtypes führt). Rot, bis die Typen existieren.
#[cfg(test)]
mod spec_2_4_newtypes {
    use super::*;

    #[test]
    fn subject_tag_cluster_index_gen_are_16_byte_handles() {
        let t = SubjectTag::from_bytes([1; 16]);
        assert_eq!(t.as_bytes(), &[1; 16]);
        assert_eq!(ClusterId::from_bytes([2; 16]).as_bytes(), &[2; 16]);
        assert_eq!(IndexGen::from_bytes([3; 16]).as_bytes(), &[3; 16]);
    }

    #[test]
    fn object_id_is_32_bytes_and_hides_content_in_debug() {
        let o = ObjectId::from_bytes([7; 32]);
        assert_eq!(o.as_bytes(), &[7; 32]);
        assert!(!format!("{o:?}").contains('7'));
    }
}

// SPEC: 2.4, 6.2 (Groessen und Debug aller Handles)
#[cfg(test)]
mod spec_2_4_sizes {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn handles_are_16_bytes_and_object_and_chain_ids_are_32() {
        assert_eq!(size_of::<TenantId>(), 16);
        assert_eq!(size_of::<DocRef>(), 16);
        assert_eq!(size_of::<VerRef>(), 16);
        assert_eq!(size_of::<PersonalRef>(), 16);
        assert_eq!(size_of::<CaseId>(), 16);
        assert_eq!(size_of::<ActorRef>(), 16);
        assert_eq!(size_of::<ErasureId>(), 16);
        assert_eq!(size_of::<SubjectTag>(), 16);
        assert_eq!(size_of::<ClusterId>(), 16);
        assert_eq!(size_of::<IndexGen>(), 16);
        assert_eq!(size_of::<ObjectId>(), 32);
        assert_eq!(size_of::<ChainId>(), 32);
    }

    #[test]
    fn every_handle_debug_hides_content() {
        let all = [
            format!("{:?}", TenantId([7; 16])),
            format!("{:?}", DocRef([7; 16])),
            format!("{:?}", VerRef([7; 16])),
            format!("{:?}", PersonalRef([7; 16])),
            format!("{:?}", CaseId([7; 16])),
            format!("{:?}", ActorRef([7; 16])),
            format!("{:?}", ErasureId([7; 16])),
            format!("{:?}", SubjectTag([7; 16])),
            format!("{:?}", ClusterId([7; 16])),
            format!("{:?}", IndexGen([7; 16])),
            format!("{:?}", ObjectId([7; 32])),
            format!("{:?}", ChainId([7; 32])),
        ];
        for s in all {
            assert!(!s.contains('7'), "{s}");
            assert!(s.ends_with("(..)"), "{s}");
        }
        assert_eq!(format!("{:?}", ChainId([0; 32])), "ChainId(..)");
    }

    #[test]
    fn chain_id_roundtrips_bytes_and_orders_bytewise() {
        let c = ChainId::from_bytes([9; 32]);
        assert_eq!(c.as_bytes(), &[9; 32]);
        assert!(ChainId([1; 32]) < ChainId([2; 32]));
        assert_ne!(ChainId([1; 32]), ChainId([2; 32]));
    }

    // Positivkontrolle (16.4): die Monotonie-Pruefung kann scheitern.
    #[test]
    fn ordinals_are_strictly_monotone_and_none_at_max() {
        assert_eq!(TenantOrd::SYSTEM.next(), Some(TenantOrd(1)));
        assert_eq!(TenantOrd(u32::MAX).next(), None);
        assert_eq!(SubjOrd(u32::MAX).next(), None);
        assert_eq!(KOrd(u64::MAX).next(), None);
        assert_eq!(KOrd(u64::MAX - 1).next(), Some(KOrd(u64::MAX)));
        let mut o = SubjOrd(0);
        for _ in 0..1000 {
            let n = o.next();
            assert!(n.is_some_and(|n| n > o));
            o = n.unwrap_or(o);
        }
        assert_eq!(o, SubjOrd(1000));
        assert!(VerOrd(2) > VerOrd(1));
    }
}
