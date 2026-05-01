use crate::identity::IdentityKeypair;
use crate::identity::did::DidKey;
use crate::types::credential::Proof;
use crate::types::governance::WarrantCredential;
use chrono::{Duration, Utc};

/// Represents a Judicial Authority (Court) that can authorize law enforcement.
#[allow(dead_code)]
pub struct Court {
    /// The Court's cryptographic keypair.
    pub keypair: IdentityKeypair,
    /// The Court's public DID.
    pub did: String,
}

#[allow(dead_code)]
impl Court {
    /// Creates a new Judicial Authority.
    pub fn new(keypair: IdentityKeypair) -> Self {
        let did = DidKey::from_keypair(&keypair);
        Self { keypair, did }
    }

    /// Issues a Warrant for a specific officer to access a specific citizen's data.
    pub fn issue_warrant(
        &self,
        officer_did: String,
        citizen_did: String,
        scope: &str,
    ) -> anyhow::Result<WarrantCredential> {
        let expires_at = (Utc::now() + Duration::hours(24)).to_rfc3339();

        let mut warrant = WarrantCredential {
            judge_did: self.did.clone(),
            authorized_officer_did: officer_did,
            target_citizen_did: citizen_did,
            access_scope: scope.to_string(),
            expires_at,
            proof: None,
        };

        // Sign the warrant
        let json_data = serde_json::to_vec(&warrant)?;
        let signature = self.keypair.sign(&json_bytes_clean(&json_data)?);

        warrant.proof = Some(Proof {
            r#type: "Ed25519Signature2020".to_string(),
            verification_method: format!("{}#key-1", self.did),
            proof_value: multibase::encode(multibase::Base::Base58Btc, signature),
        });

        Ok(warrant)
    }
}

/// Helper to ensure we sign clean JSON.
#[allow(dead_code)]
fn json_bytes_clean(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let v: serde_json::Value = serde_json::from_slice(data)?;
    Ok(serde_json::to_vec(&v)?)
}
