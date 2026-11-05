use acta_db::{Acta, ActaConfig};

#[tokio::test]
async fn test_open_dimension_mismatch_fails() {
    let dir = tempfile::tempdir().unwrap();
    let config_768 = ActaConfig {
        dimension: 768,
        ..Default::default()
    };
    let _db = Acta::open_with_config(dir.path(), config_768)
        .await
        .unwrap();

    // Zweites Öffnen mit falscher Dimension muss früh fehlschlagen
    let config_1536 = ActaConfig {
        dimension: 1536,
        ..Default::default()
    };
    let result = Acta::open_with_config(dir.path(), config_1536).await;
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(err.to_string().contains("Dimension mismatch"));
}

#[tokio::test]
async fn test_open_dimension_zero_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let config = ActaConfig {
        dimension: 0,
        ..Default::default()
    };
    let result = Acta::open_with_config(dir.path(), config).await;
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(err.to_string().contains("dimension must be > 0"));
}

#[tokio::test]
async fn test_open_dimension_too_large_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let config = ActaConfig {
        dimension: 100_000,
        ..Default::default()
    };
    let result = Acta::open_with_config(dir.path(), config).await;
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert!(err.to_string().contains("dimension exceeds maximum"));
}
