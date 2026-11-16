//! Verdichtung des Gate-Logs (SPEC 5.3, C23): Snapshot, neues Log, atomarer Austausch.
//!
//! Jede Negativprüfung hat eine Positivkontrolle (SPEC 16.4).
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::indexing_slicing)]

mod support;

use acta_base::ids::{KOrd, SubjOrd, VerOrd};
use acta_gate::{
    EraseReason, ErasedView, GateError, GateOp, GateState, GateWriter, HighWaterKind,
    RestrictReason,
};
use support::{disk, durable, fresh, key, reopen, snapshot, FaultSink};

fn populate(w: &mut GateWriter) {
    w.commit(&[
        GateOp::EraseVer {
            ver: VerOrd(1),
            reason: EraseReason::Request,
        },
        GateOp::Restrict {
            ver: VerOrd(2),
            reason: RestrictReason::LegalHold,
        },
        GateOp::Supersede(VerOrd(3)),
        GateOp::RedactK(KOrd(9)),
        GateOp::EraseSubject(SubjOrd(4)),
        GateOp::HighWater {
            kind: HighWaterKind::Ver,
            value: 40,
        },
    ])
    .expect("commit");
}

fn assert_state(v: &dyn ErasedView) {
    assert_eq!(v.check_ver(VerOrd(1)), GateState::Erased);
    assert_eq!(
        v.check_ver(VerOrd(2)),
        GateState::Restricted(RestrictReason::LegalHold)
    );
    assert_eq!(v.check_ver(VerOrd(3)), GateState::Superseded);
    assert_eq!(v.check_ver(VerOrd(5)), GateState::Live);
    assert!(v.is_redacted_k(KOrd(9)));
    assert!(v.is_subject_erased(SubjOrd(4)));
}

/// Nach Verdichtung und Neustart ist der Stand gleich; Sperren werden nie verkleinert.
#[test]
fn compact_then_recover_keeps_every_block() {
    let d = disk();
    let (mut w, view) = fresh(&d);
    populate(&mut w);
    assert_state(&view); // Positivkontrolle vor der Verdichtung
    let before = durable(&d).len();
    let epoch = w.compact().expect("compact");
    assert_eq!(epoch, 6);
    assert!(snapshot(&d).is_some());
    assert!(
        durable(&d).len() < before,
        "neues Log ist kürzer als das alte"
    );
    assert_state(&view);

    let (_w2, view2) = reopen(&d, FaultSink::ok(&d)).expect("reopen");
    assert_state(&view2);
    assert_eq!(
        view2.epoch(),
        7,
        "Kopfrahmen 0x08 trägt die nächste Sequenz"
    );
}

/// Nach der Verdichtung läuft die Kette weiter; weitere Sperren überleben einen Neustart.
#[test]
fn commits_after_compaction_are_durable_and_chained() {
    let d = disk();
    let (mut w, _view) = fresh(&d);
    populate(&mut w);
    w.compact().expect("compact");
    w.commit(&[GateOp::EraseVer {
        ver: VerOrd(5),
        reason: EraseReason::Request,
    }])
    .expect("commit");
    w.compact().expect("zweite Verdichtung");
    w.commit(&[GateOp::Supersede(VerOrd(6))]).expect("commit");

    let (_w2, v2) = reopen(&d, FaultSink::ok(&d)).expect("reopen");
    assert_eq!(v2.check_ver(VerOrd(1)), GateState::Erased);
    assert!(v2.is_redacted_k(KOrd(9)));
    assert_eq!(v2.check_ver(VerOrd(5)), GateState::Erased);
    assert_eq!(v2.check_ver(VerOrd(6)), GateState::Superseded);
}

/// Ein verändertes Byte im Snapshot ist ein Integritätsfehler (Positivkontrolle: unverändert).
#[test]
fn tampered_snapshot_is_rejected() {
    let d = disk();
    let (mut w, _v) = fresh(&d);
    populate(&mut w);
    w.compact().expect("compact");
    assert!(reopen(&d, FaultSink::ok(&d)).is_ok(), "Positivkontrolle");

    let mut snap = snapshot(&d).expect("snapshot");
    let mid = snap.len() / 2;
    snap[mid] ^= 1;
    let log = durable(&d);
    let r =
        GateWriter::recover_with_snapshot(key(), Some(&snap), &log, Box::new(FaultSink::ok(&d)));
    assert!(matches!(r, Err(GateError::ChainBroken { .. })));
}

/// Falscher Schlüssel und abgeschnittener Snapshot werden abgelehnt.
#[test]
fn wrong_key_and_truncated_snapshot_are_rejected() {
    let d = disk();
    let (mut w, _v) = fresh(&d);
    populate(&mut w);
    w.compact().expect("compact");
    let snap = snapshot(&d).expect("snapshot");
    let log = durable(&d);

    let other = acta_gate::GateKey::from_bytes([9u8; 32]);
    let r =
        GateWriter::recover_with_snapshot(other, Some(&snap), &log, Box::new(FaultSink::ok(&d)));
    assert!(r.is_err());

    let cut = &snap[..snap.len() - 1];
    let r = GateWriter::recover_with_snapshot(key(), Some(cut), &log, Box::new(FaultSink::ok(&d)));
    assert!(r.is_err());
}

/// Ein Log aus einem anderen Stand (alter Kopf ohne passenden Snapshot) bricht die Kette.
#[test]
fn log_head_bound_to_its_snapshot() {
    let d = disk();
    let (mut w, _v) = fresh(&d);
    populate(&mut w);
    w.compact().expect("compact");
    let first_snap = snapshot(&d).expect("snapshot");
    let first_log = durable(&d);
    w.commit(&[GateOp::RedactK(KOrd(10))]).expect("commit");
    w.compact().expect("compact 2");
    let second_snap = snapshot(&d).expect("snapshot 2");

    // Positivkontrolle: passende Paare öffnen.
    let ok = GateWriter::recover_with_snapshot(
        key(),
        Some(&first_snap),
        &first_log,
        Box::new(FaultSink::ok(&d)),
    );
    assert!(ok.is_ok());
    // Log 1 mit Snapshot 2 ist unzulässig.
    let bad = GateWriter::recover_with_snapshot(
        key(),
        Some(&second_snap),
        &first_log,
        Box::new(FaultSink::ok(&d)),
    );
    assert!(bad.is_err());
}

/// C23: scheitert der Austausch, gilt der alte Stand; der Mandant ist nicht vergiftet,
/// die Verdichtung ist wiederholbar.
#[test]
fn failed_install_keeps_old_state_and_is_retryable() {
    let d = disk();
    let (mut w, view) = fresh(&d);
    populate(&mut w);
    drop(w);
    let (mut w, view2) = reopen(&d, FaultSink::failing_install(&d)).expect("reopen");
    let log_before = durable(&d);
    assert!(matches!(w.compact(), Err(GateError::LogWrite)));
    assert!(!w.is_poisoned());
    assert!(snapshot(&d).is_none());
    assert_eq!(durable(&d), log_before, "altes Log unverändert");
    assert_state(&view2);
    drop(view);

    // Wiederholung mit gesundem Sink: Positivkontrolle.
    drop(w);
    let (mut w, _v) = reopen(&d, FaultSink::ok(&d)).expect("reopen");
    w.compact().expect("retry");
    assert!(snapshot(&d).is_some());
}

/// Ein Sink ohne Austausch lehnt die Verdichtung ab, ohne den Stand zu ändern.
#[test]
fn compact_on_unsupported_sink_fails_cleanly() {
    struct Plain;
    impl acta_gate::GateSink for Plain {
        fn append(&mut self, _: &[u8]) -> std::io::Result<()> {
            Ok(())
        }
        fn sync(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let (mut w, _v) = GateWriter::recover(key(), &[], Box::new(Plain)).expect("recover");
    w.commit(&[GateOp::RedactK(KOrd(1))]).expect("commit");
    assert!(matches!(w.compact(), Err(GateError::LogWrite)));
    assert!(!w.is_poisoned());
    w.commit(&[GateOp::RedactK(KOrd(2))])
        .expect("weiter schreibbar");
}

/// Verdichtung eines leeren Gates ist zulässig und ergibt einen leeren, prüfbaren Stand.
#[test]
fn compact_of_empty_gate_roundtrips() {
    let d = disk();
    let (mut w, _v) = fresh(&d);
    assert_eq!(w.compact().expect("compact"), 0);
    let (_w2, v2) = reopen(&d, FaultSink::ok(&d)).expect("reopen");
    assert_eq!(v2.check_ver(VerOrd(1)), GateState::Live);
    assert_eq!(v2.epoch(), 1);
}

/// Ohne Snapshot ist ein Kopfrahmen 0x08 nicht zulässig.
#[test]
fn snapshot_frame_without_snapshot_is_rejected() {
    let d = disk();
    let (mut w, _v) = fresh(&d);
    w.compact().expect("compact");
    let head = durable(&d);
    let r = GateWriter::recover(key(), &head, Box::new(FaultSink::ok(&d)));
    assert!(r.is_err());
}
