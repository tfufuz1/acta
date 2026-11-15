// Audit-Experiment (nicht Teil des Ziel-Repos): belegt V-01 und V-02.

use acta_erasure::types_local::{KeyBuf, RecordSealer, SealCtx};
use acta_erasure::vault::VaultSealer;
use acta_erasure::vault::SlotType;
use acta_erasure::vault::FileVault;
use parking_lot::RwLock;
use std::sync::Arc;

fn seal(s: &VaultSealer, key: &KeyBuf, p: &[u8]) -> Vec<u8> {
    let ctx = SealCtx::new(1);
    futures_lite_block(s.seal(&ctx, key, p)).unwrap().payload().to_vec()
}
fn open(s: &VaultSealer, key: &KeyBuf, env: &[u8]) -> Result<Vec<u8>, String> {
    futures_lite_block(s.open(key, env))
}
// minimaler Block-on ohne unsafe
fn futures_lite_block<T>(f: impl std::future::Future<Output = T>) -> T {
    use std::sync::Arc;
    use std::task::{Context, Poll, Wake, Waker};
    struct N;
    impl Wake for N { fn wake(self: Arc<Self>) {} }
    let w = Waker::from(Arc::new(N));
    let mut cx = Context::from_waker(&w);
    let mut f = std::pin::pin!(f);
    loop { if let Poll::Ready(v) = f.as_mut().poll(&mut cx) { return v; } }
}

#[test]
fn v01_destroyed_slot_yields_garbage_not_erased() {
    let vault = Arc::new(RwLock::new(FileVault::new_in_memory(1024)));
    let k_unit = [7u8; 32];
    let key = KeyBuf([0x11; 16]);
    let tag = [0x11u8; 16];
    {
        let mut v = vault.write();
        v.put_slot(SlotType::Tenant, tag, [1; 32]).unwrap();
        v.put_slot(SlotType::Collection, tag, [2; 32]).unwrap();
        v.put_slot(SlotType::Document, tag, [3; 32]).unwrap();
    }
    let s = VaultSealer::with_vault(vault.clone(), k_unit);
    let plain = b"Max Mustermann, Gehalt 84.000 EUR".to_vec();
    let env = seal(&s, &key, &plain);
    assert_eq!(open(&s, &key, &env).unwrap(), plain, "Baseline");

    // Schlüssel vernichten (D-Slot), neue Instanz ohne Cache
    vault.write().destroy_slot(SlotType::Document, tag).unwrap();
    let s2 = VaultSealer::with_vault(vault.clone(), k_unit);
    let res = open(&s2, &key, &env);
    println!("V-01 after destroy: {:?}", res.as_ref().map(|v| v.len()));
    // Erwartung laut SPEC S1/S7: Err(Erased). Ist: Ok(...) mit Nicht-Klartext
    assert!(res.is_ok(), "Ist-Zustand: Öffnen liefert Bytes statt Fehler");
    assert_ne!(res.unwrap(), plain, "Ist-Zustand: Bytes sind Müll, kein Klartext");
}

#[test]
fn v01_default_sealer_is_keyless() {
    // VaultSealer::new(): k_unit = [0x55;32], kein Tresor. Angreifer kennt nur key.
    let s = VaultSealer::new();
    let key = KeyBuf([0x22; 16]);
    let plain = b"vertraulich vertraulich vertraulich".to_vec();
    let env = seal(&s, &key, &plain);
    // Angreifer: DEK = keyed_hash([0x55;32], key); Strom = keyed_hash(DEK, label)
    let dek = blake3::keyed_hash(&[0x55u8; 32], &key.0);
    let stream = blake3::keyed_hash(dek.as_bytes(), b"ctx/envelope/stream");
    let rec: Vec<u8> = env.iter().enumerate()
        .map(|(i, b)| b ^ stream.as_bytes()[i % 32]).collect();
    assert_eq!(rec, plain, "Angreifer ohne Schlüssel liest Klartext");
}

#[test]
fn v02_xor_two_records_same_key_leaks_xor_of_plaintexts() {
    let s = VaultSealer::new();
    let key = KeyBuf([0x33; 16]);
    let a = b"AAAAAAAAAAAAAAAA".to_vec();
    let b = b"BBBBBBBBBBBBBBBB".to_vec();
    let ea = seal(&s, &key, &a);
    let eb = seal(&s, &key, &b);
    let x: Vec<u8> = ea.iter().zip(&eb).map(|(p, q)| p ^ q).collect();
    let px: Vec<u8> = a.iter().zip(&b).map(|(p, q)| p ^ q).collect();
    assert_eq!(x, px, "XOR der Chiffrate = XOR der Klartexte");
}
