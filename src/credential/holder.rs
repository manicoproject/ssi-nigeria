use crate::identity::IdentityKeypair;
use crate::identity::did::DidKey;
use crate::types::credential::{Proof, VerifiableCredential};
use crate::types::presentation::VerifiablePresentation;
use serde::Serialize;

/// Represents the citizen (Holder) who controls their own keys and credentials.
pub struct Holder {
    /// The citizen's cryptographic keypair (stored on their device/card).
    pub keypair: IdentityKeypair,
    /// The citizen's public DID.
    pub did: String,
}

#[allow(dead_code)]
impl Holder {
    /// Creates a new Holder from an existing keypair.
    pub fn new(keypair: IdentityKeypair) -> Self {
        let did = DidKey::from_keypair(&keypair);
        Self { keypair, did }
    }

    /// Wraps one or more credentials into a signed Verifiable Presentation.
    ///
    /// This proves that the holder of the credentials is the same person who
    /// controls the private key associated with the DID in the credentials.
    pub fn create_presentation<T: Serialize + Clone>(
        &self,
        credentials: Vec<VerifiableCredential<T>>,
    ) -> anyhow::Result<VerifiablePresentation<T>> {
        let mut vp = VerifiablePresentation {
            context: vec!["https://www.w3.org/2018/credentials/v1".to_string()],
            r#type: vec!["VerifiablePresentation".to_string()],
            verifiable_credential: credentials,
            holder: self.did.clone(),
            proof: None,
        };

        // 1. Serialize the VP (without proof) to JSON for signing
        let json_data = serde_json::to_vec(&vp)?;

        // 2. Sign the data using the holder's secret key
        let signature = self.keypair.sign(&json_data);

        // 3. Attach the proof
        vp.proof = Some(Proof {
            r#type: "Ed25519Signature2020".to_string(),
            verification_method: format!("{}#key-1", self.did),
            proof_value: multibase::encode(multibase::Base::Base58Btc, signature),
        });

        Ok(vp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::credential::NigerianCitizenCredential;

    #[test]
    fn test_presentation_creation() {
        let holder_key = IdentityKeypair::generate();
        let holder = Holder::new(holder_key);

        let vc = VerifiableCredential {
            context: vec![],
            r#type: vec![],
            issuer: "did:key:nimc".to_string(),
            issuance_date: "2025-01-01".to_string(),
            credential_subject: crate::types::credential::CredentialSubject {
                id: holder.did.clone(),
                claims: NigerianCitizenCredential {
                    name: "Amaka".to_string(),
                    state_of_origin: "Anambra".to_string(),
                },
            },
            proof: None,
        };

        let vp_result = holder.create_presentation(vec![vc]);
        assert!(vp_result.is_ok());
        let vp = vp_result.unwrap();
        assert_eq!(vp.holder, holder.did);
        assert!(vp.proof.is_some());
    }
}
