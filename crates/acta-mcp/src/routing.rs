use crate::config::RouterConfig;
use acta::Acta;
use acta_types::{ContextChunk, ContextWindow, ActaError, EntityId, TokenBudget};
use std::sync::Arc;

struct CollectionAdapter(Arc<acta::Collection>);

impl acta_router::ports_local::HybridSearchProvider for CollectionAdapter {
    fn search_hybrid<'a>(
        &'a self,
        query_text: &'a str,
        query_embedding: &'a [f32],
        top_k: usize,
    ) -> acta_ports::BoxFuture<'a, Result<Vec<ContextChunk>, ActaError>> {
        Box::pin(async move {
            let search_results = self
                .0
                .query()
                .text(query_text)
                .vector(query_embedding)
                .k(top_k)
                .execute()
                .await?;

            let mut chunks = Vec::with_capacity(search_results.len());
            for res in search_results {
                if let Ok(chunk) = ContextChunk::try_from(res) {
                    chunks.push(chunk);
                }
            }
            Ok(chunks)
        })
    }
}

struct CommunityAdapter;

impl acta_router::ports_local::CommunityResolver for CommunityAdapter {
    fn get_community<'a>(
        &'a self,
        _entity_id: EntityId,
    ) -> acta_ports::BoxFuture<'a, Result<Option<u64>, ActaError>> {
        Box::pin(async move { Ok(None) })
    }
}

struct ContextPreparerAdapter;

impl acta_router::ports_local::ContextPreparer for ContextPreparerAdapter {
    fn prepare_context(
        &self,
        chunks: Vec<ContextChunk>,
        budget: &TokenBudget,
        _relevance_threshold: f32,
    ) -> Result<ContextWindow, ActaError> {
        let total_tokens = chunks.iter().map(|c| c.token_count).sum();
        let limit = budget.limit;
        let truncated = total_tokens > limit;
        Ok(ContextWindow {
            chunks,
            total_tokens,
            truncated,
        })
    }
}

struct RouterDriftAdapter(Arc<acta_router::DefaultRouterEngine>);

impl acta_ports::DriftStatusProvider for RouterDriftAdapter {
    fn overall_drift_status(&self) -> String {
        self.0.overall_drift_status()
    }
}

/// Holds strong Arc references to routing and calibration components to maintain live Weak references in `Acta`.
pub struct RoutingHandle {
    pub router: Arc<acta_router::DefaultRouterEngine>,
    pub calibrator: Arc<parking_lot::Mutex<acta_rank::IsotonicCalibrator>>,
    pub _drift_adapter: Arc<dyn acta_ports::DriftStatusProvider>,
}

pub async fn setup_routing(
    db: &Arc<Acta>,
    config: &RouterConfig,
) -> Result<Option<Arc<RoutingHandle>>, ActaError> {
    if config.profiles.is_empty() {
        return Ok(None);
    }

    let default_col = db.collection("default").await?;

    let search_provider = Arc::new(CollectionAdapter(default_col.clone()));
    let community_resolver = Arc::new(CommunityAdapter);
    let context_preparer = Arc::new(ContextPreparerAdapter);

    let router = Arc::new(acta_router::RouterEngine::new(
        search_provider,
        community_resolver,
        context_preparer,
        config.profiles.clone(),
        config.calibration_store_path.clone(),
    ));

    let calibrator = Arc::new(parking_lot::Mutex::new(
        acta_rank::IsotonicCalibrator::with_defaults(),
    ));

    let drift_adapter: Arc<dyn acta_ports::DriftStatusProvider> =
        Arc::new(RouterDriftAdapter(router.clone()));
    let router_weak = Arc::downgrade(&drift_adapter);
    db.set_router(router_weak);
    db.set_calibrator(Arc::downgrade(&calibrator));

    Ok(Some(Arc::new(RoutingHandle {
        router,
        calibrator,
        _drift_adapter: drift_adapter,
    })))
}
