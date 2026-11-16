//! Zustandsregeln des Gates (SPEC 5.2, 5.3, 8.1, Z13).
//!
//! Jede Negativprüfung wird von einer Positivkontrolle begleitet, die zeigt, dass der Zustand
//! vor der Prüfung tatsächlich wirksam war (SPEC 16.4).
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::indexing_slicing)]

mod support;

use acta_base::ids::{KOrd, SubjOrd, VerOrd};
use acta_gate::{
    EraseReason, ErasedView, GateError, GateOp, GateState, HighWaterKind, Lineage, LineageHit,
    RestrictReason,
};
use roaring::{RoaringBitmap, RoaringTreemap};
use support::{disk, fresh};

fn v(n: u32) -> VerOrd {
    VerOrd(n)
}

/// Positivkontrolle: der Zustand ist nach dem Commit tatsächlich gesetzt.
#[test]
fn t_gate_state_positive_control_commit_is_visible() {
    let d = disk();
    let (mut w, view) = fresh(&d);
    assert_eq!(view.check_ver(v(1)), GateState::Live);
    w.commit(&[GateOp::EraseVer {
        ver: v(1),
        reason: EraseReason::Request,
    }])
    .expect("commit");
    assert_eq!(view.check_ver(v(1)), GateState::Erased);
}

/// SPEC 5.3: `ErasedVers` hat Vorrang vor `RestrictedVers` und `SupersededVers`.
#[test]
fn t_gate_state_priority_erased_over_restricted_over_superseded() {
    let d = disk();
    let (mut w, view) = fresh(&d);
    w.commit(&[
        GateOp::Supersede(v(1)),
        GateOp::Restrict {
            ver: v(1),
            reason: RestrictReason::Retention,
        },
        GateOp::Supersede(v(2)),
        GateOp::Restrict {
            ver: v(2),
            reason: RestrictReason::LegalHold,
        },
        GateOp::Supersede(v(3)),
    ])
    .expect("commit");

    // Positivkontrolle: jede Stufe ist einzeln erreichbar.
    assert_eq!(
        view.check_ver(v(1)),
        GateState::Restricted(RestrictReason::Retention),
        "Restricted überdeckt Superseded"
    );
    assert_eq!(
        view.check_ver(v(2)),
        GateState::Restricted(RestrictReason::LegalHold)
    );
    assert_eq!(view.check_ver(v(3)), GateState::Superseded);
    assert_eq!(view.check_ver(v(4)), GateState::Live);

    w.commit(&[GateOp::EraseVer {
        ver: v(1),
        reason: EraseReason::Objection,
    }])
    .expect("erase restricted version");
    assert_eq!(
        view.check_ver(v(1)),
        GateState::Erased,
        "Erased überdeckt Restricted und Superseded"
    );
}

/// `Erased` ist terminal: keine Einschränkung, keine Freigabe danach (SPEC 5.3, 8.1).
#[test]
fn t_gate_state_erased_is_terminal() {
    let d = disk();
    let (mut w, view) = fresh(&d);
    w.commit(&[GateOp::EraseVer {
        ver: v(5),
        reason: EraseReason::CourtOrder,
    }])
    .expect("commit");
    let epoch_before = view.epoch();

    assert!(matches!(
        w.commit(&[GateOp::Restrict {
            ver: v(5),
            reason: RestrictReason::Precautionary
        }]),
        Err(GateError::EraseTerminal)
    ));
    assert!(matches!(
        w.commit(&[GateOp::Release(v(5))]),
        Err(GateError::EraseTerminal)
    ));
    assert_eq!(view.check_ver(v(5)), GateState::Erased);
    assert_eq!(
        view.epoch(),
        epoch_before,
        "abgelehnte Stapel ändern die Epoche nicht"
    );
    assert!(
        !w.is_poisoned(),
        "Regelverstöße vergiften den Mandanten nicht"
    );
}

/// `Release` nur aus `Restricted` (SPEC 5.3).
#[test]
fn t_gate_release_only_from_restricted() {
    let d = disk();
    let (mut w, view) = fresh(&d);
    assert!(matches!(
        w.commit(&[GateOp::Release(v(9))]),
        Err(GateError::NotRestricted)
    ));
    assert_eq!(view.check_ver(v(9)), GateState::Live);

    w.commit(&[GateOp::Restrict {
        ver: v(9),
        reason: RestrictReason::Objection,
    }])
    .expect("restrict");
    assert_eq!(
        view.check_ver(v(9)),
        GateState::Restricted(RestrictReason::Objection),
        "Positivkontrolle vor der Freigabe"
    );
    w.commit(&[GateOp::Release(v(9))]).expect("release");
    assert_eq!(view.check_ver(v(9)), GateState::Live);
}

/// Ein Stapel mit einer unzulässigen Operation verändert nichts (atomar, SPEC 5.3).
#[test]
fn t_gate_failed_batch_changes_nothing() {
    let d = disk();
    let (mut w, view) = fresh(&d);
    let result = w.commit(&[
        GateOp::EraseVer {
            ver: v(1),
            reason: EraseReason::Request,
        },
        GateOp::Release(v(2)),
    ]);
    assert!(matches!(result, Err(GateError::NotRestricted)));
    assert_eq!(
        view.check_ver(v(1)),
        GateState::Live,
        "erste Operation nicht übernommen"
    );
    assert_eq!(view.epoch(), 0);

    w.commit(&[GateOp::EraseVer {
        ver: v(1),
        reason: EraseReason::Request,
    }])
    .expect("gültiger Stapel");
    assert_eq!(view.check_ver(v(1)), GateState::Erased, "Positivkontrolle");
}

/// Epoche ist die Sequenznummer des letzten Rahmens (SPEC 5.3).
#[test]
fn t_gate_epoch_is_sequence_of_last_frame() {
    let d = disk();
    let (mut w, view) = fresh(&d);
    assert_eq!(w.commit(&[GateOp::Supersede(v(1))]).expect("first"), 1);
    let epoch = w
        .commit(&[GateOp::Supersede(v(2)), GateOp::Supersede(v(3))])
        .expect("batch");
    assert_eq!(epoch, 3);
    assert_eq!(view.epoch(), 3);
}

#[test]
fn t_gate_empty_batch_is_rejected() {
    let d = disk();
    let (mut w, _view) = fresh(&d);
    assert!(matches!(w.commit(&[]), Err(GateError::EmptyBatch)));
}

/// SPEC 8.1, Betroffene: Takedown in K und Sperre neuer Zuordnung.
#[test]
fn t_gate_redacted_k_and_subject_erasure_are_visible() {
    let d = disk();
    let (mut w, view) = fresh(&d);
    w.commit(&[GateOp::RedactK(KOrd(42)), GateOp::EraseSubject(SubjOrd(3))])
        .expect("commit");
    assert!(view.is_redacted_k(KOrd(42)), "Positivkontrolle");
    assert!(!view.is_redacted_k(KOrd(43)), "nur das gesperrte Objekt");
    assert!(view.is_subject_erased(SubjOrd(3)), "Positivkontrolle");
    assert!(
        !view.is_subject_erased(SubjOrd(4)),
        "nur der gesperrte Betroffene"
    );
}

/// Z13 und SPEC 5.2 Ziffer 3: leere Herkunft ist nicht konstruierbar; Treffer nach Rangfolge.
#[test]
fn t_gate_lineage_rejects_empty_and_ranks_hits() {
    let empty = Lineage::new(RoaringBitmap::new(), RoaringTreemap::new());
    assert!(matches!(empty, Err(GateError::EmptyLineage)));

    let d = disk();
    let (mut w, view) = fresh(&d);
    w.commit(&[GateOp::Supersede(v(1))]).expect("supersede");

    let only_live: RoaringBitmap = [2u32].into_iter().collect();
    let live = Lineage::new(only_live, RoaringTreemap::new()).expect("lineage");
    assert_eq!(
        view.lineage_hit(&live),
        LineageHit::Live,
        "Positivkontrolle: leer ist Live"
    );

    let with_superseded: RoaringBitmap = [1u32, 2].into_iter().collect();
    let sup = Lineage::new(with_superseded, RoaringTreemap::new()).expect("lineage");
    assert_eq!(view.lineage_hit(&sup), LineageHit::Superseded);

    w.commit(&[GateOp::Restrict {
        ver: v(3),
        reason: RestrictReason::Contested,
    }])
    .expect("restrict");
    let with_restricted: RoaringBitmap = [1u32, 3].into_iter().collect();
    let res = Lineage::new(with_restricted, RoaringTreemap::new()).expect("lineage");
    assert_eq!(
        view.lineage_hit(&res),
        LineageHit::Restricted(RestrictReason::Contested),
        "Restricted schlägt Superseded"
    );

    w.commit(&[GateOp::RedactK(KOrd(7))]).expect("redact");
    let k_hit =
        Lineage::new([3u32].into_iter().collect(), [7u64].into_iter().collect()).expect("lineage");
    assert_eq!(
        view.lineage_hit(&k_hit),
        LineageHit::Redacted,
        "Redacted schlägt Restricted"
    );

    w.commit(&[GateOp::EraseVer {
        ver: v(3),
        reason: EraseReason::Takedown,
    }])
    .expect("erase");
    assert_eq!(
        view.lineage_hit(&k_hit),
        LineageHit::Erased,
        "Erased ist die stärkste Wirkung"
    );
}

/// SPEC 5.3: Hochwasser der Ordinale ist monoton; Gleichstand ist zulässig.
#[test]
fn t_gate_highwater_is_monotone() {
    let d = disk();
    let (mut w, _view) = fresh(&d);
    w.commit(&[GateOp::HighWater {
        kind: HighWaterKind::Ver,
        value: 10,
    }])
    .expect("first");
    w.commit(&[GateOp::HighWater {
        kind: HighWaterKind::Ver,
        value: 10,
    }])
    .expect("gleicher Wert ist zulässig");
    assert!(matches!(
        w.commit(&[GateOp::HighWater {
            kind: HighWaterKind::Ver,
            value: 9
        }]),
        Err(GateError::HighWaterRegression)
    ));
    // Andere Art, eigener Zähler: unabhängig.
    w.commit(&[GateOp::HighWater {
        kind: HighWaterKind::K,
        value: 1,
    }])
    .expect("K ist unabhängig von Ver");
}

/// Codes der Gründe sind stabil und umkehrbar; unbekannte Codes werden abgelehnt.
#[test]
fn t_gate_reason_codes_round_trip() {
    let restrict = [
        RestrictReason::Retention,
        RestrictReason::LegalHold,
        RestrictReason::Objection,
        RestrictReason::Contested,
        RestrictReason::MultiSubjectReview,
        RestrictReason::Precautionary,
    ];
    for r in restrict {
        assert_eq!(RestrictReason::from_code(r.code()), Some(r));
    }
    assert_eq!(RestrictReason::from_code(0), None);
    assert_eq!(RestrictReason::from_code(7), None);

    let erase: [EraseReason; 12] = [
        EraseReason::Request,
        EraseReason::Withdrawal,
        EraseReason::Objection,
        EraseReason::RetentionExpiry,
        EraseReason::PurposeEnded,
        EraseReason::Takedown,
        EraseReason::Offboarding,
        EraseReason::Departure,
        EraseReason::QuarantineExpiry,
        EraseReason::PayloadRetentionExpiry,
        EraseReason::HistoryHorizonExpiry,
        EraseReason::CourtOrder,
    ];
    for r in erase {
        assert_eq!(EraseReason::from_code(r.code()), Some(r));
    }
    assert_eq!(EraseReason::from_code(0), None);
    assert_eq!(EraseReason::from_code(13), None);
}
