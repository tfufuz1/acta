//! Reihenfolge, Haltbarkeit und Integrität des Gate-Logs (SPEC 5.3 Z4, 3.7, Z1, Z10).
//!
//! Die Tests arbeiten mit dem Speicherlog aus `support`. Jede Negativprüfung hat eine
//! Positivkontrolle (SPEC 16.4): ein Lauf ohne Fehler, der dieselbe Prüfung besteht.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::indexing_slicing)]

mod support;

use acta_base::ids::VerOrd;
use acta_gate::GateWriter;
use acta_gate::{EraseReason, ErasedView, GateError, GateOp, GateState, RestrictReason};
use support::{disk, durable, fresh, open, FaultSink};

fn erase(n: u32) -> GateOp {
    GateOp::EraseVer {
        ver: VerOrd(n),
        reason: EraseReason::Request,
    }
}

/// Z1: nach Rückkehr des Aufrufs sperrt das Gate sofort, ohne Warten auf die Platte.
#[test]
fn z1_erase_blocks_reads_as_soon_as_commit_returns() {
    let d = disk();
    let (mut w, view) = fresh(&d);
    let reader = view.clone();
    w.commit(&[erase(1)]).expect("commit");
    assert_eq!(reader.check_ver(VerOrd(1)), GateState::Erased);
    assert_eq!(view.check_ver(VerOrd(1)), GateState::Erased);
    assert!(
        !durable(&d).is_empty(),
        "Positivkontrolle: Log ist haltbar geschrieben"
    );
}

/// Z4: schlägt `fsync` fehl, bleibt die Sperre im Arbeitsspeicher. Der Aufruf meldet keinen
/// Erfolg, der Mandant ist vergiftet, und der durable Stand enthält die Sperre nicht.
#[test]
fn z4_sync_failure_keeps_ram_block_and_poisons_tenant() {
    let d = disk();
    let (mut w, view) = open(&[], FaultSink::failing_sync(&d));
    let result = w.commit(&[erase(1)]);
    assert!(
        matches!(result, Err(GateError::LogSync)),
        "fsync-Fehler meldet Integrity"
    );
    assert_eq!(
        view.check_ver(VerOrd(1)),
        GateState::Erased,
        "RAM-Sperre bleibt bestehen"
    );
    assert!(w.is_poisoned());
    assert!(matches!(w.commit(&[erase(2)]), Err(GateError::Poisoned)));

    // Absturz: nur der durable Stand überlebt; der Antrag muss erneut gestellt werden.
    let (_, after_crash) = open(&durable(&d), FaultSink::ok(&d));
    assert_eq!(
        after_crash.check_ver(VerOrd(1)),
        GateState::Live,
        "nicht bestätigte Sperre ist nach dem Neustart nicht durabel"
    );
}

/// Positivkontrolle zu Z4: derselbe Ablauf mit funktionierendem `fsync` ist nach dem Neustart
/// durabel. Ohne sie wäre der Negativtest oben bedeutungslos.
#[test]
fn z4_positive_control_successful_sync_survives_restart() {
    let d = disk();
    let (mut w, _view) = fresh(&d);
    w.commit(&[erase(1)]).expect("commit");
    assert!(!w.is_poisoned());
    let (_, after_restart) = open(&durable(&d), FaultSink::ok(&d));
    assert_eq!(after_restart.check_ver(VerOrd(1)), GateState::Erased);
}

#[test]
fn z4_append_failure_poisons_and_reports_log_write() {
    let d = disk();
    let (mut w, view) = open(&[], FaultSink::failing_append(&d));
    assert!(matches!(w.commit(&[erase(1)]), Err(GateError::LogWrite)));
    assert!(w.is_poisoned());
    assert_eq!(
        view.check_ver(VerOrd(1)),
        GateState::Erased,
        "RAM-Sperre bleibt"
    );
}

/// Der Wiederaufbau aus dem Log reproduziert den Live-Zustand genau.
#[test]
fn recovery_reproduces_live_state() {
    let d = disk();
    let (mut w, view) = fresh(&d);
    w.commit(&[
        erase(1),
        GateOp::Restrict {
            ver: VerOrd(2),
            reason: RestrictReason::LegalHold,
        },
        GateOp::Supersede(VerOrd(3)),
    ])
    .expect("first");
    w.commit(&[GateOp::Release(VerOrd(2))]).expect("second");

    let (_, restored) = open(&durable(&d), FaultSink::ok(&d));
    assert_eq!(restored.epoch(), view.epoch());
    for n in 0..5 {
        assert_eq!(
            restored.check_ver(VerOrd(n)),
            view.check_ver(VerOrd(n)),
            "Version {n}"
        );
    }
    // Positivkontrolle: die Stichprobe enthält alle relevanten Zustände.
    assert_eq!(view.check_ver(VerOrd(1)), GateState::Erased);
    assert_eq!(view.check_ver(VerOrd(2)), GateState::Live);
    assert_eq!(view.check_ver(VerOrd(3)), GateState::Superseded);
}

/// Ein verändertes Byte in der Kette ist ein Integritätsfehler, kein Ersatzwert (Z10).
#[test]
fn chain_tamper_is_detected_and_untouched_log_is_accepted() {
    let d = disk();
    let (mut w, _view) = fresh(&d);
    w.commit(&[erase(1)]).expect("commit");
    let log = durable(&d);

    // Positivkontrolle: unverändert wird er akzeptiert.
    assert!(recover_default(&log).is_ok());

    // Erstes Nutzlastbyte (Kopf 11 Bytes) verändern.
    let mut tampered = log.clone();
    tampered[11] ^= 0x01;
    assert!(matches!(
        recover_default(&tampered),
        Err(GateError::ChainBroken { seq: 1 })
    ));

    // Verändertes Kettenetikett.
    let mut tag_tampered = log.clone();
    let last = tag_tampered.len() - 1;
    tag_tampered[last] ^= 0x80;
    assert!(matches!(
        recover_default(&tag_tampered),
        Err(GateError::ChainBroken { seq: 1 })
    ));
}

/// Ein falscher Schlüssel verifiziert nicht; der Stand wird nicht übernommen (Z10).
#[test]
fn wrong_key_fails_closed() {
    let d = disk();
    let (mut w, _view) = fresh(&d);
    w.commit(&[erase(1)]).expect("commit");
    let log = durable(&d);
    let other = acta_gate::GateKey::from_bytes([9u8; 32]);
    let result = GateWriter::recover(other, &log, Box::new(FaultSink::ok(&d)));
    assert!(matches!(result, Err(GateError::ChainBroken { seq: 1 })));
}

/// Ein abgeschnittener letzter Rahmen ist nie bestätigt und wird verworfen; der Rest bleibt.
#[test]
fn truncated_tail_is_discarded_and_earlier_frames_kept() {
    let d = disk();
    let (mut w, _view) = fresh(&d);
    w.commit(&[erase(1)]).expect("first");
    w.commit(&[erase(2)]).expect("second");
    let log = durable(&d);

    // Positivkontrolle: beide Rahmen sind vorhanden.
    let (_, full) = open(&log, FaultSink::ok(&d));
    assert_eq!(full.check_ver(VerOrd(2)), GateState::Erased);

    let cut = &log[..log.len() - 5];
    let (_, partial) = open(cut, FaultSink::ok(&d));
    assert_eq!(partial.check_ver(VerOrd(1)), GateState::Erased);
    assert_eq!(partial.check_ver(VerOrd(2)), GateState::Live);
}

/// Eine fehlende Sequenznummer (ein ganzer Rahmen entfernt) ist ein Kettenbruch.
#[test]
fn sequence_gap_is_detected() {
    let d = disk();
    let (mut w, _view) = fresh(&d);
    // EraseSubject: Kopf 11 + Nutzlast 4 + Etikett 32 = 47 Bytes je Rahmen.
    for s in 1..=3u32 {
        w.commit(&[GateOp::EraseSubject(acta_base::ids::SubjOrd(s))])
            .expect("commit");
    }
    let log = durable(&d);
    assert_eq!(log.len(), 3 * 47, "Positivkontrolle: feste Rahmenlänge");
    let mut gap = log[..47].to_vec();
    gap.extend_from_slice(&log[94..]);
    assert!(matches!(
        recover_default(&gap),
        Err(GateError::ChainBroken { seq: 3 })
    ));
}

/// Ein leeres Log ist der Anfangszustand.
#[test]
fn empty_log_is_initial_state() {
    let d = disk();
    let (_w, view) = open(&[], FaultSink::ok(&d));
    assert_eq!(view.epoch(), 0);
    assert_eq!(view.check_ver(VerOrd(1)), GateState::Live);
}

/// Hilfsfunktion: Wiederaufbau mit dem Standardschlüssel, ohne Sink-Nebenwirkungen.
fn recover_default(log: &[u8]) -> Result<(), GateError> {
    let d = disk();
    GateWriter::recover(support::key(), log, Box::new(FaultSink::ok(&d))).map(|_| ())
}
