//! Keyspaces des Umschlags v1 (Spec 4.5). Der Wert steht als `u16` in der AAD (`SealCtx::keyspace`).

/// P-Datensatz.
pub const P_RECORD: u16 = 0x0001;
/// P-Dokumentkopf.
pub const P_DOC_HEADER: u16 = 0x0002;
/// Betroffenenverzeichnis.
pub const SUBJECT_DIR: u16 = 0x0003;
/// Handle-Register.
pub const HANDLE_REGISTER: u16 = 0x0004;
/// Vorgang.
pub const CASE: u16 = 0x0005;
/// Entscheidungsnutzlast.
pub const DECISION_PAYLOAD: u16 = 0x0006;
/// K-Pack-Block.
pub const K_PACK_BLOCK: u16 = 0x0010;
/// Entscheidungsprotokoll-Block.
pub const DECISION_LOG_BLOCK: u16 = 0x0011;
/// DK-Block.
pub const DK_BLOCK: u16 = 0x0020;
/// DP-Block.
pub const DP_BLOCK: u16 = 0x0021;
/// Journalnutzlast.
pub const JOURNAL_PAYLOAD: u16 = 0x0030;
/// Arbeitsdaten (`ScopeKey`).
pub const WORK_DATA: u16 = 0x0040;
/// Zugriffsprotokoll-Eintrag.
pub const ACCESS_LOG_ENTRY: u16 = 0x0050;
/// Ausgangskorb (`ScopeKey`).
pub const OUTBOX: u16 = 0x0060;
/// Idempotenztabelle.
pub const IDEMPOTENCY: u16 = 0x0070;

/// Alle in Spec 4.5 festgelegten Keyspaces.
pub const ALL: [u16; 15] = [
    P_RECORD,
    P_DOC_HEADER,
    SUBJECT_DIR,
    HANDLE_REGISTER,
    CASE,
    DECISION_PAYLOAD,
    K_PACK_BLOCK,
    DECISION_LOG_BLOCK,
    DK_BLOCK,
    DP_BLOCK,
    JOURNAL_PAYLOAD,
    WORK_DATA,
    ACCESS_LOG_ENTRY,
    OUTBOX,
    IDEMPOTENCY,
];

/// Ist `keyspace` in Spec 4.5 festgelegt?
#[must_use]
pub fn is_known(keyspace: u16) -> bool {
    ALL.contains(&keyspace)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_are_unique_and_known() {
        for (i, a) in ALL.iter().enumerate() {
            assert!(is_known(*a));
            for b in ALL.iter().skip(i + 1) {
                assert_ne!(a, b);
            }
        }
        assert!(!is_known(0));
        assert!(!is_known(0x0007));
    }
}
