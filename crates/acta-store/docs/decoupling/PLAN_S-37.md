# Implementierungsplan S-37: acta-store Entkopplung

## 1. Übersicht
Ziel von S-37 ist die Entkopplung des LSM-Speicherkerns `acta-store` gemäß Ring 1 / Klasse B Spezifikation:
- Kern-Closure ⊆ {`acta-ids`, `acta-env`, `acta-sys`, `acta-mvcc`, `acta-crypto`}.
- `docid-128` wird auf `acta-ids` umgelenkt.
- Bitgleiche Fehlerabbildung über `StoreError` und `From<StoreError> for ActaError`.
- Unveränderte Golden-Formate für SSTable, WAL, Manifest und Rollback-Intent.

## 2. Arbeitspakete
- WP0: Baseline & Planungserfassung
- WP1: Golden-Korpus (G-I) verifizieren und sicherstellen
- WP2: StoreError vervollständigen & compat/error.rs
- WP3: Lokale Nähte & Schnittstellenverträge im Kern
- WP4a: Entkopplung LSM- und Compaction-Engines & Konfiguration
- WP4b: Entkopplung Scanner-Subsystem
- WP4c: Entkopplung WAL, SSTable, Manifest, Tenant-Codec und Hilfsmodule
- WP4d: Entkopplung LSM-Recovery mit Durability-Absicherung
- WP5: Manifest v1/v2 Prüfung
- WP6: Umstellung auf acta-ids und acta-env
- WP7: Standalone-Nachweis & Fassade
- WP8: Feature-Hygiene, Doku & Abschlussbericht
