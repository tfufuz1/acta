// Compile-Fail Test: Fehlender TenantContext verhindert Zugriff auf Collection (SPEC 7.1, D-07, T-API-1)
use acta::Acta;

#[tokio::main]
async fn main() {
    // Auf Acta gibt es keine mandantenlose Methode collection(...)
    // Man muss erst .tenant(ctx).await aufrufen!
    fn check_requires_tenant_ctx(db: &Acta) {
        let _ = db.collection("default", Default::default());
    }
}
