//! Snapshot-Datei der Verdichtung (SPEC 5.3, C23).
//!
//! Aufbau: `"AGSN" ‖ ver u8 ‖ epoch u64 ‖ high_water 3×u64 ‖ prev_tag 32 ‖
//! n u32 ‖ n×(ver u32, grund u8) ‖ 4 Mengen je [len u32 ‖ Roaring] ‖ tag 32`.
//! Der Tag ist `HMAC(k, "acta/gate/snap/v1" ‖ Körper)`; er ist zugleich Startwert der
//! Kette des neuen Logs. `ErasedVers`, `ErasedSubjects` und `RedactedK` werden nie
//! verkleinert: der Snapshot gibt sie unverändert wieder.

use hmac::Mac;
use roaring::{RoaringBitmap, RoaringTreemap};

use crate::error::GateError;
use crate::log::{Cursor, GateKey, TAG_LEN};
use crate::reasons::RestrictReason;
use crate::state::GateSnapshot;

const MAGIC: &[u8; 4] = b"AGSN";
const FORMAT: u8 = 1;
const DOMAIN: &[u8] = b"acta/gate/snap/v1";

fn put_set(
    out: &mut Vec<u8>,
    len: usize,
    write: impl FnOnce(&mut Vec<u8>) -> std::io::Result<()>,
) -> Result<(), GateError> {
    let n = u32::try_from(len).map_err(|_| GateError::LogFormat)?;
    out.extend_from_slice(&n.to_be_bytes());
    write(out).map_err(|_| GateError::LogFormat)
}

/// Kodiert den Stand. `prev` ist das letzte Kettenetikett des alten Logs.
pub(crate) fn encode(
    key: &GateKey,
    snap: &GateSnapshot,
    prev: &[u8; TAG_LEN],
) -> Result<(Vec<u8>, [u8; TAG_LEN]), GateError> {
    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    out.push(FORMAT);
    out.extend_from_slice(&snap.epoch.to_be_bytes());
    for w in snap.high_water {
        out.extend_from_slice(&w.to_be_bytes());
    }
    out.extend_from_slice(prev);
    let n = u32::try_from(snap.restrict_reasons.len()).map_err(|_| GateError::LogFormat)?;
    out.extend_from_slice(&n.to_be_bytes());
    for (ver, reason) in &snap.restrict_reasons {
        out.extend_from_slice(&ver.to_be_bytes());
        out.push(reason.code());
    }
    put_set(&mut out, snap.erased_subjects.serialized_size(), |o| {
        snap.erased_subjects.serialize_into(o)
    })?;
    put_set(&mut out, snap.erased_vers.serialized_size(), |o| {
        snap.erased_vers.serialize_into(o)
    })?;
    put_set(&mut out, snap.superseded_vers.serialized_size(), |o| {
        snap.superseded_vers.serialize_into(o)
    })?;
    put_set(&mut out, snap.redacted_k.serialized_size(), |o| {
        snap.redacted_k.serialize_into(o)
    })?;

    let mut mac = key.mac()?;
    mac.update(DOMAIN);
    mac.update(&out);
    let tag: [u8; TAG_LEN] = mac.finalize().into_bytes().into();
    out.extend_from_slice(&tag);
    Ok((out, tag))
}

/// Prüft das MAC und dekodiert. Jede Abweichung ist ein Integritätsfehler.
pub(crate) fn decode(
    key: &GateKey,
    bytes: &[u8],
) -> Result<(GateSnapshot, [u8; TAG_LEN]), GateError> {
    let split = bytes
        .len()
        .checked_sub(TAG_LEN)
        .ok_or(GateError::LogFormat)?;
    let (body, tag_bytes) = bytes.split_at_checked(split).ok_or(GateError::LogFormat)?;
    let mut mac = key.mac()?;
    mac.update(DOMAIN);
    mac.update(body);
    mac.verify_slice(tag_bytes)
        .map_err(|_| GateError::ChainBroken { seq: 0 })?;
    let tag = <[u8; TAG_LEN]>::try_from(tag_bytes).map_err(|_| GateError::LogFormat)?;

    let mut c = Cursor(body);
    if c.take::<4>()? != *MAGIC || c.u8()? != FORMAT {
        return Err(GateError::LogFormat);
    }
    let epoch = c.u64()?;
    let mut snap = GateSnapshot {
        epoch,
        ..GateSnapshot::default()
    };
    for slot in &mut snap.high_water {
        *slot = c.u64()?;
    }
    let _prev = c.take::<TAG_LEN>()?;
    for _ in 0..c.u32()? {
        let ver = c.u32()?;
        let reason = RestrictReason::from_code(c.u8()?).ok_or(GateError::LogFormat)?;
        snap.restricted_vers.insert(ver);
        snap.restrict_reasons.insert(ver, reason);
    }
    snap.erased_subjects = read_bitmap(&mut c)?;
    snap.erased_vers = read_bitmap(&mut c)?;
    snap.superseded_vers = read_bitmap(&mut c)?;
    let n = usize::try_from(c.u32()?).map_err(|_| GateError::LogFormat)?;
    snap.redacted_k =
        RoaringTreemap::deserialize_from(c.bytes(n)?).map_err(|_| GateError::LogFormat)?;
    c.finish()?;
    Ok((snap, tag))
}

fn read_bitmap(c: &mut Cursor<'_>) -> Result<RoaringBitmap, GateError> {
    let n = usize::try_from(c.u32()?).map_err(|_| GateError::LogFormat)?;
    RoaringBitmap::deserialize_from(c.bytes(n)?).map_err(|_| GateError::LogFormat)
}
