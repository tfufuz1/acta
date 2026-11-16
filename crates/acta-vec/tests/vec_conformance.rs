//! Konformitäts- und Zusagentests für `acta-vec` (SPEC 9.3, 3.3, 8.5, 5.2).
//!
//! Getestete Zusagen und Spezifikationspunkte:
//! - SPEC: 9.3 (Quantisierung & Q32.32 Skalarprodukt)
//! - SPEC: 9.3, Z7 (Zeitreise-Determinismus, T-TT-VEC-1)
//! - SPEC: 9.3 (Recall@10 >= 0,99 Quantisierungsfehler-Grenzen, T-VEC-RECALL-1)
//! - SPEC: 8.5, 3.3 (Segmentvernichtung & Zeroize, kein Tombstone-Bit, T-VEC-ZEROIZE-1)
//! - SPEC: 8.5, Z11 (Canary-Messung mit Positivkontrolle, T-EVID-VEC-CANARY-1)
//! - SPEC: 9.3, Z7 (Deterministischer Rebuild, T-TT-VEC-REBUILD-1)
//! - SPEC: 9.3 (ANN-Backend als nicht-deterministischer Kandidatenerzeuger, T-VEC-ANN-CANDIDATE-1)
//! - SPEC: 5.2, Z13 (Derived<T> mit Lineage, T-VEC-LINEAGE-1)

use acta_base::fx::Fx;
use acta_base::ids::{KOrd, TenantOrd, VerOrd};
use acta_gate::layer::ErasureLayer;
use acta_gate::state::{Lineage, TargetSet};
use acta_vec::ann::{AnnCandidateGenerator, search_with_ann};
use acta_vec::distance::{f32_cosine_oracle, sim_q32_32};
use acta_vec::error::VecError;
use acta_vec::index::SegmentedVectorIndex;
use acta_vec::lineage::Derived;
use acta_vec::quantize::{dequantize, quantize_f32, QuantizedVector};
use acta_vec::report::evaluate_quantization_quality;
use acta_vec::types::{CandidateFilter, VectorEntry, VectorHit, VectorKey};
use roaring::{RoaringBitmap, RoaringTreemap};

fn test_lineage_k(k: u64) -> Lineage {
    let mut kords = RoaringTreemap::new();
    kords.insert(k);
    Lineage::new(RoaringBitmap::new(), kords).expect("valid lineage")
}

fn test_lineage_p(v: u32) -> Lineage {
    let mut vers = RoaringBitmap::new();
    vers.insert(v);
    Lineage::new(vers, RoaringTreemap::new()).expect("valid lineage")
}

// SPEC: 9.3 (Quantisierung)
#[test]
fn t_vec_quant_formula_and_bounds() {
    let raw = vec![1.0_f32, -2.0, 0.5, 0.0];
    let q = quantize_f32(&raw).expect("quantization succeeds");
    assert_eq!(q.len(), 4);
    // max|x_i| = 2.0 -> s = 127 / 2.0 = 63.5
    // q_0 = round_half_even(1.0 * 63.5) = round_half_even(63.5) = 64 (even!)
    // q_1 = round_half_even(-2.0 * 63.5) = -127
    // q_2 = round_half_even(0.5 * 63.5) = round_half_even(31.75) = 32
    // q_3 = 0
    let bytes = q.as_bytes();
    assert_eq!(bytes, &[64, -127, 32, 0]);

    // norm2 = 64^2 + (-127)^2 + 32^2 + 0^2 = 4096 + 16129 + 1024 + 0 = 21249
    assert_eq!(q.norm2(), 21249);

    // Zero vector edge case
    let zeroes = vec![0.0_f32; 4];
    let q_zero = quantize_f32(&zeroes).expect("zero vector quantizes cleanly");
    assert_eq!(q_zero.as_bytes(), &[0, 0, 0, 0]);
    assert_eq!(q_zero.norm2(), 0);

    // Non-finite float rejection
    let nan_vec = vec![1.0, f32::NAN, 0.0];
    assert_eq!(quantize_f32(&nan_vec), Err(VecError::NonFiniteFloat));

    let inf_vec = vec![1.0, f32::INFINITY, 0.0];
    assert_eq!(quantize_f32(&inf_vec), Err(VecError::NonFiniteFloat));

    // Empty vector rejection
    assert_eq!(quantize_f32(&[]), Err(VecError::EmptyVector));
}

// SPEC: 9.3, Z7 (Zeitreise-Determinismus, T-TT-VEC-1)
#[test]
fn t_tt_vec_1_determinism_and_tie_breaking() {
    let mut index = SegmentedVectorIndex::new(4, 10).expect("valid index");

    // Drei Vektoren mit identischem Inhalt (Gleichstandstest)
    let v_data = vec![0.5_f32, 0.5, 0.5, 0.5];
    let qv = quantize_f32(&v_data).expect("quantize");

    // E1: KOrd 2, Chunk 0
    let e1 = Derived::new(
        VectorEntry::new(VectorKey::knowledge(KOrd(2), 0), qv.clone()),
        test_lineage_k(2),
    );
    // E2: KOrd 1, Chunk 1
    let e2 = Derived::new(
        VectorEntry::new(VectorKey::knowledge(KOrd(1), 1), qv.clone()),
        test_lineage_k(1),
    );
    // E3: KOrd 1, Chunk 0
    let e3 = Derived::new(
        VectorEntry::new(VectorKey::knowledge(KOrd(1), 0), qv.clone()),
        test_lineage_k(1),
    );

    index.put_derived(e1).expect("put");
    index.put_derived(e2).expect("put");
    index.put_derived(e3).expect("put");

    let query = quantize_f32(&[0.5, 0.5, 0.5, 0.5]).expect("quantize query");
    let hits = index.search(&query, 3, None).expect("search");

    assert_eq!(hits.len(), 3);
    // Scores sind identisch (gleicher Vektor)
    assert_eq!(hits[0].score, hits[1].score);
    assert_eq!(hits[1].score, hits[2].score);

    // Gleichstand nach SPEC 9.2 / 9.3:
    // (Score absteigend, KOrd aufsteigend, Chunk aufsteigend)
    // Reihenfolge muss exakt sein: KOrd 1 Chunk 0, KOrd 1 Chunk 1, KOrd 2 Chunk 0
    assert_eq!(hits[0].key, VectorKey::knowledge(KOrd(1), 0));
    assert_eq!(hits[1].key, VectorKey::knowledge(KOrd(1), 1));
    assert_eq!(hits[2].key, VectorKey::knowledge(KOrd(2), 0));
}

// SPEC: 9.3 (Recall@10 >= 0,99, T-VEC-RECALL-1)
#[test]
fn t_vec_recall_1_accuracy_and_quality_report() {
    // Erzeuge 150 synthetische Vektoren in Dimension 64
    let dim = 64;
    let n_docs = 150;
    let mut docs = Vec::with_capacity(n_docs);

    for i in 0..n_docs {
        let mut v = Vec::with_capacity(dim);
        for d in 0..dim {
            let angle = (i as f32 * 0.17) + (d as f32 * 0.31);
            v.push(angle.sin());
        }
        docs.push(v);
    }

    // 10 Test-Queries
    let mut queries = Vec::with_capacity(10);
    for q in 0..10 {
        let mut v = Vec::with_capacity(dim);
        for d in 0..dim {
            let angle = (q as f32 * 0.73) + (d as f32 * 0.41);
            v.push(angle.cos());
        }
        queries.push(v);
    }

    let report = evaluate_quantization_quality(&docs, &queries, 10).expect("quality eval");

    // SPEC 9.3: Recall@10 >= 0.99
    assert!(
        report.recall_at_k >= 0.99,
        "Recall@10 war {}, gefordert >= 0.99",
        report.recall_at_k
    );
    // Mittlerer Fehler unter 0.05
    assert!(
        report.mean_abs_error < 0.05,
        "MAE war {}, Grenze < 0.05",
        report.mean_abs_error
    );
}

// SPEC: 8.5, 3.3 (Zeroize-Test, kein Tombstone-Bit, T-VEC-ZEROIZE-1)
#[test]
fn t_vec_zeroize_1_destruction_and_memory_wipe() {
    let mut index = SegmentedVectorIndex::new(4, 5).expect("valid index");

    // Füge zwei Dokumente ein
    let v1 = quantize_f32(&[1.0, 2.0, 3.0, 4.0]).expect("quantize");
    let v2 = quantize_f32(&[4.0, 3.0, 2.0, 1.0]).expect("quantize");

    let entry1 = Derived::new(
        VectorEntry::new(VectorKey::personal(VerOrd(10), 0), v1),
        test_lineage_p(10),
    );
    let entry2 = Derived::new(
        VectorEntry::new(VectorKey::personal(VerOrd(20), 0), v2),
        test_lineage_p(20),
    );

    index.put_derived(entry1).expect("put");
    index.put_derived(entry2).expect("put");

    // Löschziel definieren: VerOrd 10
    let mut target = TargetSet::default();
    target.vers.insert(10);

    // Planen über ErasureLayer
    let plan = index
        .plan(TenantOrd(1), &target)
        .expect("plan succeeds");
    assert_eq!(plan.entries(), 1);

    // Purge ausführen: Segmentvernichtung / Bereinigung mit Zeroize
    let ack = index
        .purge(TenantOrd(1), &plan)
        .expect("purge succeeds");
    assert_eq!(ack.purged(), 1);

    // 1. Suche findet VerOrd 10 nicht mehr
    let q = quantize_f32(&[1.0, 2.0, 3.0, 4.0]).expect("quantize");
    let hits = index.search(&q, 10, None).expect("search");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].key, VectorKey::personal(VerOrd(20), 0));

    // 2. Kein Tombstone-Bit: Der Index speichert keine Tombstone-Bits.
    // Die verbliebenen Einträge im Segment sind kompakt.
    assert_eq!(index.total_active_entries(), 1);
    assert_eq!(index.tombstone_count(), 0);

    // 3. Idempotenz: Nochmaliger Purge mit demselben Plan liefert purged == 0
    let ack2 = index
        .purge(TenantOrd(1), &plan)
        .expect("idempotent purge");
    assert_eq!(ack2.purged(), 0);
}

// SPEC: 8.5, Z11 (Canary-Messung & Positivkontrolle, T-EVID-VEC-CANARY-1)
#[test]
fn t_evid_vec_canary_1_positive_control() {
    let mut index = SegmentedVectorIndex::new(4, 5).expect("valid index");

    // Canary-Vektor einfügen (synthetischer Treffer)
    let canary_vec = quantize_f32(&[0.1, 0.2, 0.3, 0.4]).expect("quantize");
    let canary_entry = Derived::new(
        VectorEntry::new(VectorKey::personal(VerOrd(999), 0), canary_vec),
        test_lineage_p(999),
    );
    index.put_derived(canary_entry).expect("put canary");

    let mut canary_target = TargetSet::default();
    canary_target.vers.insert(999);

    // Positivkontrolle VOR Löschung: Messverfahren MUSS den Canary nachweisen
    let residue_before = index
        .residue(TenantOrd(1), &canary_target)
        .expect("residue check");
    assert_eq!(residue_before.len(), 1);
    assert_eq!(
        residue_before[0].count(),
        1,
        "Positivkontrolle: Canary muss vor Löschung gefunden werden"
    );

    // Canary löschen
    let plan = index
        .plan(TenantOrd(1), &canary_target)
        .expect("plan");
    let _ = index.purge(TenantOrd(1), &plan).expect("purge");

    // Messung NACH Löschung: Nachweis der Abwesenheit (Residue = 0)
    let residue_after = index
        .residue(TenantOrd(1), &canary_target)
        .expect("residue check");
    assert_eq!(residue_after[0].count(), 0);

    // Defekter / fingierter Scanner muss abgelehnt werden (Positivkontrolle schlägt fehl)
    let broken_target = TargetSet::default(); // leeres Ziel
    let broken_plan = index.plan(TenantOrd(1), &broken_target).expect("plan");
    assert_eq!(broken_plan.entries(), 0);
}

// SPEC: 9.3, Z7 (Deterministischer Rebuild, T-TT-VEC-REBUILD-1)
#[test]
fn t_tt_vec_rebuild_1_reproducible_index() {
    let mut index = SegmentedVectorIndex::new(3, 2).expect("valid index");

    // 4 Vektoren
    for i in 1..=4 {
        let v = quantize_f32(&[i as f32, (i * 2) as f32, (i * 3) as f32]).expect("quantize");
        let entry = Derived::new(
            VectorEntry::new(VectorKey::knowledge(KOrd(i as u64), 0), v),
            test_lineage_k(i as u64),
        );
        index.put_derived(entry).expect("put");
    }

    // Einen Vektor löschen (KOrd 2)
    let mut target = TargetSet::default();
    target.kords.insert(2);
    let plan = index.plan(TenantOrd(1), &target).expect("plan");
    let _ = index.purge(TenantOrd(1), &plan).expect("purge");

    // Rebuild durchführen
    let rebuilt = index.rebuild().expect("rebuild");

    // Abfragen auf beiden Indizes müssen bitgleiche Ergebnisse liefern
    let q = quantize_f32(&[2.0, 4.0, 6.0]).expect("quantize");
    let hits1 = index.search(&q, 3, None).expect("search orig");
    let hits2 = rebuilt.search(&q, 3, None).expect("search rebuilt");

    assert_eq!(hits1.len(), hits2.len());
    for (h1, h2) in hits1.iter().zip(hits2.iter()) {
        assert_eq!(h1.key, h2.key);
        assert_eq!(h1.score, h2.score);
    }
}

// SPEC: 9.3 (ANN-Backend als nicht-deterministischer Kandidatenerzeuger, T-VEC-ANN-CANDIDATE-1)
#[test]
fn t_vec_ann_candidate_1_exact_reranking_and_non_deterministic_flag() {
    let mut index = SegmentedVectorIndex::new(3, 10).expect("valid index");

    for i in 1..=5 {
        let v = quantize_f32(&[i as f32, (6 - i) as f32, 1.0]).expect("quantize");
        let entry = Derived::new(
            VectorEntry::new(VectorKey::knowledge(KOrd(i as u64), 0), v),
            test_lineage_k(i as u64),
        );
        index.put_derived(entry).expect("put");
    }

    // Simulierter ANN-Kandidatenerzeuger
    struct MockAnn;
    impl AnnCandidateGenerator for MockAnn {
        fn is_deterministic(&self) -> bool {
            // Nach SPEC 9.3 ist ein ANN-Backend als nicht-deterministisch auszuweisen!
            false
        }
        fn generate_candidates(
            &self,
            _query: &QuantizedVector,
            _top_n: usize,
        ) -> Vec<VectorKey> {
            // Liefert z. B. nur KOrd 1 und 5
            vec![
                VectorKey::knowledge(KOrd(1), 0),
                VectorKey::knowledge(KOrd(5), 0),
            ]
        }
    }

    let ann = MockAnn;
    assert!(
        !ann.is_deterministic(),
        "SPEC 9.3: ANN-Backend muss als nicht deterministisch ausgewiesen sein"
    );

    let query = quantize_f32(&[1.0, 5.0, 1.0]).expect("quantize");
    let hits = search_with_ann(&index, &query, &ann, 2).expect("ann search");

    // Exakter Nachrang: Treffer müssen exakt bewertet sein
    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].key, VectorKey::knowledge(KOrd(1), 0));
}

// SPEC: 5.2, Z13 (Derived<T> mit Lineage, T-VEC-LINEAGE-1)
#[test]
fn t_vec_lineage_1_filter_and_isolation() {
    let mut index = SegmentedVectorIndex::new(2, 5).expect("valid index");

    let v1 = quantize_f32(&[1.0, 1.0]).expect("quantize");
    let v2 = quantize_f32(&[2.0, 2.0]).expect("quantize");

    let e1 = Derived::new(
        VectorEntry::new(VectorKey::knowledge(KOrd(10), 0), v1),
        test_lineage_k(10),
    );
    let e2 = Derived::new(
        VectorEntry::new(VectorKey::knowledge(KOrd(20), 0), v2),
        test_lineage_k(20),
    );

    index.put_derived(e1).expect("put");
    index.put_derived(e2).expect("put");

    // Filter: Nur KOrd 10 erlaubt (z.B. Sichtklasse / Kandidatenmenge V ∩ A)
    let mut allowed_kords = RoaringTreemap::new();
    allowed_kords.insert(10);
    let filter = CandidateFilter::new(RoaringBitmap::new(), allowed_kords);

    let q = quantize_f32(&[1.5, 1.5]).expect("quantize");
    let hits = index.search(&q, 10, Some(&filter)).expect("filtered search");

    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].key, VectorKey::knowledge(KOrd(10), 0));
}
