import os

CRATES_META = {
    'acta-base': {
        'level': 0, 'budget': 2500, 'spec': 'docs/spec/02_grundmodell.md, 03_systemarchitektur.md, 04_kryptografie.md, 05_speicher.md',
        'desc': 'Basis-Newtypes (Ids, Ordinale), DataClass, ErrorClass, ClassifiedError, Ports Clock/CryptoRng, kanonische Kodierung, Festkomma fx',
        'arch_dir': 'archive/acta-base', 'has_arch': True,
        'steps': [
            'Typ- und Ordinal-Migration: Vollstaendige Migration der Newtypes nach SPEC 2.4 (16-Byte-Handles, streng monotone Ordinale, SubjectTag, ClusterId, IndexGen, ObjectId)',
            'Festkomma-Arithmetik: fx Q32.32 (Addition, Subtraktion mit Saettigung, Multiplikation, Division, fx_ln, isqrt) gemaess Z7 mit Goldvektoren verifizieren',
            'Ports Clock und CryptoRng: Strikte Abstraktion; CryptoRng fallibel via try_fill() (G3: keine direkten Systemzeit-/Zufallsaufrufe)',
            'Kanonische Kodierung: canon::Reader / canon::Writer fuer Big-Endian-Serialisierung ohne serde auf Plattenformaten (G-NOSERDE-DISK)',
            'Orakel-Tests: Uebernahme der invarianten Testsuiten aus archive/acta-base/ids/tests/'
        ]
    },
    'acta-access': {
        'level': 1, 'budget': 3000, 'spec': 'docs/spec/09_ingest_abruf_antwort.md, 12_mandanten_rollen_aufbewahrung.md',
        'desc': 'Mandanten, Rollen, ABAC-Attribute, Richtlinienauswertung, Sichtklassen, Zugriffsmengen A_K und A_P (Z16)',
        'arch_dir': 'archive/acta-access', 'has_arch': True,
        'steps': [
            'ABAC-Modell gemaess SPEC 9.4/12.1 umsetzen (allow/deny-Regeln ueber Subjekt- und Ressourcenattribute)',
            'Zugriffszellen und Roaring-Bitmaps: Verwaltung der Zell-Bitmaps fuer KOrd; Berechnung der effektiven Sichtklasse view_class und Zugriffsmenge A_K',
            'Sichtklassen-Isolation (Z16): Garantieren, dass unzugaengliche Dokumente weder Treffer noch Ranking noch Fehlermeldungen beeinflussen',
            'Filter-Logik aus archive/acta-access/types/src/types/filter.rs refaktorisieren und serde-frei einbinden',
            'Orakel-Tests aus types/tests/tenant_scope_isolation.rs als Z16-Garanten uebernehmen'
        ]
    },
    'acta-crypto': {
        'level': 1, 'budget': 4200, 'spec': 'docs/spec/04_kryptografie.md',
        'desc': 'Kryptografische Primitive: AEAD (XChaCha20-Poly1305), HKDF, HMAC, Ed25519, Sealed<P>, Zeroizing-Schluesseltypen, HashChain, Padding',
        'arch_dir': 'archive/acta-crypto', 'has_arch': True,
        'steps': [
            'Umschlag v1 gemaess SPEC 4.1 implementieren (XChaCha20-Poly1305, Klassen Personal/Knowledge/Derived, AAD-Bindung)',
            'Schluesselableitungshierarchie: HKDF-SHA256 fuer DEK(v), HK(v), DirKey, ClusterId, VK(t,g), K_tag(t) NFC-normalisiert',
            'Zeroizing & G-CRY: Alle Schluesseltypen kapseln rohes Material hermetisch ab (kein Clone/Debug/Serialize/Default auf Secrets)',
            'Ed25519-Signaturpruefung: Strikte Pruefung via verify_strict mit Domaenenpraefixen (acta/report/v1, acta/checkpoint/v1)',
            'RFC- und PyNaCl-Testvektoren aus archive/acta-crypto/ vollstaendig als Regressionstests integrieren'
        ]
    },
    'acta-fs': {
        'level': 1, 'budget': 2500, 'spec': 'docs/spec/03_systemarchitektur.md, 05_speicher.md, 16_test_freigabe.md',
        'desc': 'Dateisystemabstraktion: Vfs-Port, DurableVfs, FaultVfs, atomare Operationen (rename_atomic), Scrubbing (remove_scrub)',
        'arch_dir': 'archive/acta-fs', 'has_arch': True,
        'steps': [
            'Vfs-Portvertrag: Synchroner I/O-Kern gemaess SPEC 3.7 und 5.6 (WP-1 bis WP-7); kein async/tokio',
            'DurableVfs: Implementierung von rename_atomic mit Verzeichnis-fsync und remove_scrub (Nullen ueberschreiben + fsync)',
            'WP-7: FsyncFailed-Zustand verharrt irreversibel und verbietet Retries auf derselben Datei',
            'FaultVfs: Fehlersimulation fuer Crash-Matrix C01-C27 aus archive/acta-fs/testkit/src/fault_vfs.rs ueberfuehren',
            'Bereinigung: std::fs nur innerhalb von acta-fs kapseln (G-FS), parking_lot und crc32fast durch std/crc32c ersetzen'
        ]
    },
    'acta-gate': {
        'level': 1, 'budget': 1200, 'spec': 'docs/spec/03_systemarchitektur.md, 05_speicher.md',
        'desc': 'Zentrales Gate: ErasedView, GateWriter, unveraenderliche Roaring-Sperrmengen (ErasedSubjects, ErasedVers, RestrictedVers, RedactedK), ErasureLayer-Vertrag',
        'arch_dir': 'archive/acta-gate', 'has_arch': True,
        'steps': [
            'Gate-Datenstrukturen: Schnappschuesse ueber Arc mit atomarem Tausch (3.6) fuer O(1)-Pruefungen ohne I/O',
            'GateWriter: Fuehrung des append-only Gate-Logs mit HMAC-Kette und Gruppen-fsync (5.3)',
            'Sperrvorrang (Z4): ErasedVers > RestrictedVers > SupersededVers; RedactedK fuer Takedowns',
            'ErasureLayer-Trait (HR-3): Universeller Vertrag fuer Loeschkerndurchgriff auf alle Speicher und Caches',
            'Bereinigung: parking_lot eliminieren, synchrone Sperren gemaess Lock-Order verwenden'
        ]
    },
    'acta-infer': {
        'level': 1, 'budget': 700, 'spec': 'docs/spec/11_inferenz.md',
        'desc': 'Inferenz-Vertraege: Embedder, Reranker, Generator, Verifier, ModelFingerprint, ContextPurge',
        'arch_dir': 'archive/acta-infer', 'has_arch': True,
        'steps': [
            'Port-Definitionen gemaess SPEC 11.1 synchron definieren (Embedder, Reranker, Generator, Verifier)',
            'ModelFingerprint: Eindeutige Kennung fuer Modellfamilie, Versions-Hash, Quantisierung und Prompt-Template',
            'ContextPurge-Port (Z1): purge(request_id)-Schnittstelle zur Bereinigung ephemerer Modellzustaende',
            'Token-Counting und Budgetierungs-Vertraege nach SPEC 11.5'
        ]
    },
    'acta-ingest': {
        'level': 1, 'budget': 3000, 'spec': 'docs/spec/09_ingest_abruf_antwort.md',
        'desc': 'Dokumentaufbereitung vor Speicherung: Deterministisches Chunking (chunk_cfg v1), UAX #29 Normalisierung, Worker-Protokollrahmen',
        'arch_dir': 'archive/acta-ingest', 'has_arch': True,
        'steps': [
            'Reine Chunking-Funktion: (Bytes, chunk_cfg) -> Vec<Chunk> deterministisch implementieren (SPEC 9.1.1)',
            'Heading-Hierarchie & Normzitate: Erkennung und Pflege von heading_path und norm_ref Feldern',
            'Prozessgrenze zum Parser-Worker: Big-Endian Rahmenprotokoll fuer Sandbox-Worker (max 64 MiB)',
            'Bereinigung: blake3 entfernen, SHA-256 via acta-crypto nutzen'
        ]
    },
    'acta-parse': {
        'level': 1, 'budget': 5000, 'spec': 'docs/spec/09_ingest_abruf_antwort.md, 13_sicherheit_lieferkette.md',
        'desc': 'Dokumenten-Parser: PDF, DOCX/Office, HTML, Plaintext, Archive fuer nicht vertrauenswuerdige Daten',
        'arch_dir': 'archive/acta-parse', 'has_arch': True,
        'steps': [
            'Parser-Module aus archive/acta-parse/ uebernehmen (Text-, HTML-, Office-Extraktion)',
            'Ressourcen-Schranken strikt durchsetzen: Eingabe max 512 MiB, Dekompression max 100:1, Tiefe max 8',
            'Bereinigung: Keine direkte Dateisystem- oder Netzwerkinteraktion im Parser-Code (reine Speicherverarbeitung)',
            'Parser-Fuzzing-Tests und Fixtures als Orakel ueberfuehren'
        ]
    },
    'acta-parse-worker': {
        'level': 9, 'budget': 600, 'spec': 'docs/spec/09_ingest_abruf_antwort.md, 13_sicherheit_lieferkette.md',
        'desc': 'Isolierter Sandbox-Worker (Binary): Fuehrt acta-parse isoliert ohne Netz und ohne Schluessel aus',
        'arch_dir': 'archive/acta-parse-worker', 'has_arch': True,
        'steps': [
            'Prozessstart und IPC: Liest Dokumentendaten ueber stdin/Pipe und gibt Chunks/Metadaten strukturiert aus',
            'Sandboxing aktivieren: Linux Landlock und seccomp-Filter (via acta-sys) vor Dokumentenverarbeitung sperren',
            'Harte Ressourcenbeschraenkungen: rlimit fuer CPU-Zeit (60 s), RAM (2 GiB) setzen',
            'Keine Krypto- oder Mandantenschluessel im Prozessraum'
        ]
    },
    'acta-pii': {
        'level': 1, 'budget': 4000, 'spec': 'docs/spec/02_grundmodell.md, 07_entscheidungsprotokoll.md',
        'desc': 'Erkennung personenbezogener Daten: Regelbasierte PII-Erkennung, N-Validator-Bausteine, Validated<T>',
        'arch_dir': 'archive/acta-pii', 'has_arch': True,
        'steps': [
            'Erkennungsmuster: Deterministische Erkennung von Namen, Adressen, Aktenzeichen, Kennziffern (SPEC 2.2)',
            'N-Validator: Pruefung fuer Fragen und Auszuege; Validated<T> Typensiegel umsetzen',
            'Split-Modus-Unterstuetzung: Automatisierte Trennung in K- und P-Anteile nur bei bestandenem Test (ADR-02)',
            'Bereinigung: tracing eliminieren, interne Benchmarks und Regressionsmuster uebernehmen'
        ]
    },
    'acta-sys': {
        'level': 1, 'budget': 1600, 'spec': 'docs/spec/04_kryptografie.md, 13_sicherheit_lieferkette.md',
        'desc': 'Betriebssystem-Nahtstelle & einziges unsafe: mlock, MADV_DONTDUMP, SecureBuf, seccomp/Landlock, SIMD',
        'arch_dir': 'archive/acta-sys', 'has_arch': True,
        'steps': [
            'G-UNSAFE einhalten: Jeder unsafe Block traegt // SAFETY: Begruendung',
            'Speicherschutz: SecureBuf mit mlock() und madvise(MADV_DONTDUMP) gegen Swapping und Core-Dumps',
            'Sandboxing-Helfer: Landlock-Regelsatz und seccomp-Filter fuer acta-parse-worker bereitstellen',
            'Host-Pruefungen: Validierung von Entropiequellen, Zeitsynchronitaet und CPU-Merkmalen'
        ]
    },
    'acta-ledger': {
        'level': 2, 'budget': 2800, 'spec': 'docs/spec/06_wissensebene.md',
        'desc': 'Append-only-Protokolle mit kryptografischer Kette: K-, E- und A-Ledger, signierte Checkpoints, Commitments, RegistrySink/DirSink',
        'arch_dir': 'archive/acta-ledger', 'has_arch': True,
        'steps': [
            'Hash-Kette: SHA-256 Kette ueber Datensaetze gemaess SPEC 6.2 mit ChainKey(k_idx)',
            'Checkpointing: Signierte Checkpoints mit Ed25519 und Merkle-Root alle 1.024 Eintraege',
            'Senken: Trait RegistrySink; DirSink im synchronen Kern implementieren (ADR-29)',
            'Bereinigung: bincode und futures-util entfernen; Handkodierung v1 verwenden'
        ]
    },
    'acta-vault': {
        'level': 2, 'budget': 5500, 'spec': 'docs/spec/04_kryptografie.md',
        'desc': 'Mandantentresore: KeyAnchor, FileAnchor mit Argon2id, Wurzelrotation mit Generationen g, Sicherungsexport, Vernichtung',
        'arch_dir': 'archive/acta-vault', 'has_arch': True,
        'steps': [
            'Tresorformat: AAD-gebundene Speicherung (slot, Generation g), Index rein im RAM (ADR-26)',
            'Wurzelrotation: Generationen-Modell VK(t, g) mit Freeze-Phase (3.6) und Bereinigung alter Schluessel (ADR-07)',
            'Anker: FileAnchor mit Argon2id nach SPEC 4.1; KeyAnchor-Abstraktion fuer HSM/KMS',
            'R-KEY-FIRST: Schluessel durabel vor Datenverwendung schreiben; physische Vernichtung bei Loeschung',
            'Bereinigung: ahash, bincode, blake3, parking_lot eliminieren'
        ]
    },
    'acta-seg': {
        'level': 2, 'budget': 6500, 'spec': 'docs/spec/05_speicher.md',
        'desc': 'Versiegelter, schluesselfreier Segmentspeicher: WAL/Journal, Segmente (K, P, DK, DP), Manifest, Ersatzsegmente',
        'arch_dir': 'archive/acta-seg', 'has_arch': True,
        'steps': [
            'HR-9 strikt einhalten: acta-seg ist voellig schluesselfrei; Schluessel &SegmentKey kommen ausschliesslich als Parameter hinein',
            'Kanonische Formate: 64 KiB Bloecke, versiegelt mit Umschlag v1; handkodiert ohne serde (G-NOSERDE-DISK)',
            'Segment-Partitionierung (ADR-33): K (global), P (Cluster), DK (Lineage), DP (ClusterId)',
            'Altlasten tilgen: Legacy-WAL-Schluesselpfade und Alt-Magics entfernen (siehe ALTLASTEN.md)',
            'ErasureLayer implementieren: Gezielter Segmentersatz und Drop fuer Loeschstufe S3'
        ]
    },
    'acta-testkit': {
        'level': 2, 'budget': 5000, 'spec': 'docs/spec/16_test_freigabe.md',
        'desc': 'Testwerkzeuge & Mocks: ManualClock, SeededRng, Fehlerinjektion, Durabilitaetstests, Testdaten-Generatoren',
        'arch_dir': 'archive/acta-testkit', 'has_arch': True,
        'steps': [
            'ManualClock und SeededRng als praezise deterministische Testdoubles bereitstellen',
            'Crash- und Durabilitaets-Harness fuer Punkte C01 bis C27 umsetzen',
            'Proptest-Generatoren fuer alle Acta-Typen und Datensaetze zur Verfuegung stellen',
            'Bereinigung: parking_lot durch std-Primitive ersetzen'
        ]
    },
    'acta-pstore': {
        'level': 3, 'budget': 5000, 'spec': 'docs/spec/05_speicher.md',
        'desc': 'Personenbezogener Speicher (P-Speicher): Dokumentkopf HK(v), Versionen unter DEK(v), Betroffenenverzeichnis, Handle-Register, Vorgaenge (Case)',
        'arch_dir': '', 'has_arch': False,
        'steps': [
            'Dokument- und Versionsverwaltung: Speicherung von Inhaltsrecords unter HKDF-abgeleitetem DEK(v)',
            'Betroffenenverzeichnis: Datensatz je ClusterId unter DirKey(s) mit SubjOrd und Alias-Listen',
            'Handle-Register: PersonalRef -> (DocRef, VerRef, VerOrd, ClusterId) versiegelt unter RegistryKey(t)',
            'Vorgangsverwaltung (Case): Fallindex (Roaring-Bitmap) als DP-Segment zur Speisung von A_P',
            'Schnittstelle fuer ErasureLayer: Atomare Bereinigung bei Betroffenen- und Versionsloeschung'
        ]
    },
    'acta-history': {
        'level': 3, 'budget': 6500, 'spec': 'docs/spec/06_wissensebene.md',
        'desc': 'Git-aehnliche Versionskontrolle der Wissensebene K: Immutable Blobs, Trees, Commits, Refs, Sichtbarkeitsmengen, Takedown',
        'arch_dir': 'archive/acta-history', 'has_arch': True,
        'steps': [
            'Objektmodell: Blobs, Trees, Commits mit SHA-256 Hash-Identitaet und blob_salt (2.4)',
            'Sichtbarkeitsmengen: Roaring-Bitmap V je Commit fuer O(1) Zeitreisen ohne Reindexierung (ADR-04)',
            'Branching und Merging: Fast-Forward, 3-Wege-Merge, Vier-Augen-Freigabe fuer main',
            'Takedown-Prozess: RedactedK Platzhalter im K-Bestand verankern ohne History-Hashbruch (6.7)',
            'Bereinigung: ahash, bincode, parking_lot eliminieren'
        ]
    },
    'acta-text': {
        'level': 3, 'budget': 5000, 'spec': 'docs/spec/09_ingest_abruf_antwort.md',
        'desc': 'Segmentierter Volltextindex: BM25 in Festkomma Q32.32, DE/EN-Stemmer, sichtklassenrelative Statistik (Z7, Z16)',
        'arch_dir': 'archive/acta-text', 'has_arch': True,
        'steps': [
            'BM25 in Q32.32: Berechnung ueber fx_ln mit festem k1=1.2, b=0.75 ohne Gleitkomma (Z7)',
            'Sichtklassen-Statistik: N und df relativ zu V ∩ A; Vermeidung von Seitenkanaelen gemaess Z16',
            'Segmentierte Posting-Listen: Handkodiertes Plattenformat mit KOrd-Lineage',
            'ErasureLayer-Integration fuer DP-Textsegmente und Cache-Invalidierung',
            'Bereinigung: bincode, futures, parking_lot entfernen'
        ]
    },
    'acta-vec': {
        'level': 3, 'budget': 4000, 'spec': 'docs/spec/09_ingest_abruf_antwort.md',
        'desc': 'Segmentierter Vektorindex: Exakte Flat-Suche, i8-Symmetrische Quantisierung, Q32.32 Skalarprodukt (Z7)',
        'arch_dir': 'archive/acta-vec', 'has_arch': True,
        'steps': [
            'Quantisierung: i8-Vektoren mit Skalierungsfaktor s und norm2 als u32 vorhalten',
            'Aehnlichkeit: Deterministisches Q32.32 dot-product mit ganzzahligem isqrt gemaess SPEC 9.3',
            'Exakte Flat-Suche ueber Kandidatenmenge (K: V ∩ A_K, P: A_P ∩ Live)',
            'Segmentorganisation: Paralleler Scan ueber Segmente mit fester Gleichstandsordnung',
            'Bereinigung: ahash, blake3, parking_lot entfernen'
        ]
    },
    'acta-graph': {
        'level': 3, 'budget': 4000, 'spec': 'docs/spec/05_speicher.md, 09_ingest_abruf_antwort.md, anhang_f_adr.md',
        'desc': 'Segmentierter Graphindex: CSR-Struktur, Festkomma-PPR Q32.32, Pfad-RAG, Entitaetsextraktion, Lineage (ADR-34)',
        'arch_dir': 'archive/acta-graph', 'has_arch': True,
        'steps': [
            'CSR-Struktur (Compressed Sparse Row) synchron und versiegelt implementieren',
            'PPR (Personalized PageRank) deterministisch in Festkomma Q32.32 gemaess Z7 umsetzen (kein f32/f64)',
            'Pfad-RAG und Entitaetsextraktion fuer K- und P-Dokumente (P-Graph nur in DP-Segmenten, 5.5)',
            'ErasureLayer fuer Graph-Segmente und Takedown-Unterstuetzung integrieren',
            'Bereinigung: arc-swap, scc, ahash, parking_lot eliminieren; compat/ aufloesen'
        ]
    },
    'acta-erasure': {
        'level': 4, 'budget': 9000, 'spec': 'docs/spec/08_loeschkern.md',
        'desc': 'Loeschkern: 7-Stufen-Loeschung (S1-S7), Saga-Orchestrierung, Gate-Integration, Nachweismessung, Restore-Schutz',
        'arch_dir': 'archive/acta-erasure', 'has_arch': True,
        'steps': [
            'Saga-Zustandsmaschine fuer S1 (Sperre), S2 (Outbox), S3 (Bereinigung), S4 (Messung), S5 (Vernichtung), S6 (Ledger), S7 (Abschluss)',
            'Mehrbetroffenen-Planung: Erkennung und Loesung von Konflikten mit Zwischenzustand Restricted(MultiSubjectReview)',
            'Messung und Nachweis (Z11): Scanner mit Positivkontroll-Tests vor Schluesselvernichtung',
            'Restore-Schutz: Verhinderung des Wiederauflebens geloeschter Daten bei Backupeinspielung (8.8)',
            'Bereinigung: bincode, futures-util, parking_lot entfernen'
        ]
    },
    'acta-decision': {
        'level': 4, 'budget': 4500, 'spec': 'docs/spec/07_entscheidungsprotokoll.md',
        'desc': 'Entscheidungsprotokoll: Unveraenderliches Protokollieren, Szenarienanalyse, Akteursmodelle, Begruendungsketten',
        'arch_dir': 'archive/acta-decision', 'has_arch': True,
        'steps': [
            'Entscheidungseintrag: Begruendungskette (Wissensstand, Prompt, Modell, Belege, Eingabe)',
            'Entkopplung von Personendaten (Z8): Nutzlast liegt unter getrenntem DEK; Begruendung uebersteht P-Loeschung',
            'Szenarien: Isoliertes Verzweigen und Simulieren auf Wissensstaenden ohne Produktivwirkung (Z9)',
            'Akteure und ActorRef: Mitarbeiter-Handles ueber Epochen hinweg konsistent halten',
            'Bereinigung: ahash entfernen, direkte Anbindung an acta-ledger'
        ]
    },
    'acta-rag': {
        'level': 4, 'budget': 7000, 'spec': 'docs/spec/09_ingest_abruf_antwort.md',
        'desc': 'Abrufpipeline: Hybridsuche (Text, Vektor, Graph), Reciprocal Rank Fusion (RRF), Reranking, Kontextaufbau, Belegpruefung',
        'arch_dir': 'archive/acta-rag', 'has_arch': True,
        'steps': [
            'Hybridsuche & RRF: Parallele Abfrage von Text, Vektor und Graph; 6-Listen-Fusion gemaess SPEC 9.4',
            'Reranker-Anbindung mit Rundung auf Q32.32 an der Portgrenze',
            'Kontextaufbau: Budgetierung (ctx_len), Belegklammerung mit Zufallstoken gegen Prompt-Injection (T7)',
            'Belegpruefung (Z17): Lexikalische und semantische Stuetzungspruefung; Enthaltung wenn nicht belegt',
            'ErasureLayer: Implementierung fuer Antwort- und Sitzungscaches'
        ]
    },
    'acta-reports': {
        'level': 5, 'budget': 3500, 'spec': 'docs/spec/18_berichte_schnittstellen.md',
        'desc': 'Auditierbare Berichte: Loeschbericht (JCS), Art.-30-Verzeichnis, DSFA-Entwurf, Krypto- und Betriebsberichte, Wortfilter',
        'arch_dir': 'archive/acta-reports', 'has_arch': True,
        'steps': [
            'Loeschbericht nach RFC 8785 (JSON Canonicalization Scheme) mit kryptografischer Signatur generieren',
            'Art. 30 Verzeichnis und DSFA-Berichtsvorlagen strukturiert erzeugen',
            'Wortfilter-Pruefung gemaess SPEC 1.6 (Liste A und B) ueber alle Textausgaben erzwingen',
            'Bereinigung: blake3 und futures-util entfernen'
        ]
    },
    'acta-engine': {
        'level': 6, 'budget': 12000, 'spec': 'docs/spec/10_fachfunktionen.md, 19_lebenszyklen.md',
        'desc': 'Zentrale Geschaeftslogik: Orchestrierung der Lebenszyklen (Kap. 19), Fachfunktionen FF1-FF9, Mandantenverarbeitung',
        'arch_dir': 'archive/acta-engine', 'has_arch': True,
        'steps': [
            'Lebenszyklen gemaess SPEC 19 vollstaendig implementieren (Ingest, Query, Erase, Szenario, Maintenance)',
            'Fachfunktionen FF1 bis FF9 gemaess SPEC 10 einbinden',
            'Thread-Pools verwalten: writer(t), read, rewrite, maint gemaess SPEC 3.6',
            'Bereinigung: scc, parking_lot, blake3, tokio-util entfernen; synchrone Ausfuehrung garantieren'
        ]
    },
    'acta-node': {
        'level': 7, 'budget': 3000, 'spec': 'docs/spec/03_systemarchitektur.md, 14_betrieb.md',
        'desc': 'Composition Root: Konfiguration, Instanziierung aller Kernadapter, Selbsttest, Hintergrundaufgaben',
        'arch_dir': 'archive/acta-node', 'has_arch': True,
        'steps': [
            'HR-7 einhalten: Einzige Stelle, die konkrete Kernimplementierungen verdrahtet',
            'Node-Konfiguration: Laden aller Pflichtparameter aus Anhang A ohne stille Standardwerte',
            'Selbsttest und Integritaetspruefung beim Hochfahren (Crash-Recovery, Ledger-Validierung)',
            'Hintergrunddienste: Fristenpruefung, Gate-Verdichtung, Schluesselrotation orchestrieren'
        ]
    },
    'acta-infer-http': {
        'level': 8, 'budget': 3000, 'spec': 'docs/spec/11_inferenz.md',
        'desc': 'HTTP-Client fuer externe/lokale Inferenzdienste: OpenAI-kompatible Schnittstelle, Kontingentpruefung, tokio erlaubt',
        'arch_dir': 'archive/acta-infer-http', 'has_arch': True,
        'steps': [
            'Asynchroner HTTP-Client mit reqwest fuer Embedding-, Rerank- und Generation-Endpunkte',
            'Egress-Sicherheit (Z18): Strikte Pruefung gegen freigegebene Zieladressen',
            'Kontingentueberwachung (inference_budget) und Timeout-Behandlung',
            'Bereinigung: tracing kapseln, Fehler auf ActaError mappen'
        ]
    },
    'acta-sink-net': {
        'level': 8, 'budget': 600, 'spec': 'docs/spec/06_wissensebene.md',
        'desc': 'Netzwerk-Senke: Asynchrone Uebertragung von Checkpoints und Commitments ueber mTLS (RegistrySink-Port)',
        'arch_dir': '', 'has_arch': False,
        'steps': [
            'RegistrySink-Port asynchron ueber mTLS implementieren',
            'Sichere Uebertragung mit Wiederholungslogik und Backoff',
            'Strikte Egress-Pruefung gemaess Z18',
            'Nahtstelle zu acta-ledger'
        ]
    },
    'acta-api': {
        'level': 8, 'budget': 6000, 'spec': 'docs/spec/13_sicherheit_lieferkette.md, 18_berichte_schnittstellen.md',
        'desc': 'Oeffentliche Schnittstelle: REST/JSON, OIDC-Authentifizierung, mTLS, Streaming-Antworten, Epochenpruefung',
        'arch_dir': 'archive/acta-api', 'has_arch': True,
        'steps': [
            'HTTP-Routing und Controller mit axum umsetzen',
            'OIDC und mTLS Authentifizierung; Abbildung auf Mandant und Rollen (ABAC)',
            'Streaming-Antworten mit Rahmenpruefung der Gate-Epoche gemaess SPEC 9.5 (gate_propagation_max = 1)',
            'Ratenbegrenzung rate_limit_per_min zur Abwehr von Seitenkanalangriffen'
        ]
    },
    'acta-server': {
        'level': 9, 'budget': 700, 'spec': 'docs/spec/03_systemarchitektur.md',
        'desc': 'Server-Hauptprogramm (Binary): Einstiegspunkt, Signalbehandlung, Konfigurationsaufbau, Injektion der Kantenports',
        'arch_dir': '', 'has_arch': False,
        'steps': [
            'main()-Einstiegspunkt: Initialisierung von Logger, Konfiguration und Threadpools',
            'Injektion der Kantenports: acta-infer-http, acta-sink-net in acta-node einhaengen',
            'Graceful Shutdown bei SIGTERM/SIGINT: kontrollierter Ablauf aller laufenden Sagas'
        ]
    },
    'acta-cli': {
        'level': 9, 'budget': 3000, 'spec': 'docs/spec/14_betrieb.md, 18_berichte_schnittstellen.md',
        'desc': 'Kommandozeilenwerkzeug (Binary): Administration, Mandantenverwaltung, Berichtsabruf, Reparatur, Notfall-Tools',
        'arch_dir': '', 'has_arch': False,
        'steps': [
            'CLI-Befehlsstruktur mit clap aufbauen (admin, query, tenant, erase, report, repair)',
            'Direkte Interaktion mit acta-node und API',
            'Support-Paket-Generierung mit Z15-Canary-Schutz (keine Personendaten in Logs/Paketen)'
        ]
    },
    'acta-verify': {
        'level': 9, 'budget': 1200, 'spec': 'docs/spec/17_pruefer.md',
        'desc': 'Unabhaengiger Pruefer (Binary): Standalone-Tool ohne interne Workspace-Abhaengigkeiten zur Auditierung von Ledgern und Berichten',
        'arch_dir': 'archive/acta-verify', 'has_arch': True,
        'steps': [
            'Vollstaendig abhaengigkeitsfrei von internen Crates halten (eigenstaendige Prueflogik)',
            'Kryptografische Verifikation von JCS-Loeschberichten und Ed25519-Signaturen',
            'Verifikation der Hash-Ketten von K-, E- und A-Ledger-Dateien',
            'Bereinigung: bincode und blake3 entfernen'
        ]
    }
}

for crate, meta in sorted(CRATES_META.items()):
    plan_path = os.path.join('crates', crate, 'PLAN.md')
    has_arch_str = f"archive/{crate}/" if meta['has_arch'] else "Kein Archiv (Neubau)"
    manifest_link = f"[archive/{crate}/MANIFEST.md](../../archive/{crate}/MANIFEST.md)" if meta['has_arch'] else "-"
    
    steps_lines = []
    for i, s in enumerate(meta['steps']):
        if ":" in s:
            k, v = s.split(":", 1)
            steps_lines.append(f"{i+1}. **{k.strip()}:** {v.strip()}")
        else:
            steps_lines.append(f"{i+1}. {s.strip()}")
    steps_md = "\n".join(steps_lines)
    
    spec_primary = meta['spec'].split(',')[0].strip()
    
    content = f"""# Implementierungsplan: `{crate}`

> **Status:** Normativ fuer die Crate-Entwicklung · **Ziel:** Inkrementelle Ueberfuehrung aus `{has_arch_str}` in sauberen Acta-Produktionscode.  
> **Architekturvorgaben:** [docs/SPEC.md](../../docs/SPEC.md) · [{spec_primary}](../../{spec_primary}) · [CLAUDE.md](CLAUDE.md)

---

## 1. Einordnung & Spezifikation
- **Level:** L{meta['level']} (`rings.toml`)
- **Code-Budget:** {meta['budget']} LOC Produktionscode (G-HUB, strikt einzuhalten)
- **Spezifikationskapitel:** {meta['spec']}
- **Zweck & Aufgabe:** {meta['desc']}
- **Archiv-Quelle:** {manifest_link}

---

## 2. Abgrenzung & Architektur-Leitplanken
- **Keine Sammel-Crates (HR-1 bis HR-3):** Typen gehoeren dorthin, wo sie gebraucht werden. Keine Typen-Hubs.
- **Fehlerbehandlung (HR-4):** Genau ein eigener Fehlertyp (`thiserror`, `#[non_exhaustive]`, kein `Other(String)`), implementiert `ClassifiedError`.
- **Synchroner Kern (G-NOTOKIO):** Level 0 bis 7 sind strikt synchron. Kein `tokio`, kein `async`.
- **Speicher- und Krypto-Hygiene (G-CRY, G-UNSAFE, G-NOSERDE-DISK):**
  - Plattenformate sind handkodiert (Big-Endian, Umschlag v1), kein `serde`/`bincode`.
  - `unsafe` ist ausschliesslich in `acta-sys` erlaubt (mit `// SAFETY:`-Dokumentation).
  - Keine verbotenen Traits (`Clone`, `Debug`, `Serialize`, `Default`) auf Schluesseltypen.
- **Wortfilter (G-WORD):** Keine verbotenen Begriffe aus Liste A (siehe SPEC 1.6).

---

## 3. Umbau- und Implementierungsschritte

{steps_md}

---

## 4. Testkatalog & Freigabekriterien
1. **Zusagen-Konformitaet:** Mindestens ein zugeordneter Test gemaess Zusagenmatrix 16.3 / G-ZUSAGE.
2. **Positivkontrolle (16.4):** Durabilitaets- und Messpruefungen muessen bei defektem Verhalten deterministisch fehlschlagen.
3. **Workspace-Gates:** `cargo xtask gates` muss zu jedem Schritt vollstaendig gruen sein.
4. **Clippy & Lints:** `cargo clippy -p {crate} --all-targets -- -D warnings` ohne Befunde.
5. **Archiv-Bereinigung:** Nach vollstaendiger Ueberfuehrung und Validierung wird der Ordner `{has_arch_str}` geloescht.
"""
    with open(plan_path, 'w', encoding='utf-8') as out:
        out.write(content)
    print(f"Wrote {plan_path}")
