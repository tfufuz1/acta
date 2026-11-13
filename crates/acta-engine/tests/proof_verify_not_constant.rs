//! # Integrationstest: proof verify not constant
//!
//! **Zweck:** Prüft spezifische Garantien, Transaktionskonsistenz und Fehlertoleranz der Engine.
//!
//! Gehört zur Testsuite von `acta-engine`.

use acta_crypto::deletion_proof::{CustodyLevel, DeletionProof};
use acta_ports::scanner::ScanTechnique;
use acta_ports::SystemClock;
use acta_testkit::mint_witness_for_tests;
use acta_types::{RequestContext, SagaRecord, TenantId, TxId};

// ANCHOR[acta-engine:TST29-001]
/// Testblock zur Überprüfung von `test_proof_verify_with_wrong_key_fails`.
#[test]
fn test_proof_verify_with_wrong_key_fails() {
    let clock = SystemClock::new();
    let ctx = RequestContext::new(TenantId::try_new(1).unwrap(), TxId(100));
    let saga = SagaRecord::new([0xAA; 32]);
    let real_key = b"correct_key_32_bytes_long_secret!";
    let wrong_key = b"wrong___key_32_bytes_long_secret!";

    let witness = vec![mint_witness_for_tests(
        "LsmMemtable",
        10,
        0,
        ScanTechnique::Structural,
    )];

    let proof = DeletionProof::issue(
        &ctx,
        &saga,
        &witness,
        &[],
        CustodyLevel::Local,
        real_key.as_slice(),
        &clock,
    )
    .unwrap();

    let is_valid = proof.verify(wrong_key.as_slice()).unwrap_or(false);
    assert!(
        !is_valid,
        "Verification with wrong key must return false, but returned true"
    );
}
