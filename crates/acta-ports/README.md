# acta-ports

Abstrakte Port-Schnittstellen und Trait-Definitionen für Entkopplung.

Normativitätsklasse: **Kern `[K]`** (Ring 0, `stable`; §6.1, §6.9, Anhang A, `capabilities.toml`).

## Rolle in einem Satz
`acta-ports` definiert sämtliche abstrakten Port-Traits und Schnittstellenverträge zur sauberen Entkopplung von Kern und Peripherie ohne eigene Implementierungen.

## Ring, Klasse & Reifegrad
- **Ring:** Ring 0 (Port-Fundament, deterministisch)
- **Klasse:** Kern `[K]`
- **Reifegrad:** `stable`
- **Erlaubte Abhängigkeiten (`may_use` Obergrenze):** `acta-types` (Spec §6.9, `capabilities.toml`)
