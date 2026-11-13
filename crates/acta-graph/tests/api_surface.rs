//! Compile-time verification of all public re-exports from acta-graph.
#[allow(unused_imports)]
use acta_graph::*;

#[test]
fn test_api_surface_types() {
    let _ = CsrGraph::new();
}
