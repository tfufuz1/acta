//! # Integrationstest: tenant collection handle sharing
//!
//! **Zweck:** Prüft spezifische Garantien, Transaktionskonsistenz und Fehlertoleranz der Engine.
//!
//! Gehört zur Testsuite von `acta-engine`.

#![cfg(not(loom))]
use acta_engine::{Acta, ActaConfig};
use acta_types::TenantId;
use tempfile::tempdir;

// ANCHOR[acta-engine:TST34-001]
/// Testblock zur Überprüfung von `test_tenant_handles_are_shared_for_same_tenant_and_collection`.
#[tokio::test]
async fn test_tenant_handles_are_shared_for_same_tenant_and_collection(
) -> acta_types::Result<()> {
    let dir = tempdir().unwrap();
    let db = Acta::open_with_config(dir.path(), ActaConfig::default()).await?;

    let tenant_1 = TenantId::try_new(101).unwrap();
    let col_a1 = db.collection_for_tenant("col_a", tenant_1).await?;
    let col_a2 = db.collection_for_tenant("col_a", tenant_1).await?;

    // Arc comparison: both handles must point to the exact same Collection instance
    assert!(
        std::sync::Arc::ptr_eq(&col_a1, &col_a2),
        "Handles for same tenant and collection name must be identical (shared Arc)"
    );

    // Insert via handle 1
    let vec = vec![0.1f32; 768];
    col_a1
        .insert(
            "doc1",
            &vec,
            Some(serde_json::json!({"text": "test document"})),
        )
        .await?;

    // Verify immediately visible in handle 2
    let doc = col_a2.get("doc1").await?;
    assert!(
        doc.is_some(),
        "Insertion via col_a1 must be immediately visible in col_a2"
    );

    db.close().await?;
    Ok(())
}

// ANCHOR[acta-engine:TST34-002]
/// Testblock zur Überprüfung von `test_tenant_handles_are_isolated_for_different_tenants`.
#[tokio::test]
async fn test_tenant_handles_are_isolated_for_different_tenants() -> acta_types::Result<()> {
    let dir = tempdir().unwrap();
    let db = Acta::open_with_config(dir.path(), ActaConfig::default()).await?;

    let tenant_1 = TenantId::try_new(101).unwrap();
    let tenant_2 = TenantId::try_new(102).unwrap();

    let col_t1 = db.collection_for_tenant("shared_name", tenant_1).await?;
    let col_t2 = db.collection_for_tenant("shared_name", tenant_2).await?;

    assert!(
        !std::sync::Arc::ptr_eq(&col_t1, &col_t2),
        "Handles for different tenants must NOT be identical"
    );

    let vec1 = vec![0.1f32; 768];
    col_t1
        .insert(
            "doc1",
            &vec1,
            Some(serde_json::json!({"text": "tenant 1 data"})),
        )
        .await?;

    // Tenant 2 must not see tenant 1's doc1
    let doc_t2 = col_t2.get("doc1").await?;
    assert!(doc_t2.is_none(), "Tenant 2 must not see Tenant 1 data");

    db.close().await?;
    Ok(())
}
