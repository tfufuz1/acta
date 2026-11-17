//! Erfüllungsklassen G, B, K und M (SPEC 8.7).
//!
//! Die Klassen bauen aufeinander auf (8.7): B setzt G voraus, K setzt B voraus, M setzt K voraus.
//! Der Bericht nennt je Klasse nur `reached` oder `not_reached`. Das Wort „gelöscht“ taucht hier
//! nicht auf; Aussagen über Löschung sind nur mit der erreichten Klasse zulässig (SPEC 1.6, Regel B).

use crate::measure::Measurement;

/// Erfüllungsklasse (SPEC 8.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Class {
    /// Gesperrt: Gate durabel, Sondierung liefert `Erased`.
    G,
    /// Bereinigt: S3 vollzogen, Messung Klasse B.
    B,
    /// Schlüssel vernichtet: S5 und S6, Canary-Probe, Sicherungsfenster geschlossen.
    K,
    /// Vollständig gemessen: K, Segment- und Journalmessung, Registersenke, alle Schichten erfasst.
    M,
}

/// Ergebnis einer Klasse für den Bericht.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Die Klasse ist erreicht.
    Reached,
    /// Die Klasse ist nicht erreicht. Auch ungemessene Prüfungen zählen so (Z11).
    NotReached,
}

impl Verdict {
    /// Stabile Bezeichnung für den Bericht.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reached => "reached",
            Self::NotReached => "not_reached",
        }
    }
}

/// Messergebnisse und Zustände, aus denen die Klassen abgeleitet werden.
///
/// Der Aufrufer liefert die Fakten. Dieser Crate führt keine Messung aus und kennt keine
/// Kennungen; er prüft nur die Ableitung.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassFacts {
    /// Gate-Sperre ist durabel (Z4).
    pub gate_durable: bool,
    /// Sondierungslesungen aller Lesepfade mit Positivkontrolle am Canary (8.5).
    pub read_probes: Measurement,
    /// S3 ist vollzogen.
    pub purge_done: bool,
    /// Messung Klasse B mit Positivkontrolle (8.5).
    pub plaintext: Measurement,
    /// S5 ist vollzogen: Schlüssel der Zielmenge vernichtet.
    pub key_destroyed: bool,
    /// S6 ist vollzogen: Rotation und `DestroyProof`.
    pub rotated: bool,
    /// Canary-Probe: Ergebnis `Passed` heißt, die Probe ist wie erwartet gescheitert.
    pub canary_probe: Measurement,
    /// Sicherungsfenster (`backup_exposure_until`) ist erreicht, Epochen vernichtet.
    pub backups_destroyed: bool,
    /// Segment- und Journalmessung ohne Zielschlüsselräume (8.5).
    pub segment_journal: Measurement,
    /// Registersenke bestätigt (`registry_mirrored`).
    pub registry_mirrored: bool,
    /// Alle Schichten sind gemessen oder deklariert.
    pub all_layers_covered: bool,
}

/// Bericht über alle vier Klassen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassReport {
    g: Verdict,
    b: Verdict,
    k: Verdict,
    m: Verdict,
}

impl ClassReport {
    /// Ergebnis einer Klasse.
    #[must_use]
    pub const fn get(&self, class: Class) -> Verdict {
        match class {
            Class::G => self.g,
            Class::B => self.b,
            Class::K => self.k,
            Class::M => self.m,
        }
    }
}

const fn verdict(reached: bool) -> Verdict {
    if reached {
        Verdict::Reached
    } else {
        Verdict::NotReached
    }
}

/// Leitet die Klassen aus den Fakten ab. Jede Klasse setzt die vorherige voraus (8.7).
#[must_use]
pub const fn evaluate(facts: &ClassFacts) -> ClassReport {
    let g = facts.gate_durable && facts.read_probes.is_passed();
    let b = g && facts.purge_done && facts.plaintext.is_passed();
    let k = b
        && facts.key_destroyed
        && facts.rotated
        && facts.canary_probe.is_passed()
        && facts.backups_destroyed;
    let m = k
        && facts.segment_journal.is_passed()
        && facts.registry_mirrored
        && facts.all_layers_covered;
    ClassReport {
        g: verdict(g),
        b: verdict(b),
        k: verdict(k),
        m: verdict(m),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::measure::Check;

    fn passed() -> Measurement {
        Measurement::new(Check::Passed, Check::Passed)
    }

    fn all_green() -> ClassFacts {
        ClassFacts {
            gate_durable: true,
            read_probes: passed(),
            purge_done: true,
            plaintext: passed(),
            key_destroyed: true,
            rotated: true,
            canary_probe: passed(),
            backups_destroyed: true,
            segment_journal: passed(),
            registry_mirrored: true,
            all_layers_covered: true,
        }
    }

    #[test]
    fn all_green_reaches_every_class() {
        let report = evaluate(&all_green());
        for class in [Class::G, Class::B, Class::K, Class::M] {
            assert_eq!(report.get(class), Verdict::Reached);
        }
    }

    #[test]
    fn verdict_strings_are_stable() {
        assert_eq!(Verdict::Reached.as_str(), "reached");
        assert_eq!(Verdict::NotReached.as_str(), "not_reached");
    }

    #[test]
    fn z11_failed_positive_control_on_plaintext_blocks_b_and_above() {
        let facts = ClassFacts {
            plaintext: Measurement::new(Check::Failed, Check::Passed),
            ..all_green()
        };
        let report = evaluate(&facts);
        assert_eq!(report.get(Class::G), Verdict::Reached);
        assert_eq!(report.get(Class::B), Verdict::NotReached);
        assert_eq!(report.get(Class::K), Verdict::NotReached);
        assert_eq!(report.get(Class::M), Verdict::NotReached);
    }

    #[test]
    fn z11_unmeasured_canary_probe_blocks_k() {
        let facts = ClassFacts {
            canary_probe: Measurement::new(Check::NotMeasured, Check::Passed),
            ..all_green()
        };
        let report = evaluate(&facts);
        assert_eq!(report.get(Class::K), Verdict::NotReached);
        assert_eq!(report.get(Class::B), Verdict::Reached);
    }

    #[test]
    fn open_backup_window_keeps_k_not_reached() {
        let facts = ClassFacts {
            backups_destroyed: false,
            ..all_green()
        };
        let report = evaluate(&facts);
        assert_eq!(report.get(Class::K), Verdict::NotReached);
        assert_eq!(report.get(Class::M), Verdict::NotReached);
        assert_eq!(report.get(Class::B), Verdict::Reached);
    }

    #[test]
    fn unregistered_mirror_keeps_m_not_reached() {
        let facts = ClassFacts {
            registry_mirrored: false,
            ..all_green()
        };
        let report = evaluate(&facts);
        assert_eq!(report.get(Class::K), Verdict::Reached);
        assert_eq!(report.get(Class::M), Verdict::NotReached);
    }

    #[test]
    fn non_durable_gate_blocks_everything() {
        let facts = ClassFacts {
            gate_durable: false,
            ..all_green()
        };
        let report = evaluate(&facts);
        for class in [Class::G, Class::B, Class::K, Class::M] {
            assert_eq!(report.get(class), Verdict::NotReached);
        }
    }

    #[test]
    fn classes_are_monotone_over_all_fact_combinations() {
        // Jede Kombination der Wahrheitswerte: M impliziert K, K impliziert B, B impliziert G.
        for bits in 0u32..(1 << 7) {
            let bit = |i: u32| (bits >> i) & 1 == 1;
            let facts = ClassFacts {
                gate_durable: bit(0),
                read_probes: if bit(1) {
                    passed()
                } else {
                    Measurement::new(Check::Failed, Check::Failed)
                },
                purge_done: bit(2),
                plaintext: if bit(3) {
                    passed()
                } else {
                    Measurement::new(Check::Passed, Check::Failed)
                },
                key_destroyed: bit(4),
                rotated: bit(5),
                canary_probe: if bit(6) {
                    passed()
                } else {
                    Measurement::new(Check::Passed, Check::Failed)
                },
                backups_destroyed: true,
                segment_journal: passed(),
                registry_mirrored: true,
                all_layers_covered: true,
            };
            let r = evaluate(&facts);
            let reached = |c: Class| r.get(c) == Verdict::Reached;
            assert!(!reached(Class::M) || reached(Class::K), "bits {bits:b}");
            assert!(!reached(Class::K) || reached(Class::B), "bits {bits:b}");
            assert!(!reached(Class::B) || reached(Class::G), "bits {bits:b}");
        }
    }
}
