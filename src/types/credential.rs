use serde::{Deserialize, Serialize};

/// Represents a claim that a person is a Nigerian citizen.
///
/// Implements: credential-hierarchy.md#Section-1-Foundational-Credentials
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NigerianCitizenCredential {
    /// The citizen's full name.
    pub name: String,
    /// State of origin.
    pub state_of_origin: String,
}

/// A standard W3C-compliant Verifiable Credential wrapper.
///
/// This structure holds the claim, the issuer's DID, and the cryptographic proof.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifiableCredential<T> {
    /// The context URL for the W3C VC standard.
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    /// The type of credential.
    pub r#type: Vec<String>,
    /// The DID of the authority that issued this credential (e.g., NIMC).
    pub issuer: String,
    /// The date the credential was issued.
    pub issuance_date: String,
    /// The actual data being claimed (e.g., NigerianCitizenCredential).
    pub credential_subject: CredentialSubject<T>,
    /// The cryptographic signature proving the issuer signed this data.
    pub proof: Option<Proof>,
}

/// The subject of the credential, linked to the holder's DID.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CredentialSubject<T> {
    /// The DID of the person this credential belongs to.
    pub id: String,
    /// The claims about the subject.
    #[serde(flatten)]
    pub claims: T,
}

/// The cryptographic proof (signature) attached to a credential.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Proof {
    /// The type of signature (e.g., "Ed25519Signature2020").
    pub r#type: String,
    /// The verification method (issuer's DID + key identifier).
    pub verification_method: String,
    /// The actual signature value (usually base58 or hex).
    pub proof_value: String,
}
