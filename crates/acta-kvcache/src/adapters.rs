//! # Produktive Port-Adapter für den Acta KV-Cache
//!
//! ANCHOR[acta-kvcache:ADAPT-001]
//!
//! Dieses Modul stellt konkrete Implementierungen der Ring-0/Ring-1 Port-Traits bereit:
//! - [`StorageEngineKvBridge`]: Verbindet Tier-2 Persistence über den generischen [`StorageEngine`]-Port.
//! - [`EmbeddingProviderSemanticEmbedder`]: Adaptiert den [`EmbeddingProvider`]-Port für semantische Cache-Lookups.
//! - [`KvCachePluginManifest`]: Bietet ein validiertes Manifest mit Capabilities, Limits und Versionierung.

use std::sync::Arc;

use bytes::Bytes;
use acta_ports::embedding::EmbeddingProvider;
use acta_ports::kv_bridge_port::KvBridgeStorage;
use acta_ports::license::FeatureRing;
use acta_ports::plugin::{PluginCapability, PluginError, PluginManifest};
use acta_ports::storage::StorageEngine;
use acta_ports::BoxFuture;
use acta_types::{ActaError, Result as ActaResult, TxId};

#[cfg(feature = "content-addressed-kv-cache")]
use crate::radix::SemanticEmbedder;

#[cfg(feature = "content-addressed-kv-cache")]
fn block_on_simple<F: std::future::Future>(fut: F) -> F::Output {
    let waker = std::task::Waker::noop();
    let mut cx = std::task::Context::from_waker(waker);
    let mut pinned = std::pin::pin!(fut);
    loop {
        match pinned.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(val) => return val,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

// ANCHOR[acta-kvcache:ADAPT-002]
/// Produktiver Tier-2 Spill-over Adapter über das generische [`StorageEngine`]-Interface.
///
/// Bietet mandantengetrennte Persistierung von KV-Segmenten im Storage-Layer.
pub struct StorageEngineKvBridge {
    engine: Arc<dyn StorageEngine>,
}

impl StorageEngineKvBridge {
    /// Erstellt eine neue `StorageEngineKvBridge` um eine gegebene [`StorageEngine`].
    pub fn new(engine: Arc<dyn StorageEngine>) -> Self {
        Self { engine }
    }
}

impl KvBridgeStorage for StorageEngineKvBridge {
    /// Liest Roh-Bytes zu einem Schlüssel asynchron aus der Storage-Engine.
    fn get<'a>(&'a self, key: &'a [u8]) -> BoxFuture<'a, ActaResult<Option<Bytes>>> {
        Box::pin(async move { self.engine.get(key).await })
    }

    /// Schreibt ein KV-Paar im Kontext einer Transaktion (`tx_id`) in die Storage-Engine.
    fn put<'a>(
        &'a self,
        tx_id: TxId,
        key: &'a [u8],
        value: &'a [u8],
    ) -> BoxFuture<'a, ActaResult<()>> {
        Box::pin(async move { self.engine.put(tx_id, key, value).await })
    }

    /// Bestätigt eine Transaktion (`commit`) dauerhaft in der Storage-Engine.
    fn commit<'a>(&'a self, tx_id: TxId) -> BoxFuture<'a, ActaResult<()>> {
        Box::pin(async move { self.engine.commit(tx_id).await })
    }
}

// ANCHOR[acta-kvcache:ADAPT-003]
/// Adaptiert einen generischen [`EmbeddingProvider`] für den `SemanticEmbedder`-Trait im KV-Cache.
pub struct EmbeddingProviderSemanticEmbedder {
    provider: Arc<dyn EmbeddingProvider>,
}

impl EmbeddingProviderSemanticEmbedder {
    /// Erstellt einen neuen `EmbeddingProviderSemanticEmbedder`.
    pub fn new(provider: Arc<dyn EmbeddingProvider>) -> Self {
        Self { provider }
    }

    /// Liefert eine Referenz auf den zugrundeliegenden [`EmbeddingProvider`].
    pub fn provider(&self) -> &Arc<dyn EmbeddingProvider> {
        &self.provider
    }
}

#[cfg(feature = "content-addressed-kv-cache")]
impl SemanticEmbedder for EmbeddingProviderSemanticEmbedder {
    /// Berechnet das Text-Embedding für eine Sequenz von Token-IDs.
    ///
    /// # Ablauf
    /// 1. Validieren, dass Tokens nicht leer sind.
    /// 2. Tokens als String repräsentieren.
    /// 3. In separatem Thread asynchron über den Provider berechnen und auf Ergebnis warten.
    fn embed(&self, tokens: &[u32]) -> ActaResult<Vec<f32>> {
        if tokens.is_empty() {
            return Err(ActaError::InvalidInput(
                "Cannot compute embedding for empty token sequence".to_string(),
            ));
        }

        let token_text = tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        let provider = Arc::clone(&self.provider);
        let handle = std::thread::spawn(move || {
            let future = provider.embed(&token_text);
            block_on_simple(future)
        });

        // NOTE: Bei Aktivierung von `content-addressed-kv-cache` schlägt `handle.join()...?`
        // typmäßig fehl, da `EmbeddingError` nicht in `ActaError` konvertiert wird.
        // Gemäß Vorgabe wird der Code nicht modifiziert.
        handle.join().map_err(|_| {
            ActaError::Internal("Thread panic during embedding computation".to_string())
        })?
    }
}

// ANCHOR[acta-kvcache:ADAPT-004]
/// Validierte Konfiguration für das KV-Cache-Plugin-Manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KvCachePluginConfig {
    /// Plugin Versions-Tupel (Major, Minor, Patch).
    pub version: (u32, u32, u32),
    /// Maximales Speicherlimit im RAM in Bytes für KV-Cache-Allokationen.
    pub max_memory_bytes: u64,
    /// Mindestens erforderlicher Feature-Ring der Lizenz.
    pub feature_ring_required: FeatureRing,
}

impl Default for KvCachePluginConfig {
    fn default() -> Self {
        Self {
            version: (1, 0, 0),
            max_memory_bytes: 512 << 20, // 512 MiB
            feature_ring_required: FeatureRing::Fast,
        }
    }
}

impl KvCachePluginConfig {
    /// Validiert die Konfigurationsparameter (mindestens 1 MiB RAM erforderlich).
    pub fn validate(&self) -> ActaResult<()> {
        if self.max_memory_bytes < (1 << 20) {
            return Err(ActaError::InvalidInput(
                "max_memory_bytes must be at least 1 MiB".to_string(),
            ));
        }
        Ok(())
    }
}

// ANCHOR[acta-kvcache:ADAPT-005]
/// Konkrete, validierte [`PluginManifest`]-Implementierung für `acta-kvcache`.
#[derive(Debug, Clone)]
pub struct KvCachePluginManifest {
    config: KvCachePluginConfig,
}

impl KvCachePluginManifest {
    /// Erstellt ein neues, validiertes `KvCachePluginManifest`.
    pub fn new(config: KvCachePluginConfig) -> ActaResult<Self> {
        config.validate()?;
        Ok(Self { config })
    }
}

impl PluginManifest for KvCachePluginManifest {
    fn name(&self) -> &str {
        "acta-kvcache"
    }

    fn requires(&self) -> &[&str] {
        &[]
    }

    fn conflicts(&self) -> &[&str] {
        &[]
    }

    fn feature_ring_required(&self) -> FeatureRing {
        self.config.feature_ring_required
    }

    fn capability(&self) -> PluginCapability {
        PluginCapability {
            name: "acta-kvcache",
            version: self.config.version,
            ring: 1,
            feature_ring_required: self.config.feature_ring_required,
        }
    }

    fn activate(&self) -> Result<(), PluginError> {
        tracing::info!(
            version = ?self.config.version,
            max_memory_bytes = self.config.max_memory_bytes,
            "Activated acta-kvcache plugin manifest"
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Prüft Validierungsregeln des Plugin-Manifests (z. B. Mindestspeichergröße).
    #[test]
    fn test_kv_cache_plugin_manifest_validation() {
        let invalid_config = KvCachePluginConfig {
            version: (1, 0, 0),
            max_memory_bytes: 100, // < 1 MiB -> Invalid
            feature_ring_required: FeatureRing::Fast,
        };
        assert!(KvCachePluginManifest::new(invalid_config).is_err());

        let valid_config = KvCachePluginConfig::default();
        if let Ok(manifest) = KvCachePluginManifest::new(valid_config) {
            assert_eq!(manifest.name(), "acta-kvcache");
            assert_eq!(manifest.capability().ring, 1);
            assert!(manifest.activate().is_ok());
        }
    }
}
