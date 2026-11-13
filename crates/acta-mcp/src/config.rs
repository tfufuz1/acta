// =========================================================================
// SECTION: config
// =========================================================================
// ANCHOR[acta-mcp:CONFIG-001]
// FILE-CONTEXT
// ZWECK:       Embedding-Provider-Auflösung (fail-closed) und Startup-Probe
// INVARIANTEN: Kein Default-Provider; unbekannte Provider schlagen fehl; Mock nur in Test-/Dev-Builds
// HOTSPOTS:    create_embedding_provider(), verify_embedding_provider()
// SIEHE AUCH:  ADR-010, acta-ports/src/traits/mod.rs

use acta_ports::EmbeddingProvider;
use acta_types::ActaError;
use std::sync::{Arc, Once};

static EMBEDDING_PROVIDER_DEPRECATION_WARN_ONCE: Once = Once::new();

/// Liste der einkompilierten Provider-Kennungen. Ohne Backend-Crates nur `mock` (Test/Dev).
pub fn available_compiled_providers() -> Vec<&'static str> {
    let mut list = Vec::new();
    if cfg!(any(test, feature = "test-utils")) {
        list.push("mock");
    }
    list
}

fn no_provider_error() -> ActaError {
    let avail = available_compiled_providers();
    let avail_str = if avail.is_empty() {
        "keine / none".to_string()
    } else {
        avail.join(", ")
    };
    ActaError::InvalidInput(format!(
        "Kein Embedding-Provider konfiguriert / No embedding provider configured. \
         Bitte ACTA_EMBEDDING_PROVIDER setzen. \
         Einkompilierte Provider / Available compiled providers: [{avail_str}]."
    ))
}

fn unknown_provider_error(provider: &str) -> ActaError {
    let avail = available_compiled_providers();
    let avail_str = if avail.is_empty() {
        "keine / none".to_string()
    } else {
        avail.join(", ")
    };
    ActaError::InvalidInput(format!(
        "Unbekannter Embedding-Provider '{provider}' / Unknown embedding provider '{provider}'. \
         Verfügbare einkompilierte Provider / Available compiled providers: [{avail_str}]."
    ))
}

/// Startup embedding self-test (probe) verifying provider functionality.
pub fn verify_embedding_provider(
    provider: &dyn EmbeddingProvider,
    expected_dim: Option<usize>,
) -> Result<(), ActaError> {
    let probe_text = "acta startup probe";
    let probe_fut = provider.embed(probe_text);

    let vec = match tokio::runtime::Handle::try_current() {
        Ok(handle) => tokio::task::block_in_place(|| handle.block_on(probe_fut)),
        Err(_) => {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| {
                    ActaError::Internal(format!(
                        "Konnte Tokio-Laufzeit für Probe nicht erstellen: {e}"
                    ))
                })?;
            rt.block_on(probe_fut)
        }
    }
    .map_err(|e| {
        ActaError::Internal(format!(
            "Embedding-Selbsttest Probe fehlgeschlagen / Embedding probe failed: {e}"
        ))
    })?;

    // Probe check 1: Dimension > 0
    if vec.is_empty() {
        return Err(ActaError::Internal(
            "Embedding-Selbsttest fehlgeschlagen: Vektordimension ist 0 / Embedding self-test failed: vector dimension is 0".to_string(),
        ));
    }

    // Probe check 2: Vector is not completely zero
    let is_all_zero = vec.iter().all(|&x| x == 0f32);
    if is_all_zero {
        return Err(ActaError::Internal(
            "Embedding-Selbsttest fehlgeschlagen: Vektor ist komplett Null / Embedding self-test failed: vector is all zeros".to_string(),
        ));
    }

    // Probe check 3: Norm is plausible (finite, non-zero, within 1e-6..1e6)
    let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
    if !norm.is_finite() || norm < 1e-6 || norm > 1e6 {
        return Err(ActaError::Internal(format!(
            "Embedding-Selbsttest fehlgeschlagen: Unplausibler Normwert {norm} / Embedding self-test failed: implausible norm {norm}"
        )));
    }

    // Probe check 4: Dimension matches expected_dim if provided
    if let Some(exp) = expected_dim {
        if vec.len() != exp {
            return Err(ActaError::Internal(format!(
                "Embedding-Selbsttest fehlgeschlagen: Dimension {got} stimmt nicht mit erwarteter Indexdimension {exp} überein / Dimension mismatch: got {got}, expected {exp}",
                got = vec.len()
            )));
        }
    }

    Ok(())
}

/// Ergebnis der Provider-Einstellung-Auflösung.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ResolvedProviderSetting {
    pub value: String,
    pub used_unprefixed_fallback: bool,
}

/// Löst eine Provider-Einstellung ohne Prozess-Env-Zugriff auf.
pub(crate) fn resolve_provider_setting(
    prefixed: Option<&str>,
    unprefixed: Option<&str>,
) -> ResolvedProviderSetting {
    let clean_prefixed = prefixed.map(|s| s.trim()).filter(|s| !s.is_empty());
    let clean_unprefixed = unprefixed.map(|s| s.trim()).filter(|s| !s.is_empty());

    match (clean_prefixed, clean_unprefixed) {
        (Some(p), _) => ResolvedProviderSetting {
            value: p.to_string(),
            used_unprefixed_fallback: false,
        },
        (None, Some(u)) => ResolvedProviderSetting {
            value: u.to_string(),
            used_unprefixed_fallback: true,
        },
        (None, None) => ResolvedProviderSetting {
            value: String::new(),
            used_unprefixed_fallback: false,
        },
    }
}

/// Embedding-Provider-Konfiguration.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct EmbeddingConfig {
    /// Provider-Kennung. Leer bedeutet: nicht konfiguriert.
    pub provider: String,
    /// Optionale erwartete Dimension zur Index-Alignment-Prüfung.
    pub expected_dim: Option<usize>,
}

impl EmbeddingConfig {
    /// Lädt die Konfiguration aus Umgebungsvariablen.
    pub fn from_env() -> Self {
        let prefixed = std::env::var("ACTA_EMBEDDING_PROVIDER").ok();
        let unprefixed = std::env::var("EMBEDDING_PROVIDER").ok();
        let resolved = resolve_provider_setting(prefixed.as_deref(), unprefixed.as_deref());

        if resolved.used_unprefixed_fallback {
            EMBEDDING_PROVIDER_DEPRECATION_WARN_ONCE.call_once(|| {
                tracing::warn!(
                    "Ungeprefixte Variable EMBEDDING_PROVIDER ist deprecated, bitte ACTA_EMBEDDING_PROVIDER verwenden."
                );
            });
        }

        Self {
            provider: resolved.value,
            expected_dim: None,
        }
    }

    pub fn with_expected_dim(mut self, dim: usize) -> Self {
        self.expected_dim = Some(dim);
        self
    }

    pub fn provider_opt(&self) -> Option<&str> {
        let trimmed = self.provider.trim();
        if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("none") {
            None
        } else {
            Some(trimmed)
        }
    }

    /// Instanziiert den konfigurierten `EmbeddingProvider` und führt die Startup-Probe aus.
    pub fn build_provider(&self) -> Result<Arc<dyn EmbeddingProvider>, ActaError> {
        let provider = create_embedding_provider(self.provider_opt())?;
        verify_embedding_provider(provider.as_ref(), self.expected_dim)?;
        Ok(provider)
    }
}

/// Erzeugt einen `EmbeddingProvider` anhand der Kennung. Unbekannte oder nicht einkompilierte Provider schlagen fehl.
pub fn create_embedding_provider(
    provider_type: Option<&str>,
) -> Result<Arc<dyn EmbeddingProvider>, ActaError> {
    let provider = match provider_type {
        Some(p) if !p.trim().is_empty() => p.trim().to_lowercase(),
        _ => return Err(no_provider_error()),
    };

    match provider.as_str() {
        #[cfg(any(test, feature = "test-utils"))]
        "mock" => Ok(Arc::new(MockEmbedder::new(768))),
        other => Err(unknown_provider_error(other)),
    }
}

/// Fallback Mock Embedder (nur Test- und Dev-Builds).
#[cfg(any(test, feature = "test-utils"))]
#[derive(Debug, Clone)]
pub struct MockEmbedder {
    dimension: usize,
    all_zero: bool,
}

#[cfg(any(test, feature = "test-utils"))]
impl MockEmbedder {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            all_zero: false,
        }
    }

    pub fn with_zero_vectors(dimension: usize) -> Self {
        Self {
            dimension,
            all_zero: true,
        }
    }
}

#[cfg(any(test, feature = "test-utils"))]
impl std::fmt::Display for MockEmbedder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MockEmbedder(dim={}, TEST_ONLY=true)", self.dimension)
    }
}

#[cfg(any(test, feature = "test-utils"))]
impl EmbeddingProvider for MockEmbedder {
    fn provider_name(&self) -> &str {
        "mock"
    }

    fn embed<'a>(
        &'a self,
        _text: &'a str,
    ) -> acta_ports::BoxFuture<'a, Result<Vec<f32>, acta_ports::EmbeddingError>> {
        tracing::warn!("MockEmbedder embed invoked (TEST/DEV ONLY - NOT FOR PRODUCTION)");
        let dim = self.dimension;
        let all_zero = self.all_zero;
        Box::pin(async move {
            if all_zero {
                Ok(vec![0f32; dim])
            } else {
                let val = 1.0f32 / (dim as f32).sqrt();
                Ok(vec![val; dim])
            }
        })
    }

    fn embedding_dim(&self) -> usize {
        self.dimension
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_provider_setting_prefixed_wins() {
        let res = resolve_provider_setting(Some("mock"), Some("other"));
        assert_eq!(res.value, "mock");
        assert!(!res.used_unprefixed_fallback);
    }

    #[test]
    fn test_resolve_provider_setting_unprefixed_fallback() {
        let res = resolve_provider_setting(None, Some("other"));
        assert_eq!(res.value, "other");
        assert!(res.used_unprefixed_fallback);
    }

    #[test]
    fn test_resolve_provider_setting_default_empty() {
        let res = resolve_provider_setting(None, None);
        assert!(res.value.is_empty());
        assert!(!res.used_unprefixed_fallback);
    }

    #[test]
    fn test_embedding_config_defaults_unconfigured() {
        let config = EmbeddingConfig::default();
        assert!(config.provider.is_empty());
        assert!(config.provider_opt().is_none());
        assert!(config.expected_dim.is_none());
    }

    #[test]
    fn test_create_embedding_provider_unknown_error_lists_available() {
        let res = create_embedding_provider(Some("ollama"));
        let msg = res.err().map(|e| e.to_string()).unwrap_or_default();
        assert!(msg.contains("Unbekannter Embedding-Provider 'ollama'"));
        assert!(msg.contains("mock"));
    }

    #[test]
    fn test_create_embedding_provider_no_provider_error() {
        let res = create_embedding_provider(None);
        let msg = res.err().map(|e| e.to_string()).unwrap_or_default();
        assert!(msg.contains("Kein Embedding-Provider konfiguriert"));
    }

    #[test]
    fn test_mock_embedder_available_in_test() {
        let provider = create_embedding_provider(Some("mock")).unwrap();
        assert_eq!(provider.provider_name(), "mock");
        assert_eq!(provider.embedding_dim(), 768);
    }

    #[test]
    fn test_mock_embedder_display() {
        let mock = MockEmbedder::new(768);
        let display_str = format!("{mock}");
        assert!(display_str.contains("dim=768"));
        assert!(display_str.contains("TEST_ONLY=true"));
    }

    #[test]
    fn test_verify_embedding_provider_zero_vector_fails() {
        let zero_mock = MockEmbedder::with_zero_vectors(768);
        let err = verify_embedding_provider(&zero_mock, None).err().map(|e| e.to_string());
        assert!(err.unwrap_or_default().contains("Vektor ist komplett Null"));
    }

    #[test]
    fn test_verify_embedding_provider_dimension_mismatch_fails() {
        let mock = MockEmbedder::new(768);
        let err = verify_embedding_provider(&mock, Some(1536)).err().map(|e| e.to_string());
        assert!(err
            .unwrap_or_default()
            .contains("Dimension 768 stimmt nicht mit erwarteter Indexdimension 1536 überein"));
    }

    #[test]
    fn test_verify_embedding_provider_success() {
        let mock = MockEmbedder::new(768);
        assert!(verify_embedding_provider(&mock, Some(768)).is_ok());
    }

    #[test]
    fn test_embedding_config_with_expected_dim() {
        let config = EmbeddingConfig::default().with_expected_dim(768);
        assert_eq!(config.expected_dim, Some(768));
    }

    #[test]
    fn test_available_compiled_providers_contains_mock_in_test() {
        assert!(available_compiled_providers().contains(&"mock"));
    }

    #[test]
    fn test_build_provider_unconfigured_error() {
        let err = EmbeddingConfig::default().build_provider().err().map(|e| e.to_string());
        assert!(err.unwrap_or_default().contains("Kein Embedding-Provider konfiguriert"));
    }

    #[test]
    fn test_build_provider_mock_success() {
        let config = EmbeddingConfig {
            provider: "mock".to_string(),
            expected_dim: Some(768),
        };
        let provider = config.build_provider().unwrap();
        assert_eq!(provider.embedding_dim(), 768);
    }
}
