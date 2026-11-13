// FILE-CONTEXT
// ZWECK:       Egress Security Gateway & Classifier Enforcement for Cloud MCP Queries (Re-export from acta-privacy)

pub use acta_privacy::egress_gateway::*;
pub use acta_privacy::egress_vault::{EgressClassification, EgressClassifier};

use crate::prompt_injection::PromptInjectionGuard;
use crate::protocol::McpError;
use acta_privacy::EgressError;

impl From<EgressError> for McpError {
    fn from(err: EgressError) -> Self {
        match err {
            EgressError::InvalidParams(msg) => McpError::invalid_params(msg),
            EgressError::PolicyViolation(msg) => McpError::invalid_params(msg),
            EgressError::InternalError(msg) => McpError::internal_error(msg),
            EgressError::SurrogateCollision { ref surrogate, .. } => {
                McpError::internal_error(format!("Surrogate collision for {surrogate}"))
            }
        }
    }
}

impl acta_privacy::InjectionDetector for PromptInjectionGuard {
    fn detect(&self, text: &str) -> Option<String> {
        self.detect(text)
    }
}
