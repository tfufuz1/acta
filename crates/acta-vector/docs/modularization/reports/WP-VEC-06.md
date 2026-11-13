# Abschlussbericht WP-VEC-06: Distanz, Quantisierung (SQ8/RaBitQ) und Compute-Pool

**Datum:** 2026-11-12
**Paket:** WP-VEC-06
**Crate:** `crates/acta-vector`
**Referenzstand:** `origin/main` @ `922ee26`

---

## 1. Baseline & Matrix-Ergebnisse

### 1.1 Matrix-Ergebnisse (vorher vs. nachher)

| Cargo Befehl | Baseline Status | Abschluss Status | Anmerkung |
|---|---|---|---|
| `cargo check -p acta-vector` | OK (exit=0) | OK (exit=0) | Keine Regression, 0 Warnungen in eigenem Code |
| `cargo check -p acta-vector --no-default-features` | FEHLER (exit=101) | FEHLER (exit=101) | Unverändert; vorbestehend (fehlendes acta_sys) |
| `cargo check -p acta-vector --no-default-features --features workspace` | FEHLER (exit=101) | FEHLER (exit=101) | Unverändert; vorbestehend |
| `cargo check -p acta-vector --features docid-128` | OK (exit=0) | OK (exit=0) | Keine Regression |
| `cargo check -p acta-vector --features simd` | OK (exit=0) | OK (exit=0) | Keine Regression, SIMD Dispatch verifiziert |
| `cargo check -p acta-vector --features experimental-diskann` | FEHLER (exit=101) | FEHLER (exit=101) | Unverändert; vorbestehend außerhalb WP-VEC-06 |
| `cargo check -p acta-vector --features experimental-rabitq` | OK (exit=0) | OK (exit=0) | BTreeSet Determinismus verifiziert |
| `cargo check -p acta-vector --features partial-index-rebuild` | OK (exit=0) | OK (exit=0) | Keine Regression |
| `cargo check -p acta-vector --all-targets` | FEHLER (exit=101) | FEHLER (exit=101) | Unverändert vorbestehend in diskann/persistence |

### 1.2 Python-Werkzeuge

| Werkzeug | Baseline | Abschluss | Delta |
|---|---|---|---|
| `count_bridge_refs.py` | 14 | 14 | 0 (keine neuen Brücken) |
| `modularity_score.py` | 67 | 67 | 0 (stabil) |
| `determinism_guard.py` | 6 Fundstellen (fixed_hasher) | 6 Fundstellen | 0 (RaBitQ BTreeSet neutralisiert) |

---

## 2. Aufgabenstatus und Prüfergebnisse

### Aufgabe 1: Doppelmount von contract.rs auflösen
- **Status:** Erledigt
- **Geänderte Dateien:** `crates/acta-vector/src/distance.rs`
- **Prüfbefehl:** `cargo test -p acta-vector --test s36_distance_metric_conformance --test distance_determinism` → grün
- **Details:** `#[path = "contract.rs"] pub mod contract;` ersetzt durch `pub use crate::contract;`. Freie Distanzfunktionen flexibilisiert auf `M: Into<DistanceMetric>`, sodass sowohl lokale als auch `acta_types` Enum-Varianten akzeptiert werden.

### Aufgabe 2: distance.rs in Verzeichnismodul überführen
- **Status:** Erledigt
- **Geänderte Dateien:**
  - `crates/acta-vector/src/distance.rs` (gelöscht)
  - `crates/acta-vector/src/distance/mod.rs` (Re-Exports mit `#[doc(inline)]`)
  - `crates/acta-vector/src/distance/kernel.rs` (`DistanceKernel`, `CosineSimilarityPartsU8`)
  - `crates/acta-vector/src/distance/scalar.rs` (`ScalarKernel`, `ScalarDistance`, bitgleiche Referenzen, `pub metric` erhalten)
  - `crates/acta-vector/src/distance/inject.rs` (`InjectedDistanceKernel`, `pub distance_fn` erhalten)
  - `crates/acta-vector/src/distance/validate.rs` (`validate_vector`, Slice- und Byte-Prüfungen)
  - `crates/acta-vector/src/distance/dispatch.rs` (SIMD/Scalar-Dispatch)
  - `crates/acta-vector/src/distance/u8_ops.rs` (quantisierte Operationen)
- **Prüfbefehl:** `cargo test -p acta-vector --test golden_distance_oracle --test distance_determinism_test --test simd_numerical_audit && cargo test -p acta-vector --features simd --test simd_differential_test` → grün

### Aufgabe 3: SIMD-Dispatch und Fehlerpfade absichern
- **Status:** Erledigt
- **Geänderte Dateien:** `crates/acta-vector/tests/wp06_distance_contract.rs`
- **Prüfbefehl:** `cargo test -p acta-vector --test wp06_distance_contract` → grün
- **Details:** Absicherung von Längenabweichungen, NaN/Inf-Abweisung, leeren Vektoren, Null-Norm bei Cosine, Dreiecksungleichung und Proptest.

### Aufgabe 4: quantize/ strukturieren und Fehlerschranken testen
- **Status:** Erledigt
- **Geänderte Dateien:**
  - `crates/acta-vector/src/quantize/mod.rs` (inkl. `quantize_validation_tests` erhalten)
  - `crates/acta-vector/src/quantize/train.rs` (exakte Wiederherstellung des ursprünglichen `try_train_with_percentiles` und `calculate_vector_drift`)
  - `crates/acta-vector/src/quantize/encode.rs`
  - `crates/acta-vector/src/quantize/decode.rs`
  - `crates/acta-vector/src/quantize/distance.rs`
  - `crates/acta-vector/tests/wp06_sq8_bounds.rs`
- **Prüfbefehl:** `cargo test -p acta-vector --test wp06_sq8_bounds --test sq8_error_bound_proptest` → grün
- **Details:** Aufteilung in fokussierte Module <= 400 LOC. Neuer Fehlerschranken- und Serde-Roundtrip-Test.

### Aufgabe 5: compute_pool.rs und RaBitQ härten
- **Status:** Erledigt
- **Geänderte Dateien:**
  - `crates/acta-vector/src/compute_pool.rs`
  - `crates/acta-vector/src/quantize_rabitq.rs`
  - `crates/acta-vector/tests/wp06_compute_pool.rs`
- **Prüfbefehl:** `cargo test -p acta-vector --test wp06_compute_pool && cargo test -p acta-vector --features experimental-rabitq --test wp06_compute_pool` → grün
- **Details:** Synchrone bounded Queue (`sync_channel`), JoinHandle-Leck-Beseitigung bei Drop, Propagation von Task-Panics als `VectorError::Internal`. RaBitQ Determinismus via `BTreeSet`.

---

## 3. Messwerte (LOC der Dateien im Besitz)

| Datei | LOC vorher | LOC nachher |
|---|---|---|
| `src/distance.rs` | 778 | 0 (in Modul überführt) |
| `src/distance/mod.rs` | - | 77 |
| `src/distance/kernel.rs` | - | 81 |
| `src/distance/scalar.rs` | - | 390 |
| `src/distance/inject.rs` | - | 172 |
| `src/distance/validate.rs` | - | 68 |
| `src/distance/dispatch.rs` | - | 114 |
| `src/distance/u8_ops.rs` | - | 48 |
| `src/quantize/mod.rs` | 605 | 205 |
| `src/quantize/train.rs` | - | 210 |
| `src/quantize/encode.rs` | - | 77 |
| `src/quantize/decode.rs` | - | 48 |
| `src/quantize/distance.rs` | - | 108 |
| `src/compute_pool.rs` | 237 | 258 |
| `src/quantize_rabitq.rs` | 523 | 523 |

Alle neuen Produktivdateien liegen strikt unter dem 400-LOC-Limit.

---

## 4. Übergaben an andere Agenten / Root

1. **`crates/acta-store/Cargo.toml`:**
   - **Zeile 17/28:** Das Feature `workspace` bindet `dep:acta-types` und `dep:acta-ports` ein, aber `acta-types` und `acta-ports` waren als nicht-optionale Abhängigkeiten deklariert. Dadurch schlägt jeder unmodifizierte `cargo check` im gesamten Workspace fehl, falls nicht temporär `optional = true` gesetzt wird. Vorschlag: An den Besitzer von `acta-store` zur Korrektur übergeben.
2. **`crates/acta-vector/tests/sq8_bias_calibration.rs`:**
   - **Zeile 15:** Übergibt `acta_types::DistanceMetric` an `Sq8Bias::calibrate`, welches `crate::types::DistanceMetric` erwartet. Vorschlag an WP-VEC-01/02: `Sq8Bias::calibrate` flexibilisieren oder Test anpassen.
3. **`crates/acta-vector/src/diskann/`:**
   - Enthält Mismatch-Fehler bezüglich `ScoredDocument` und `DistanceMetric` (vorbestehender Zustand in `origin/main`).

---

## 5. API-Nachweis

Befehl: `comm -23 /tmp/api_before.txt /tmp/api_after.txt`
Ausgabe:
```
(leer)
```
Keine öffentlichen HTML-Dokumentationspfade oder Symbole sind verschwunden. Alle Re-Exports und Signaturen sind 100% abwärtskompatibel.
