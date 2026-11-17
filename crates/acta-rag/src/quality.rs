//! Regressionsgrenzen und `QualityDrift` (SPEC 9.7, 14, Anhang A).
//!
//! Ein Vergleich zwischen Basis und Kandidat prüft die Grenzen. Überschreitet eine Kennzahl ihre
//! Grenze, lautet der Status `QualityDrift`. Er verhindert die Aktivierung (Freigabe nach `main`,
//! Modell- oder Konfigurationswechsel).

use acta_base::fx::Fx;

/// Regressionsgrenze `0,02` in Q32.32, gerundet half-even (`0,02 · 2^32 = 85899345,92`).
pub const DROP_0_02: Fx = Fx::from_bits(85_899_346);

/// Regressionsgrenze `0,03` in Q32.32, gerundet half-even (`0,03 · 2^32 = 128849018,88`).
pub const DROP_0_03: Fx = Fx::from_bits(128_849_019);

/// Auswertung eines Goldsets (SPEC 9.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvalSummary {
    /// Recall@10.
    pub recall_at_10: Fx,
    /// Zitiergenauigkeit (SPEC 9.6).
    pub citation_accuracy: Fx,
    /// Enthaltungsgenauigkeit (SPEC 9.6).
    pub abstention_accuracy: Fx,
    /// Latenz p95 in Nanosekunden.
    pub latency_p95_ns: u64,
}

/// Regressionsgrenzen. Eine Verschlechterung gilt erst bei Überschreitung (strikt größer).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegressionLimits {
    /// Zulässiger Rückgang von Recall@10.
    pub recall_at_10_drop: Fx,
    /// Zulässiger Rückgang der Zitiergenauigkeit.
    pub citation_accuracy_drop: Fx,
    /// Zulässiger Rückgang der Enthaltungsgenauigkeit.
    pub abstention_accuracy_drop: Fx,
    /// Zulässiger relativer Anstieg der Latenz p95 in Prozent.
    pub latency_p95_increase_percent: u64,
}

impl RegressionLimits {
    /// Startwerte aus Anhang A: Recall@10 −0,02; Zitiergenauigkeit −0,02;
    /// Enthaltungsgenauigkeit −0,03; Latenz p95 +20 %.
    pub const ANHANG_A: Self = Self {
        recall_at_10_drop: DROP_0_02,
        citation_accuracy_drop: DROP_0_02,
        abstention_accuracy_drop: DROP_0_03,
        latency_p95_increase_percent: 20,
    };
}

/// Welche Kennzahlen die Grenze überschreiten.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Breaches {
    /// Recall@10 überschreitet die Grenze.
    pub recall_at_10: bool,
    /// Zitiergenauigkeit überschreitet die Grenze.
    pub citation_accuracy: bool,
    /// Enthaltungsgenauigkeit überschreitet die Grenze.
    pub abstention_accuracy: bool,
    /// Latenz p95 überschreitet die Grenze.
    pub latency_p95: bool,
}

impl Breaches {
    /// Ob mindestens eine Grenze überschritten ist.
    #[must_use]
    pub const fn any(&self) -> bool {
        self.recall_at_10 || self.citation_accuracy || self.abstention_accuracy || self.latency_p95
    }
}

/// Ergebnis des Regressionsvergleichs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriftStatus {
    /// Alle Kennzahlen liegen innerhalb der Grenzen.
    Within,
    /// Mindestens eine Grenze ist überschritten.
    QualityDrift(Breaches),
}

impl DriftStatus {
    /// Ob der Status die Aktivierung verhindert (SPEC 19: „Regression über Grenze verhindert
    /// Aktivierung“).
    #[must_use]
    pub const fn blocks_activation(&self) -> bool {
        matches!(self, Self::QualityDrift(_))
    }
}

/// Vergleicht Kandidat mit Basis anhand der Regressionsgrenzen.
#[must_use]
pub fn check_regression(
    baseline: &EvalSummary,
    candidate: &EvalSummary,
    limits: &RegressionLimits,
) -> DriftStatus {
    let breaches = Breaches {
        recall_at_10: dropped_beyond(
            baseline.recall_at_10,
            candidate.recall_at_10,
            limits.recall_at_10_drop,
        ),
        citation_accuracy: dropped_beyond(
            baseline.citation_accuracy,
            candidate.citation_accuracy,
            limits.citation_accuracy_drop,
        ),
        abstention_accuracy: dropped_beyond(
            baseline.abstention_accuracy,
            candidate.abstention_accuracy,
            limits.abstention_accuracy_drop,
        ),
        latency_p95: latency_beyond(
            baseline.latency_p95_ns,
            candidate.latency_p95_ns,
            limits.latency_p95_increase_percent,
        ),
    };
    if breaches.any() {
        DriftStatus::QualityDrift(breaches)
    } else {
        DriftStatus::Within
    }
}

fn dropped_beyond(baseline: Fx, candidate: Fx, limit: Fx) -> bool {
    baseline.sub(candidate) > limit
}

fn latency_beyond(baseline_ns: u64, candidate_ns: u64, increase_percent: u64) -> bool {
    // candidate > baseline · (100 + p) / 100, ganzzahlig und ohne Überlauf in u128.
    u128::from(candidate_ns) * 100 > u128::from(baseline_ns) * (100 + u128::from(increase_percent))
}

#[cfg(test)]
mod tests {
    use super::*;

    // SPEC: Anhang A (Startwerte). Die Konstanten entsprechen den gerundeten Brüchen.
    #[test]
    fn limit_constants_match_rounded_fractions() {
        assert_eq!(Fx::from_ratio(2, 100), Some(DROP_0_02));
        assert_eq!(Fx::from_ratio(3, 100), Some(DROP_0_03));
    }

    fn base() -> EvalSummary {
        EvalSummary {
            recall_at_10: Fx::from_ratio(9, 10).unwrap(),
            citation_accuracy: Fx::from_ratio(96, 100).unwrap(),
            abstention_accuracy: Fx::from_ratio(90, 100).unwrap(),
            latency_p95_ns: 1_000,
        }
    }

    #[test]
    fn identical_summaries_are_within_limits() {
        let b = base();
        let status = check_regression(&b, &b, &RegressionLimits::ANHANG_A);
        assert_eq!(status, DriftStatus::Within);
        assert!(!status.blocks_activation());
    }

    // SPEC: 9.7. Ein Rückgang genau auf der Grenze ist zulässig, darüber nicht.
    #[test]
    fn drop_exactly_at_limit_is_allowed_and_beyond_is_drift() {
        let b = base();
        let mut at_limit = b;
        at_limit.recall_at_10 = b.recall_at_10.sub(DROP_0_02);
        assert_eq!(
            check_regression(&b, &at_limit, &RegressionLimits::ANHANG_A),
            DriftStatus::Within
        );

        let mut beyond = b;
        beyond.recall_at_10 = b.recall_at_10.sub(DROP_0_02).sub(Fx::from_bits(1));
        let status = check_regression(&b, &beyond, &RegressionLimits::ANHANG_A);
        assert!(status.blocks_activation());
        assert!(matches!(
            status,
            DriftStatus::QualityDrift(Breaches {
                recall_at_10: true,
                citation_accuracy: false,
                abstention_accuracy: false,
                latency_p95: false
            })
        ));
    }

    #[test]
    fn improvements_never_breach() {
        let b = base();
        let mut better = b;
        better.recall_at_10 = Fx::ONE;
        better.latency_p95_ns = 10;
        assert_eq!(
            check_regression(&b, &better, &RegressionLimits::ANHANG_A),
            DriftStatus::Within
        );
    }

    // SPEC: Anhang A (Latenz p95 +20 %). 1200 ist genau +20 % und zulässig, 1201 nicht.
    #[test]
    fn latency_limit_is_relative_and_strict() {
        let b = base();
        let mut c = b;
        c.latency_p95_ns = 1_200;
        assert_eq!(
            check_regression(&b, &c, &RegressionLimits::ANHANG_A),
            DriftStatus::Within
        );
        c.latency_p95_ns = 1_201;
        let status = check_regression(&b, &c, &RegressionLimits::ANHANG_A);
        assert!(matches!(
            status,
            DriftStatus::QualityDrift(Breaches {
                latency_p95: true,
                ..
            })
        ));
    }

    #[test]
    fn breaches_report_every_violated_metric() {
        let b = base();
        let c = EvalSummary {
            recall_at_10: Fx::ZERO,
            citation_accuracy: Fx::ZERO,
            abstention_accuracy: Fx::ZERO,
            latency_p95_ns: u64::MAX,
        };
        match check_regression(&b, &c, &RegressionLimits::ANHANG_A) {
            DriftStatus::QualityDrift(br) => {
                assert!(
                    br.recall_at_10
                        && br.citation_accuracy
                        && br.abstention_accuracy
                        && br.latency_p95
                );
            }
            DriftStatus::Within => panic!("expected drift"),
        }
    }
}
