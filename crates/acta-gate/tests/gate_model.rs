//! Modellbasierte Eigenschaftstests des Gates (SPEC 16.4, Z1, Z4).
//!
//! Zufällige Operationsfolgen werden gegen ein einfaches Referenzmodell geprüft. Nach jeder
//! Folge wird das Log neu aufgebaut; der Wiederaufbau muss denselben Zustand ergeben.
#![allow(clippy::expect_used, clippy::unwrap_used)]

mod support;

use std::collections::{BTreeMap, BTreeSet};

use acta_base::ids::{KOrd, SubjOrd, VerOrd};
use acta_gate::{EraseReason, ErasedView, GateError, GateOp, GateState, RestrictReason};
use proptest::prelude::*;
use support::{disk, durable, fresh, open, FaultSink};

#[derive(Debug, Clone)]
enum Step {
    Subject(u32),
    Erase(u32),
    Restrict(u32, RestrictReason),
    Release(u32),
    Supersede(u32),
    Redact(u64),
}

const REASONS: [RestrictReason; 6] = [
    RestrictReason::Retention,
    RestrictReason::LegalHold,
    RestrictReason::Objection,
    RestrictReason::Contested,
    RestrictReason::MultiSubjectReview,
    RestrictReason::Precautionary,
];

fn step() -> impl Strategy<Value = Step> {
    prop_oneof![
        (0u32..6).prop_map(Step::Subject),
        (0u32..6).prop_map(Step::Erase),
        ((0u32..6), prop::sample::select(REASONS.to_vec())).prop_map(|(v, r)| Step::Restrict(v, r)),
        (0u32..6).prop_map(Step::Release),
        (0u32..6).prop_map(Step::Supersede),
        (0u64..4).prop_map(Step::Redact),
    ]
}

/// Referenzmodell mit denselben Regeln wie SPEC 5.3, aber ohne Roaring und ohne Log.
#[derive(Default)]
struct Model {
    subjects: BTreeSet<u32>,
    erased: BTreeSet<u32>,
    restricted: BTreeMap<u32, RestrictReason>,
    superseded: BTreeSet<u32>,
    redacted: BTreeSet<u64>,
}

impl Model {
    fn state(&self, v: u32) -> GateState {
        if self.erased.contains(&v) {
            GateState::Erased
        } else if let Some(r) = self.restricted.get(&v) {
            GateState::Restricted(*r)
        } else if self.superseded.contains(&v) {
            GateState::Superseded
        } else {
            GateState::Live
        }
    }

    /// Wendet einen Schritt an; liefert den erwarteten Ausgang.
    fn apply(&mut self, s: &Step) -> Result<(), GateError> {
        match *s {
            Step::Subject(x) => {
                self.subjects.insert(x);
                Ok(())
            }
            Step::Erase(v) => {
                self.erased.insert(v);
                self.restricted.remove(&v);
                Ok(())
            }
            Step::Restrict(v, r) => {
                if self.erased.contains(&v) {
                    return Err(GateError::EraseTerminal);
                }
                self.restricted.insert(v, r);
                Ok(())
            }
            Step::Release(v) => {
                if self.erased.contains(&v) {
                    return Err(GateError::EraseTerminal);
                }
                if self.restricted.remove(&v).is_none() {
                    return Err(GateError::NotRestricted);
                }
                Ok(())
            }
            Step::Supersede(v) => {
                if !self.erased.contains(&v) {
                    self.superseded.insert(v);
                }
                Ok(())
            }
            Step::Redact(k) => {
                self.redacted.insert(k);
                Ok(())
            }
        }
    }
}

fn to_op(s: &Step) -> GateOp {
    match *s {
        Step::Subject(x) => GateOp::EraseSubject(SubjOrd(x)),
        Step::Erase(v) => GateOp::EraseVer {
            ver: VerOrd(v),
            reason: EraseReason::Request,
        },
        Step::Restrict(v, reason) => GateOp::Restrict {
            ver: VerOrd(v),
            reason,
        },
        Step::Release(v) => GateOp::Release(VerOrd(v)),
        Step::Supersede(v) => GateOp::Supersede(VerOrd(v)),
        Step::Redact(k) => GateOp::RedactK(KOrd(k)),
    }
}

fn same_outcome(expected: &Result<(), GateError>, actual: &Result<u64, GateError>) -> bool {
    matches!(
        (expected, actual),
        (Ok(()), Ok(_))
            | (Err(GateError::EraseTerminal), Err(GateError::EraseTerminal))
            | (Err(GateError::NotRestricted), Err(GateError::NotRestricted))
    )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(96))]

    /// Jede Folge stimmt mit dem Modell überein, und der Wiederaufbau aus dem Log ebenso.
    #[test]
    fn gate_matches_reference_model_and_survives_recovery(steps in prop::collection::vec(step(), 1..40)) {
        let d = disk();
        let (mut w, view) = fresh(&d);
        let mut model = Model::default();

        for s in &steps {
            let expected = model.apply(s);
            let actual = w.commit(&[to_op(s)]);
            prop_assert!(same_outcome(&expected, &actual), "Ausgang weicht ab bei {s:?}: {actual:?}");
            for v in 0..6u32 {
                prop_assert_eq!(view.check_ver(VerOrd(v)), model.state(v), "Version {}", v);
            }
            for k in 0..4u64 {
                prop_assert_eq!(view.is_redacted_k(KOrd(k)), model.redacted.contains(&k));
            }
        }

        let (_, restored) = open(&durable(&d), FaultSink::ok(&d));
        for v in 0..6u32 {
            prop_assert_eq!(restored.check_ver(VerOrd(v)), model.state(v), "nach Wiederaufbau, Version {}", v);
        }
        for x in 0..6u32 {
            prop_assert_eq!(restored.is_subject_erased(SubjOrd(x)), model.subjects.contains(&x));
        }
    }
}

/// Positivkontrolle für den Modelltest: eine bekannte Folge ergibt den erwarteten Zustand,
/// sodass Übereinstimmung nicht zufällig durch einen leeren Zustand entsteht.
#[test]
fn model_positive_control_known_sequence() {
    let d = disk();
    let (mut w, view) = fresh(&d);
    let steps = [
        Step::Restrict(1, RestrictReason::Retention),
        Step::Supersede(2),
        Step::Erase(3),
        Step::Release(1),
    ];
    for s in &steps {
        w.commit(&[to_op(s)]).expect("commit");
    }
    assert_eq!(view.check_ver(VerOrd(1)), GateState::Live);
    assert_eq!(view.check_ver(VerOrd(2)), GateState::Superseded);
    assert_eq!(view.check_ver(VerOrd(3)), GateState::Erased);
}
