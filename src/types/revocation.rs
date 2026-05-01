use serde::{Deserialize, Serialize};

/// A unique identifier for a credential — derived by hashing its content.
///
/// This is a one-way commitment: you can check if a credential is revoked
/// without revealing whose credential it is.
#[allow(dead_code)]
pub type CredentialId = [u8; 32];

/// A signed list of revoked credential IDs published by an Issuer.
///
/// The registry contains NO personal data — only opaque content hashes.
/// Verifiers can cache and check this list entirely offline.
///
/// Implements: system-flaws.md#Section-4-Revocation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RevocationRegistry {
    /// The DID of the issuer who owns this registry.
    pub issuer_did: String,
    /// The list of revoked credential IDs (SHA-256 hashes).
    pub revoked_ids: Vec<Vec<u8>>,
    /// ISO8601 timestamp of the last update.
    pub updated_at: String,
    /// Signature over the registry contents.
    pub signature: Option<Vec<u8>>,
}

impl RevocationRegistry {
    /// Creates an empty registry for an issuer.
    pub fn new(issuer_did: String, updated_at: String) -> Self {
        Self {
            issuer_did,
            revoked_ids: Vec::new(),
            updated_at,
            signature: None,
        }
    }
}
