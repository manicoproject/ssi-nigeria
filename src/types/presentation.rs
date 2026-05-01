use crate::types::credential::{Proof, VerifiableCredential};
use serde::{Deserialize, Serialize};

/// A standard W3C-compliant Verifiable Presentation wrapper.
///
/// This structure allows a holder to package one or more credentials
/// and sign the package to prove ownership.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifiablePresentation<T> {
    /// The context URL for the W3C VP standard.
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    /// The type of presentation.
    pub r#type: Vec<String>,
    /// The list of credentials being presented.
    pub verifiable_credential: Vec<VerifiableCredential<T>>,
    /// The DID of the holder presenting the credentials.
    pub holder: String,
    /// The cryptographic proof (signature) proving the holder signed this package.
    pub proof: Option<Proof>,
}
