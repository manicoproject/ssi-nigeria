use crate::identity::IdentityKeypair;
use crate::identity::did::DidKey;
use crate::types::credential::{CredentialSubject, Proof, VerifiableCredential};
use crate::types::selective::{SelectiveCredential, SubClaim};
use chrono::Utc;
use serde::Serialize;
use serde_json::json;

/// An entity (like NIMC or a Bank) that has the authority to issue credentials.
pub struct Issuer {
    /// The institution's cryptographic keypair.
    pub keypair: IdentityKeypair,
    /// The institution's public DID.
    pub did: String,
}

#[allow(dead_code)]
impl Issuer {
    /// Creates a new Issuer from an existing keypair.
    pub fn new(keypair: IdentityKeypair) -> Self {
        let did = DidKey::from_keypair(&keypair);
        Self { keypair, did }
    }

    /// Issues a new Verifiable Credential.
    ///
    /// This function takes a claim and a holder's DID, and produces a signed
    /// VerifiableCredential struct.
    pub fn issue<T: Serialize>(
        &self,
        holder_did: String,
        claims: T,
        credential_type: &str,
    ) -> anyhow::Result<VerifiableCredential<T>> {
        let mut vc = VerifiableCredential {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            r#type: vec![
                "VerifiableCredential".to_string(),
                credential_type.to_string(),
            ],
            issuer: self.did.clone(),
            issuance_date: Utc::now().to_rfc3339(),
            credential_subject: CredentialSubject {
                id: holder_did,
                claims,
            },
            proof: None,
        };

        // 1. Serialize the VC (without proof) to JSON for signing
        let json_data = serde_json::to_vec(&vc)?;

        // 2. Sign the data using the issuer's secret key
        let signature = self.keypair.sign(&json_data);

        // 3. Attach the proof
        vc.proof = Some(Proof {
            r#type: "Ed25519Signature2020".to_string(),
            verification_method: format!("{}#key-1", self.did),
            proof_value: multibase::encode(multibase::Base::Base58Btc, signature),
        });

        Ok(vc)
    }

    /// Issues a SelectiveDisclosureCredential where each field is signed independently.
    pub fn issue_selective(
        &self,
        holder_did: String,
        fields: Vec<(&str, serde_json::Value)>,
    ) -> anyhow::Result<SelectiveCredential> {
        let mut sub_claims = Vec::new();

        for (field_name, value) in fields {
            // 1. Create the data to be signed for this specific sub-claim
            let sub_claim_data = json!({
                "holder": holder_did,
                "field": field_name,
                "value": value
            });
            let json_bytes = serde_json::to_vec(&sub_claim_data)?;

            // 2. Sign it
            let signature = self.keypair.sign(&json_bytes);

            // 3. Create the SubClaim object
            sub_claims.push(SubClaim {
                field: field_name.to_string(),
                value,
                proof: Proof {
                    r#type: "Ed25519Signature2020".to_string(),
                    verification_method: format!("{}#key-1", self.did),
                    proof_value: multibase::encode(multibase::Base::Base58Btc, signature),
                },
            });
        }

        Ok(SelectiveCredential {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            r#type: vec!["SelectiveCredential".to_string()],
            issuer: self.did.clone(),
            holder: holder_did,
            sub_claims,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::credential::NigerianCitizenCredential;

    #[test]
    fn test_credential_issuance() {
        let nimc_key = IdentityKeypair::generate();
        let nimc = Issuer::new(nimc_key);

        let citizen_did = "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string();
        let claims = NigerianCitizenCredential {
            name: "Amaka Okafor".to_string(),
            state_of_origin: "Anambra".to_string(),
        };

        let vc_result = nimc.issue(citizen_did, claims, "NigerianCitizenCredential");

        assert!(vc_result.is_ok(), "Issuance must succeed");
        let vc = vc_result.unwrap();

        assert_eq!(vc.issuer, nimc.did);
        assert!(vc.proof.is_some());
        assert_eq!(vc.proof.unwrap().r#type, "Ed25519Signature2020");
    }
}
