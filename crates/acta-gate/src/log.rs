//! Gate-Log (SPEC 5.3): Rahmen `[seq u64][op u8][len u16][payload][chain_tag 32]`,
//! HMAC-SHA256-Kette über `prev_tag ‖ header ‖ payload`.
//!
//! Dieses Modul ist rein: es kennt keine Dateien. Der Schlüssel `GateKey` wird vom
//! Aufrufer aus `HKDF(k_idx, "acta/gate/v1" ‖ TenantId)` abgeleitet (SPEC 5.3); die
//! Ableitung liegt in `acta-crypto`, das auf derselben Ebene wie dieser Crate liegt
//! und daher nicht importiert werden darf (G1).
//!
//! Die Kodierung ist von Hand festgelegt (G-NOSERDE-DISK): Big-Endian, feste Längen.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use zeroize::Zeroize;

use acta_base::ids::{KOrd, SubjOrd, VerOrd};

use crate::error::GateError;
use crate::reasons::{EraseReason, RestrictReason};
use crate::state::GateSnapshot;

pub(crate) type HmacSha256 = Hmac<Sha256>;

/// Länge des Kettenetiketts in Bytes.
pub const TAG_LEN: usize = 32;
/// Kopfgröße: `seq` (8) + `op` (1) + `len` (2).
const HEADER_LEN: usize = 11;
/// Obergrenze der Nutzlast je Rahmen (`len` ist u16).
const MAX_PAYLOAD: usize = u16::MAX as usize;

const OP_ERASE_SUBJECT: u8 = 0x01;
const OP_ERASE_VER: u8 = 0x02;
const OP_RESTRICT: u8 = 0x03;
const OP_RELEASE: u8 = 0x04;
const OP_SUPERSEDE: u8 = 0x05;
const OP_REDACT_K: u8 = 0x06;
const OP_HIGH_WATER: u8 = 0x07;
const OP_SNAPSHOT: u8 = 0x08;

/// Schlüssel der HMAC-Kette. Wird beim Verwerfen gelöscht; kein `Debug`, kein `Clone` (Z15).
pub struct GateKey([u8; 32]);

impl GateKey {
    /// Neuer HMAC-Zustand unter diesem Schlüssel.
    pub(crate) fn mac(&self) -> Result<HmacSha256, GateError> {
        HmacSha256::new_from_slice(&self.0).map_err(|_| GateError::Mac)
    }

    /// Übernimmt einen bereits abgeleiteten Schlüssel.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl Drop for GateKey {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

/// Art eines Ordinal-Hochwassers (`HighWater`, Rahmen 0x07).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HighWaterKind {
    /// Höchste vergebene `VerOrd`.
    Ver,
    /// Höchste vergebene `SubjOrd`.
    Subj,
    /// Höchste vergebene `KOrd`.
    K,
}

impl HighWaterKind {
    /// Position im Zustandsfeld.
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Ver => 0,
            Self::Subj => 1,
            Self::K => 2,
        }
    }

    const fn code(self) -> u8 {
        match self {
            Self::Ver => 1,
            Self::Subj => 2,
            Self::K => 3,
        }
    }

    const fn from_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Ver),
            2 => Some(Self::Subj),
            3 => Some(Self::K),
            _ => None,
        }
    }
}

/// Eine Operation des Gate-Logs (SPEC 5.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum GateOp {
    /// `0x01`: Betroffener, laufende oder abgeschlossene Löschung.
    EraseSubject(SubjOrd),
    /// `0x02`: Version in `Gated`, `Expired` oder erledigt.
    EraseVer {
        /// Betroffene Version.
        ver: VerOrd,
        /// Auslöser (nur im Log, nicht im Zustand).
        reason: EraseReason,
    },
    /// `0x03`: Einschränkung einer Version.
    Restrict {
        /// Betroffene Version.
        ver: VerOrd,
        /// Grund der Einschränkung.
        reason: RestrictReason,
    },
    /// `0x04`: Aufhebung einer Einschränkung (nur aus `Restricted`).
    Release(VerOrd),
    /// `0x05`: Version ist überholt.
    Supersede(VerOrd),
    /// `0x06`: Takedown eines K-Objekts.
    RedactK(KOrd),
    /// `0x07`: Hochwasser eines Ordinals; monoton.
    HighWater {
        /// Ordinalart.
        kind: HighWaterKind,
        /// Neuer Stand; darf nicht unter dem bisherigen liegen.
        value: u64,
    },
}

/// Ein dekodierter Rahmen, der mit seiner Position im Puffer verbunden ist.
struct Frame<'a> {
    seq: u64,
    op: u8,
    header: &'a [u8],
    payload: &'a [u8],
    tag: [u8; TAG_LEN],
    rest: &'a [u8],
}

/// Ergebnis des Replays.
pub struct Replayed {
    /// Stand nach allen gültigen Rahmen.
    pub snapshot: GateSnapshot,
    /// Letztes Kettenetikett (Startwert der nächsten Kette).
    pub last_tag: [u8; TAG_LEN],
    /// Nächste freie Sequenznummer.
    pub next_seq: u64,
}

/// Rahmen-Kette: HMAC über `prev ‖ header ‖ payload`.
fn chain_mac(
    key: &GateKey,
    prev: &[u8; TAG_LEN],
    header: &[u8],
    payload: &[u8],
) -> Result<HmacSha256, GateError> {
    let mut mac = HmacSha256::new_from_slice(&key.0).map_err(|_| GateError::Mac)?;
    mac.update(prev);
    mac.update(header);
    mac.update(payload);
    Ok(mac)
}

/// Kodiert einen Rahmen und hängt ihn an `out` an. Gibt das neue Kettenetikett zurück.
pub(crate) fn encode_frame(
    key: &GateKey,
    prev: &[u8; TAG_LEN],
    seq: u64,
    op: &GateOp,
    out: &mut Vec<u8>,
) -> Result<[u8; TAG_LEN], GateError> {
    let (code, payload) = encode_op(op);
    if payload.len() > MAX_PAYLOAD {
        return Err(GateError::LogFormat);
    }
    let len = u16::try_from(payload.len()).map_err(|_| GateError::LogFormat)?;

    let mut header = Vec::with_capacity(HEADER_LEN);
    header.extend_from_slice(&seq.to_be_bytes());
    header.push(code);
    header.extend_from_slice(&len.to_be_bytes());

    let tag: [u8; TAG_LEN] = chain_mac(key, prev, &header, &payload)?
        .finalize()
        .into_bytes()
        .into();

    out.extend_from_slice(&header);
    out.extend_from_slice(&payload);
    out.extend_from_slice(&tag);
    Ok(tag)
}

/// Zerlegt den Anfang eines Puffers in einen Rahmen. `None`, wenn der Puffer zu kurz ist
/// (abgeschnittener Schreibvorgang am Ende; nie bestätigt, daher ignoriert).
fn split_frame(buf: &[u8]) -> Option<Frame<'_>> {
    let (header, body) = buf.split_at_checked(HEADER_LEN)?;
    let (seq_bytes, op_len) = header.split_at_checked(8)?;
    let (op_bytes, len_bytes) = op_len.split_at_checked(1)?;
    let seq = u64::from_be_bytes(<[u8; 8]>::try_from(seq_bytes).ok()?);
    let op = *op_bytes.first()?;
    let len = usize::from(u16::from_be_bytes(<[u8; 2]>::try_from(len_bytes).ok()?));
    let (payload, after_payload) = body.split_at_checked(len)?;
    let (tag_bytes, rest) = after_payload.split_at_checked(TAG_LEN)?;
    Some(Frame {
        seq,
        op,
        header,
        payload,
        tag: <[u8; TAG_LEN]>::try_from(tag_bytes).ok()?,
        rest,
    })
}

/// Spielt das Log ab. Prüft jede Kette und Sequenz; ein abgeschnittener letzter
/// Rahmen wird verworfen, ein Kettenbruch ist ein Integritätsfehler.
/// Kopfrahmen `0x08` nach einer Verdichtung: bindet das Log an den Snapshot (C23).
pub(crate) fn encode_snapshot_frame(
    key: &GateKey,
    prev: &[u8; TAG_LEN],
    seq: u64,
    epoch: u64,
    out: &mut Vec<u8>,
) -> Result<[u8; TAG_LEN], GateError> {
    let payload = epoch.to_be_bytes();
    let len = u16::try_from(payload.len()).map_err(|_| GateError::LogFormat)?;
    let mut header = Vec::with_capacity(HEADER_LEN);
    header.extend_from_slice(&seq.to_be_bytes());
    header.push(OP_SNAPSHOT);
    header.extend_from_slice(&len.to_be_bytes());
    let tag: [u8; TAG_LEN] = chain_mac(key, prev, &header, &payload)?
        .finalize()
        .into_bytes()
        .into();
    out.extend_from_slice(&header);
    out.extend_from_slice(&payload);
    out.extend_from_slice(&tag);
    Ok(tag)
}

/// Spielt ab einem geprüften Snapshot (oder vom Anfang). Ein Rahmen `0x08` ist nur als
/// erster Rahmen nach einem Snapshot zulässig und muss dessen Epoche nennen.
pub(crate) fn replay_from(
    key: &GateKey,
    start: Option<(GateSnapshot, [u8; TAG_LEN])>,
    log: &[u8],
) -> Result<Replayed, GateError> {
    let has_start = start.is_some();
    let (mut snapshot, mut prev) = start.unwrap_or_default();
    let base_epoch = snapshot.epoch();
    let mut expected_seq = base_epoch.checked_add(1).ok_or(GateError::LogFormat)?;
    let mut first = true;
    let mut rest = log;

    while !rest.is_empty() {
        let Some(frame) = split_frame(rest) else {
            break;
        };
        if frame.seq != expected_seq {
            return Err(GateError::ChainBroken { seq: frame.seq });
        }
        chain_mac(key, &prev, frame.header, frame.payload)?
            .verify_slice(&frame.tag)
            .map_err(|_| GateError::ChainBroken { seq: frame.seq })?;

        if frame.op == OP_SNAPSHOT {
            let mut c = Cursor(frame.payload);
            let epoch = c.u64()?;
            c.finish()?;
            if !(first && has_start && epoch == base_epoch) {
                return Err(GateError::LogFormat);
            }
            snapshot.epoch = frame.seq;
        } else {
            let op = decode_op(frame.op, frame.payload)?;
            snapshot
                .apply(&op, frame.seq)
                .map_err(|_| GateError::LogFormat)?;
        }

        first = false;
        prev = frame.tag;
        expected_seq = expected_seq.checked_add(1).ok_or(GateError::LogFormat)?;
        rest = frame.rest;
    }

    Ok(Replayed {
        snapshot,
        last_tag: prev,
        next_seq: expected_seq,
    })
}

fn encode_op(op: &GateOp) -> (u8, Vec<u8>) {
    let mut payload = Vec::with_capacity(5);
    let code = match *op {
        GateOp::EraseSubject(subject) => {
            payload.extend_from_slice(&subject.0.to_be_bytes());
            OP_ERASE_SUBJECT
        }
        GateOp::EraseVer { ver, reason } => {
            payload.extend_from_slice(&ver.0.to_be_bytes());
            payload.push(reason.code());
            OP_ERASE_VER
        }
        GateOp::Restrict { ver, reason } => {
            payload.extend_from_slice(&ver.0.to_be_bytes());
            payload.push(reason.code());
            OP_RESTRICT
        }
        GateOp::Release(ver) => {
            payload.extend_from_slice(&ver.0.to_be_bytes());
            OP_RELEASE
        }
        GateOp::Supersede(ver) => {
            payload.extend_from_slice(&ver.0.to_be_bytes());
            OP_SUPERSEDE
        }
        GateOp::RedactK(k) => {
            payload.extend_from_slice(&k.0.to_be_bytes());
            OP_REDACT_K
        }
        GateOp::HighWater { kind, value } => {
            payload.push(kind.code());
            payload.extend_from_slice(&value.to_be_bytes());
            OP_HIGH_WATER
        }
    };
    (code, payload)
}

/// Lesender Zeiger über eine Nutzlast; jede Lesung prüft die Länge.
pub(crate) struct Cursor<'a>(pub(crate) &'a [u8]);

impl Cursor<'_> {
    pub(crate) fn take<const N: usize>(&mut self) -> Result<[u8; N], GateError> {
        let (head, rest) = self.0.split_at_checked(N).ok_or(GateError::LogFormat)?;
        self.0 = rest;
        <[u8; N]>::try_from(head).map_err(|_| GateError::LogFormat)
    }

    pub(crate) fn u8(&mut self) -> Result<u8, GateError> {
        let [b] = self.take::<1>()?;
        Ok(b)
    }

    pub(crate) fn u32(&mut self) -> Result<u32, GateError> {
        Ok(u32::from_be_bytes(self.take::<4>()?))
    }

    pub(crate) fn u64(&mut self) -> Result<u64, GateError> {
        Ok(u64::from_be_bytes(self.take::<8>()?))
    }

    pub(crate) fn bytes(&mut self, n: usize) -> Result<&[u8], GateError> {
        let (head, rest) = self.0.split_at_checked(n).ok_or(GateError::LogFormat)?;
        self.0 = rest;
        Ok(head)
    }

    pub(crate) fn finish(self) -> Result<(), GateError> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(GateError::LogFormat)
        }
    }
}

fn decode_op(code: u8, payload: &[u8]) -> Result<GateOp, GateError> {
    let mut c = Cursor(payload);
    let op = match code {
        OP_ERASE_SUBJECT => GateOp::EraseSubject(SubjOrd(c.u32()?)),
        OP_ERASE_VER => {
            let ver = VerOrd(c.u32()?);
            let reason = EraseReason::from_code(c.u8()?).ok_or(GateError::LogFormat)?;
            GateOp::EraseVer { ver, reason }
        }
        OP_RESTRICT => {
            let ver = VerOrd(c.u32()?);
            let reason = RestrictReason::from_code(c.u8()?).ok_or(GateError::LogFormat)?;
            GateOp::Restrict { ver, reason }
        }
        OP_RELEASE => GateOp::Release(VerOrd(c.u32()?)),
        OP_SUPERSEDE => GateOp::Supersede(VerOrd(c.u32()?)),
        OP_REDACT_K => GateOp::RedactK(KOrd(c.u64()?)),
        OP_HIGH_WATER => {
            let kind = HighWaterKind::from_code(c.u8()?).ok_or(GateError::LogFormat)?;
            GateOp::HighWater {
                kind,
                value: c.u64()?,
            }
        }
        _ => return Err(GateError::LogFormat),
    };
    c.finish()?;
    Ok(op)
}
