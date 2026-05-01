use crate::types::credential::Proof;
use serde::{Deserialize, Serialize};

/// A claim that can be independently disclosed.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubClaim {
    /// The name of the field (e.g., "over_18").
    pub field: String,
    /// The value of the field (as a JSON string or value).
    pub value: serde_json::Value,
    /// The cryptographic proof specifically for this sub-claim.
    pub proof: Proof,
}

/// A credential that supports selective disclosure of its fields.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectiveCredential {
    /// The context URL.
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    /// The type.
    pub r#type: Vec<String>,
    /// The DID of the issuer.
    pub issuer: String,
    /// The DID of the holder.
    pub holder: String,
    /// The list of signed sub-claims.
    pub sub_claims: Vec<SubClaim>,
}
