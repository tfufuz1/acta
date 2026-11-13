# Fehlerinventar acta-store (S-37-A)
| StoreError-Variante | ActaError-Variante | Exakter Display-String |
|---|---|---|
| Storage(String) | ActaError::Storage(String) | Storage error: {0} |
| Io(std::io::Error) | ActaError::Io(std::io::Error) | I/O error: {0} |
| WalCorruption { offset: u64, reason: String } | ActaError::WalCorruption | WAL corruption detected at offset {offset}: {reason} |
| ChecksumMismatch { path: String, block_id: u64 } | ActaError::ChecksumMismatch | Checksum mismatch: file={path}, block={block_id} |
| WalTruncationDetected | ActaError::WalTruncationDetected | WAL truncation attack detected |
| CrossDeviceLink | ActaError::CrossDeviceLink | Cross-device link failed |
| Transaction(String) | ActaError::Transaction(String) | Transaction error: {0} |
| Conflict(String) | ActaError::Conflict(String) | Conflict: {0} |
| CommitTimeout { tx_id: u64 } | ActaError::CommitTimeout | Group commit timed out for transaction {tx_id} |
| Serialization(String) | ActaError::Serialization(String) | Serialization error: {0} |
| ParseError(String) | ActaError::ParseError(String) | Parse error: {0} |
| InvalidInput(String) | ActaError::InvalidInput(String) | Invalid input: {0} |
| NotFound(String) | ActaError::NotFound(String) | Not found: {0} |
| Internal(String) | ActaError::Internal(String) | Internal error: {0} |
| MemoryBudgetExceeded | ActaError::MemoryBudgetExceeded | Memory budget exceeded: {used_mb}MB / {limit_mb}MB |
| LimitExceeded | ActaError::LimitExceeded | Limit exceeded: {context} (limit: {limit}) |
| DurabilityConfig(String) | ActaError::DurabilityConfig | Durability configuration error: {0} |
| Sandbox(String) | ActaError::Sandbox(String) | Sandbox error: {0} |
