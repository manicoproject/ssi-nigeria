use crate::types::credential::VerifiableCredential;
use crate::types::presentation::VerifiablePresentation;
use crate::types::selective::SubClaim;
use anyhow::{Result, anyhow};
use ed25519_dalek::{Signature, VerifyingKey};
use serde::Serialize;
use serde_json::json;

/// An entity that verifies the authenticity of credentials.
pub struct Verifier;

#[allow(dead_code)]
impl Verifier {
    /// Verifies a Verifiable Presentation.
    ///
    /// This checks:
    /// 1. The holder's signature on the VP package is valid.
    /// 2. Every credential inside the VP has a valid issuer signature.
    /// 3. Every credential actually belongs to the holder (DID match).
    pub fn verify_presentation<T: Serialize + Clone>(vp: &VerifiablePresentation<T>) -> Result<()> {
        // 1. Verify the Holder's signature on the VP itself
        let proof = vp
            .proof
            .as_ref()
            .ok_or_else(|| anyhow!("Presentation has no proof"))?;
        let public_key = Self::resolve_did_key(&vp.holder)?;

        let mut vp_clone = vp.clone();
        vp_clone.proof = None;
        let json_data = serde_json::to_vec(&vp_clone)?;

        let (_base, sig_bytes) = multibase::decode(&proof.proof_value)
            .map_err(|_| anyhow!("Invalid VP proof encoding"))?;
        let signature = Signature::from_slice(&sig_bytes)
            .map_err(|_| anyhow!("Invalid VP signature format"))?;

        let verifying_key = VerifyingKey::from_bytes(&public_key)
            .map_err(|_| anyhow!("Invalid holder public key"))?;

        verifying_key
            .verify_strict(&json_data, &signature)
            .map_err(|_| {
                anyhow!("Holder signature verification failed! (Ownership proof failed)")
            })?;

        // 2. Verify each credential inside
        for vc in &vp.verifiable_credential {
            // Check Issuer Signature
            Self::verify(vc)?;

            // Check that the credential belongs to the holder
            if vc.credential_subject.id != vp.holder {
                return Err(anyhow!(
                    "Credential subject DID mismatch! This credential belongs to someone else."
                ));
            }
        }

        Ok(())
    }

    /// Verifies a Verifiable Credential.
    ///
    /// This checks:
    /// 1. The signature is mathematically valid.
    /// 2. The issuer is who they say they are (resolved from did:key).
    /// 3. The data has not been tampered with.
    pub fn verify<T: Serialize + Clone>(vc: &VerifiableCredential<T>) -> Result<()> {
        // 1. Ensure proof exists
        let proof = vc
            .proof
            .as_ref()
            .ok_or_else(|| anyhow!("Credential has no proof"))?;

        // 2. Resolve the Issuer's Public Key from their DID
        let public_key = Self::resolve_did_key(&vc.issuer)?;

        // 3. Prepare the data for verification (serialize without proof)
        let mut vc_clone = vc.clone();
        vc_clone.proof = None;
        let json_data = serde_json::to_vec(&vc_clone)?;

        // 4. Decode the signature
        let (_base, sig_bytes) =
            multibase::decode(&proof.proof_value).map_err(|_| anyhow!("Invalid proof encoding"))?;

        let signature =
            Signature::from_slice(&sig_bytes).map_err(|_| anyhow!("Invalid signature format"))?;

        // 5. Perform the cryptographic verification
        let verifying_key = VerifyingKey::from_bytes(&public_key)
            .map_err(|_| anyhow!("Invalid issuer public key"))?;

        verifying_key
            .verify_strict(&json_data, &signature)
            .map_err(|_| anyhow!("Cryptographic signature verification failed!"))?;

        Ok(())
    }

    /// Verifies an individual sub-claim from a selective disclosure credential.
    pub fn verify_sub_claim(
        holder_did: &str,
        issuer_did: &str,
        sub_claim: &SubClaim,
    ) -> Result<()> {
        // 1. Resolve Issuer's Public Key
        let public_key = Self::resolve_did_key(issuer_did)?;

        // 2. Reconstruct the data that was signed
        let sub_claim_data = json!({
            "holder": holder_did,
            "field": sub_claim.field,
            "value": sub_claim.value
        });
        let json_data = serde_json::to_vec(&sub_claim_data)?;

        // 3. Decode signature
        let (_base, sig_bytes) = multibase::decode(&sub_claim.proof.proof_value)
            .map_err(|_| anyhow!("Invalid sub-claim proof encoding"))?;
        let signature = Signature::from_slice(&sig_bytes)
            .map_err(|_| anyhow!("Invalid sub-claim signature format"))?;

        // 4. Verify
        let verifying_key = VerifyingKey::from_bytes(&public_key)
            .map_err(|_| anyhow!("Invalid issuer public key"))?;

        verifying_key
            .verify_strict(&json_data, &signature)
            .map_err(|_| anyhow!("Sub-claim signature verification failed!"))?;

        Ok(())
    }

    /// Resolves a did:key string into its raw 32-byte public key.
    fn resolve_did_key(did: &str) -> Result<[u8; 32]> {
        if !did.starts_with("did:key:z") {
            return Err(anyhow!("Unsupported DID method or encoding"));
        }

        let encoded_part = &did[9..];
        let (_base, bytes) = multibase::decode(format!("z{}", encoded_part))
            .map_err(|_| anyhow!("Failed to decode multibase DID"))?;

        // For Ed25519, the multicodec prefix is 0xed 0x01 (2 bytes)
        if bytes.len() != 34 || bytes[0] != 0xed || bytes[1] != 0x01 {
            return Err(anyhow!("Invalid Ed25519 DID key format"));
        }

        let mut pubkey = [0u8; 32];
        pubkey.copy_from_slice(&bytes[2..]);
        Ok(pubkey)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credential::issuer::Issuer;
    use crate::identity::IdentityKeypair;
    use crate::types::credential::NigerianCitizenCredential;

    #[test]
    fn test_successful_verification() {
        let nimc_key = IdentityKeypair::generate();
        let nimc = Issuer::new(nimc_key);

        let claims = NigerianCitizenCredential {
            name: "Amaka Okafor".to_string(),
            state_of_origin: "Anambra".to_string(),
        };

        let vc = nimc
            .issue(
                "did:key:holder".to_string(),
                claims,
                "NigerianCitizenCredential",
            )
            .unwrap();

        assert!(Verifier::verify(&vc).is_ok());
    }

    #[test]
    fn test_failed_verification_tampered_data() {
        let nimc_key = IdentityKeypair::generate();
        let nimc = Issuer::new(nimc_key);

        let claims = NigerianCitizenCredential {
            name: "Amaka Okafor".to_string(),
            state_of_origin: "Anambra".to_string(),
        };

        let mut vc = nimc
            .issue(
                "did:key:holder".to_string(),
                claims,
                "NigerianCitizenCredential",
            )
            .unwrap();

        // TAMPER: Change the name
        vc.credential_subject.claims.name = "Fake Name".to_string();

        assert!(
            Verifier::verify(&vc).is_err(),
            "Verification must fail for tampered data"
        );
    }
}
