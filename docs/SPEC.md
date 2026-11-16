# Acta – Spezifikation

_Wissens- und Entscheidungsplattform mit versionierbarem Unternehmenswissen und entkoppeltem, nachweisbar löschbarem Personendatenschutz_  
_Fassung 2.1 · Stand 2026-11-16 (inkl. ADR-34 Graphindex) · Status: normativ · modularisiert in `docs/spec/`_

---

## Übersicht & Modulare Gliederung

Die Gesamtspezifikation ist modularisiert in thematische Kapiteldateien im Verzeichnis [`docs/spec/`](spec/). Die Gliederung folgt den definierten Architekturleveln und Kernverträgen. Nachfolgend sind alle Kapitel mit den jeweils zugeordneten Crates aufgeführt.

### Normative Kapitel und Crate-Zuordnung

| Kapitel | Dokument | Thema | Betroffene Crates (Level / Rolle) |
|---|---|---|---|
| **0** | [00_lesehinweise.md](spec/00_lesehinweise.md) | **Lesehinweise** (Konventionen, Vorrang, RFC 2119, Kennzeichnungen) | *Alle Crates* (Workspace-weit verbindlich) |
| **1** | [01_produkt.md](spec/01_produkt.md) | **Produkt** (Zweck, Zusagen Z1–Z19, Wortfilter) | *Alle Crates*, insb. `acta-reports`, `xtask` (G-WORD, G-ZUSAGE) |
| **2** | [02_grundmodell.md](spec/02_grundmodell.md) | **Grundmodell** (Vier Ebenen K/P/D/E, Quarantäne, Vertraulichkeit) | `acta-base` (L0), `acta-pstore` (L3), `acta-history` (L3) |
| **3** | [03_systemarchitektur.md](spec/03_systemarchitektur.md) | **Systemarchitektur** (Level L0–L9, Crate-Katalog, Hub-Regeln HR-1–HR-9, Nebenläufigkeit) | `acta-base` (L0), `acta-gate` (L1), `acta-fs` (L1), `acta-graph` (L3), `acta-node` (L7), `acta-server` (L9) |
| **4** | [04_kryptografie.md](spec/04_kryptografie.md) | **Kryptografie & Schlüsselverwahrung** (DEK, Root-Rotation, HKDF, Umschlag v1) | `acta-crypto` (L1), `acta-sys` (L1), `acta-base` (L0), `acta-vault` (L2) |
| **5** | [05_speicher.md](spec/05_speicher.md) | **Speicher** (Segmentspeicher, Gate-Log, P-Speicher, Ableitungen DK/DP, Schreibregeln WP-1–WP-7) | `acta-seg` (L2), `acta-gate` (L1), `acta-pstore` (L3), `acta-fs` (L1), `acta-graph` (L3) |
| **6** | [06_wissensebene.md](spec/06_wissensebene.md) | **Versionskontrolle K („Git für RAG“)** (Objekte, Ledger, Sichtbarkeit, Lineage) | `acta-history` (L3), `acta-ledger` (L2), `acta-sink-net` (L8) |
| **7** | [07_entscheidungsprotokoll.md](spec/07_entscheidungsprotokoll.md) | **Entscheidungsprotokoll & Akteure** (Entscheidungs-Ledger, Pseudonymisierung, Szenarien, Validator) | `acta-decision` (L4), `acta-pii` (L1) |
| **8** | [08_loeschkern.md](spec/08_loeschkern.md) | **Löschkern** (7-Stufen-Löschung S1–S7, Takedown, Messung, Sagas, Restore-Schutz) | `acta-erasure` (L4), `acta-gate` (L1), `acta-vault` (L2), `acta-pstore` (L3), `acta-seg` (L2) |
| **9** | [09_ingest_abruf_antwort.md](spec/09_ingest_abruf_antwort.md) | **Ingest, Abruf und Antwort** (Chunking, BM25, Vektorsuche, Graph-PPR, RRF-Fusion, Belegprüfung) | `acta-ingest` (L1), `acta-parse` (L1), `acta-parse-worker` (L9), `acta-text` (L3), `acta-vec` (L3), `acta-graph` (L3), `acta-access` (L1), `acta-rag` (L4) |
| **10** | [10_fachfunktionen.md](spec/10_fachfunktionen.md) | **Fachfunktionen** (Sachbearbeitung, Aktenprüfung, Fristenüberwachung) | `acta-engine` (L6) |
| **11** | [11_inferenz.md](spec/11_inferenz.md) | **Inferenz & Modell-Lebenszyklus** (Port-Vertrag `Generator`/`Embedder`, Kontingente, `IndexGen`) | `acta-infer` (L1), `acta-infer-http` (L8) |
| **12** | [12_mandanten_rollen_aufbewahrung.md](spec/12_mandanten_rollen_aufbewahrung.md) | **Mandanten, Rollen, Aufbewahrung** (ABAC, Mandantenisolation, Aufbewahrungsfristen) | `acta-access` (L1), `acta-engine` (L6) |
| **13** | [13_sicherheit_lieferkette.md](spec/13_sicherheit_lieferkette.md) | **Sicherheit, Souveränität & Lieferkette** (Egress-Sperre, Sandboxing, `cargo-deny`, mTLS) | `acta-sys` (L1), `acta-api` (L8), `acta-parse-worker` (L9) |
| **14** | [14_betrieb.md](spec/14_betrieb.md) | **Betrieb** (Bootstrap, Backups, Desaster Recovery, Monitoring, CLI) | `acta-node` (L7), `acta-cli` (L9) |
| **15** | [15_qualitaet_konfiguration.md](spec/15_qualitaet_konfiguration.md) | **Qualität, Fehler und Konfiguration** (Fehlerklassen, Vergiftung, Invarianten I1–I7) | `acta-base` (L0), *alle Crates* |
| **16** | [16_test_freigabe.md](spec/16_test_freigabe.md) | **Test und Freigabe** (Zusagenmatrix Z1–Z19, Crash-Matrix C01–C27, Architektur-Gates) | `acta-testkit` (dev), `xtask` (dev) |
| **17** | [17_pruefer.md](spec/17_pruefer.md) | **Unabhängiger Prüfer** (Externe Auditierbarkeit von Berichten & Ledgern) | `acta-verify` (Binary, L9 / extern) |
| **18** | [18_berichte_schnittstellen.md](spec/18_berichte_schnittstellen.md) | **Berichte und Schnittstellen** (Löschberichte JCS, Art. 30 Verzeichnis, DSFA, HTTP-API, CLI) | `acta-reports` (L5), `acta-api` (L8), `acta-cli` (L9) |
| **19** | [19_lebenszyklen.md](spec/19_lebenszyklen.md) | **Lebenszyklen** (Vollständige Spezifikation aller Workflows und Sagas) | `acta-engine` (L6) |
| **20** | [20_rechtliche_einordnung.md](spec/20_rechtliche_einordnung.md) | **Rechtliche Einordnung** (DSGVO Art. 5/17/25/32, Aufbewahrungsrecht, KI-VO) | *Konzeptionell / Compliance* |

---

### Anhänge

| Anhang | Dokument | Inhalt | Relevanz |
|---|---|---|---|
| **A** | [anhang_a_parameter.md](spec/anhang_a_parameter.md) | **Parameterverzeichnis** (Pflichtparameter, Startwerte A.1–A.9, inkl. `k_graph`) | Konfiguration & Engine |
| **B** | [anhang_b_automaten.md](spec/anhang_b_automaten.md) | **Zustandsautomaten** (Dokument-, Schlüssel-, Lösch- und Szenario-Status) | `acta-erasure`, `acta-pstore`, `acta-history` |
| **C** | [anhang_c_durabilitaet.md](spec/anhang_c_durabilitaet.md) | **Durabilitäts- und Absturzpunkte** (C01–C27 Fehlerinjektions-Matrix) | `acta-fs`, `acta-testkit`, Crash-Tests |
| **D** | [anhang_d_testkatalog.md](spec/anhang_d_testkatalog.md) | **Testkatalog** (Normative Testbezeichner und Pflichtfälle) | `acta-testkit`, CI |
| **E** | [anhang_e_fundstellen.md](spec/anhang_e_fundstellen.md) | **Fundstellenregister** (Normen, Urteile, RFCs, BSI-Standards) | Rechtliche Begründung & Compliance |
| **F** | [anhang_f_adr.md](spec/anhang_f_adr.md) | **Entscheidungsverzeichnis (ADR)** (ADR-01 bis ADR-34 inkl. Graphindex) | Architekturentscheidungen |

---

## 0. Lesehinweise

### 0.1 Normative Schlüsselwörter
**MUSS**, **DARF NICHT**, **SOLL**, **KANN** haben die Bedeutung nach RFC 2119.

### 0.2 Kennzeichnung der Aussagen
- **[E]** Verbindliche Architekturentscheidung dieser Spezifikation
- **[R]** Rechtliche Anforderung oder Einordnung, aus der eine technische Eigenschaft folgt
- **[Q]** Messbares Qualitätsziel (Startwert; Freigabe nur nach Messbericht)
- **[A]** Annahme über die Umgebung; im Betrieb zu bestätigen
- **[⚠]** Rechtlich offen oder umstritten; vorsichtige technische Wahl
- **[V]** Fundstelle oder Rechtstatsache vor Außenverwendung zu verifizieren

### 0.3 Vollständigkeits- und Vorrangregel [E]
1. **Default-deny:** Was diese Spezifikation nicht erlaubt, ist verboten. Keine stillen Standardwerte für Pflichtparameter (Anhang A) und keine Fallbacks im Sicherheitspfad (Z10).
2. **Vorrang bei scheinbarem Widerspruch:** Zusagen (1.5) vor Invarianten (15.4) vor Verfahren (Kap. 4–14) vor Parametern (Anhang A).
3. **Lebenszyklen sind vollständig:** Kapitel 19 definiert alle zulässigen Funktionen. Funktionen ohne Eintrag gelten als nicht spezifiziert.
