//! Eigenschaftstests für acta-base (SPEC 16.2, Ebene „Eigenschaft“).
//! Orakel: die Spezifikation der Kodierung (SPEC 0.3), mathematische Eigenschaften der
//! Festkommaarithmetik und die Zeitordnung von UUIDv7 (SPEC 2.4). Die Implementierung ist kein Orakel.

use acta_base::canon::{Reader, Writer};
use acta_base::fx::{fx_ln, isqrt, Fx};
use acta_base::ids::{ErasureId, KOrd, VerOrd};
use acta_base::limits;
use proptest::prelude::*;

proptest! {
    // SPEC: 0.3 (Big-Endian, geprüfte Längen). Hin- und Rückumwandlung aller Feldtypen.
    #[test]
    fn canon_roundtrip_all_field_types(
        a in any::<u8>(),
        b in any::<u16>(),
        c in any::<u32>(),
        d in any::<u64>(),
        e in any::<i32>(),
        f in any::<i64>(),
        blob in proptest::collection::vec(any::<u8>(), 0..256),
    ) {
        let mut w = Writer::with_limit(1 << 12);
        w.put_u8(a).expect("budget");
        w.put_u16(b).expect("budget");
        w.put_u32(c).expect("budget");
        w.put_u64(d).expect("budget");
        w.put_i32(e).expect("budget");
        w.put_i64(f).expect("budget");
        w.put_blob(&blob).expect("budget");
        let bytes = w.into_bytes();
        let mut r = Reader::new(&bytes);
        prop_assert_eq!(r.u8().expect("u8"), a);
        prop_assert_eq!(r.u16().expect("u16"), b);
        prop_assert_eq!(r.u32().expect("u32"), c);
        prop_assert_eq!(r.u64().expect("u64"), d);
        prop_assert_eq!(r.i32().expect("i32"), e);
        prop_assert_eq!(r.i64().expect("i64"), f);
        prop_assert_eq!(r.blob(256).expect("blob"), &blob[..]);
        prop_assert!(r.finish().is_ok());
    }

    // SPEC: 3.6 (Speicherbudget), 0.3. Ein Blob wird nie länger als seine Obergrenze und nie
    // länger als der Rest der Eingabe zurückgegeben; sonst Fehler (fail-closed).
    #[test]
    fn blob_never_exceeds_bound_or_input(
        bytes in proptest::collection::vec(any::<u8>(), 0..64),
        max in 0_usize..64,
    ) {
        let mut r = Reader::new(&bytes);
        if let Ok(out) = r.blob(max) {
            prop_assert!(out.len() <= max);
            prop_assert!(out.len() <= bytes.len());
        }
    }

    // SPEC: 3.3 (kanonische Kodierung), Z10 (fail-closed). Beliebige Eingaben führen nie zu Panik.
    #[test]
    fn reader_never_panics_on_arbitrary_input(
        bytes in proptest::collection::vec(any::<u8>(), 0..64),
    ) {
        let mut r = Reader::new(&bytes);
        match r.u32() {
            Ok(_) | Err(_) => {}
        }
        match r.blob(16) {
            Ok(_) | Err(_) => {}
        }
        match r.u64() {
            Ok(_) | Err(_) => {}
        }
        match r.finish() {
            Ok(()) | Err(_) => {}
        }
    }

    // SPEC: 3.6 (Speicherbudget). Der Schreiber überschreitet seine Obergrenze nie, und ein
    // abgelehnter Schreibvorgang lässt den Puffer unverändert.
    #[test]
    fn writer_respects_limit_and_is_atomic(
        max in 0_usize..32,
        ops in proptest::collection::vec(0_u8..3, 0..16),
    ) {
        let mut w = Writer::with_limit(max);
        for op in ops {
            let before = w.len();
            let res = match op {
                0 => w.put_u8(1),
                1 => w.put_u32(2),
                _ => w.put_blob(&[3_u8; 5]),
            };
            prop_assert!(w.len() <= max);
            if res.is_err() {
                prop_assert_eq!(w.len(), before);
            }
        }
    }

    // SPEC: 2.4 (ErasureId, UUIDv7). Zeitanteil bleibt exakt erhalten; Ordnung folgt der Zeit
    // unabhängig von den Zufallsbits.
    #[test]
    fn erasure_id_v7_roundtrip_and_time_order(
        ms1 in 0_u64..(1_u64 << 48),
        ms2 in 0_u64..(1_u64 << 48),
        r1 in any::<[u8; 10]>(),
        r2 in any::<[u8; 10]>(),
    ) {
        let a = ErasureId::from_v7(ms1, r1).expect("48-bit");
        let b = ErasureId::from_v7(ms2, r2).expect("48-bit");
        prop_assert_eq!(a.v7_ms(), ms1);
        prop_assert_eq!(b.v7_ms(), ms2);
        if ms1 < ms2 {
            prop_assert!(a < b);
        }
    }

    // SPEC: 2.4 (dichte Ordinale, nie wiederverwendet). `next` ist streng monoton bis zur Grenze.
    #[test]
    fn ordinals_are_strictly_increasing(x in 0_u64..u64::MAX) {
        let k = KOrd(x);
        let n = k.next().expect("below max");
        prop_assert!(n > k);
        prop_assert_eq!(n.0, x + 1);
        let v = VerOrd(u32::try_from(x % u64::from(u32::MAX)).expect("fits"));
        if v.0 < u32::MAX {
            prop_assert!(v.next().expect("below max") > v);
        }
    }

    // SPEC: 9.2 (Festkomma, Z7). Der Logarithmus ist monoton und deterministisch.
    #[test]
    fn fx_ln_is_monotone_and_deterministic(a in 1_i64..i64::MAX, b in 1_i64..i64::MAX) {
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        let flo = fx_ln(Fx::from_bits(lo)).expect("positive").to_bits();
        let fhi = fx_ln(Fx::from_bits(hi)).expect("positive").to_bits();
        prop_assert!(flo <= fhi);
        prop_assert_eq!(fx_ln(Fx::from_bits(lo)).map(Fx::to_bits), Some(flo));
    }

    // SPEC: 9.2 (Festkomma). Addition und Subtraktion sättigen exakt wie die i128-Referenz.
    #[test]
    fn fx_add_sub_saturate_like_wide_reference(a in any::<i64>(), b in any::<i64>()) {
        let sum = i128::from(a) + i128::from(b);
        let diff = i128::from(a) - i128::from(b);
        let clamp = |v: i128| i64::try_from(v).unwrap_or(if v < 0 { i64::MIN } else { i64::MAX });
        prop_assert_eq!(Fx::from_bits(a).add(Fx::from_bits(b)).to_bits(), clamp(sum));
        prop_assert_eq!(Fx::from_bits(a).sub(Fx::from_bits(b)).to_bits(), clamp(diff));
    }

    // SPEC: 9.2 (Festkomma, Z7). Multiplikation ist kommutativ; Eins ist neutrales Element;
    // Division durch Eins ist exakt.
    #[test]
    fn fx_mul_div_algebra(a in any::<i64>(), b in any::<i64>()) {
        let (x, y) = (Fx::from_bits(a), Fx::from_bits(b));
        prop_assert_eq!(x.mul(y), y.mul(x));
        prop_assert_eq!(x.mul(Fx::ONE), x);
        prop_assert_eq!(x.div(Fx::ONE), Some(x));
        prop_assert_eq!(x.div(Fx::ZERO), None);
    }

    // SPEC: 9.2 (Ganzzahlwurzel). isqrt liefert den abgerundeten Wert.
    #[test]
    fn isqrt_is_floor_of_root(n in any::<u64>()) {
        let r = u128::from(isqrt(n));
        prop_assert!(r * r <= u128::from(n));
        prop_assert!((r + 1) * (r + 1) > u128::from(n));
    }
}

// SPEC: Anhang A (Startwerte liegen in ihrem Bereich). Stichprobe über alle Grenzen.
#[test]
fn limits_bounds_check_matches_reference() {
    let b = limits::Bounds { min: 3, max: 9 };
    for v in 0..=12_u64 {
        assert_eq!(b.contains(v), (3..=9).contains(&v), "v={v}");
    }
}

// SPEC: Anhang A.2 bis A.4 (Parameter ohne Eintrag bisher). Rot, bis die Konstanten existieren.
#[test]
fn additional_parameters_have_start_values_inside_bounds() {
    let pairs = [
        (limits::COMMIT_EPOCH_MS, limits::COMMIT_EPOCH_BOUNDS),
        (
            limits::OPS_LOG_RETENTION_MS,
            limits::OPS_LOG_RETENTION_BOUNDS,
        ),
        (limits::WAL_MAX_AGE_MS, limits::WAL_MAX_AGE_BOUNDS),
        (
            limits::CLUSTER_SEG_MIN_BYTES,
            limits::CLUSTER_SEG_MIN_BOUNDS,
        ),
        (limits::DP_CLUSTER_MIN_BYTES, limits::DP_CLUSTER_MIN_BOUNDS),
        (limits::INGEST_IDEM_TTL_MS, limits::INGEST_IDEM_TTL_BOUNDS),
        (limits::ERASE_SLA_B_MS, limits::ERASE_SLA_B_BOUNDS),
        (
            limits::MULTI_REVIEW_MAX_WORKDAYS,
            limits::MULTI_REVIEW_MAX_BOUNDS,
        ),
        (limits::MEASURE_RETRIES, limits::MEASURE_RETRIES_BOUNDS),
        (
            limits::HOLD_REVIEW_INTERVAL_MS,
            limits::HOLD_REVIEW_INTERVAL_BOUNDS,
        ),
        (limits::ARCHIVE_WAIT_MAX_MS, limits::ARCHIVE_WAIT_MAX_BOUNDS),
        (limits::OUTBOX_TTL_MS, limits::OUTBOX_TTL_BOUNDS),
    ];
    for (v, b) in pairs {
        assert!(b.contains(v), "{v} nicht in {b:?}");
    }
    // Aufbewahrung der Schlüssel: Startwert 90 Tage (A.2). Die Obergrenze prüft eine const-Assertion in limits.rs.
    assert_eq!(limits::COMMIT_KEY_RETENTION_START_MS, 90 * limits::DAY_MS);
}
