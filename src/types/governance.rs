use crate::types::credential::Proof;
use serde::{Deserialize, Serialize};

/// A legal authorization signed by a Judge.
///
/// Implements: trust-and-authority.md#Section-4-Credential-Chain
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WarrantCredential {
    /// The DID of the Judge who issued the warrant.
    pub judge_did: String,
    /// The DID of the Officer authorized to execute it.
    pub authorized_officer_did: String,
    /// The DID of the Citizen who is the subject of the warrant.
    pub target_citizen_did: String,
    /// What specific data is allowed to be accessed (e.g., "FullProfile").
    pub access_scope: String,
    /// Expiration date (Warrants must be time-limited).
    pub expires_at: String,
    /// The cryptographic signature of the Court.
    pub proof: Option<Proof>,
}
