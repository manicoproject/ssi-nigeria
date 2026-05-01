use anyhow::{Result, anyhow};
use ed25519_dalek::{Signature, VerifyingKey};
use serde::Serialize;

use crate::types::credential::VerifiableCredential;
use crate::types::selective::SubClaim;

/// A verifier that operates with ZERO network calls.
///
/// This is the core guarantee of the system. All verification logic
/// is self-contained, reading public keys directly from the DID string.
///
/// This is critical for Nigeria's geography: a border agent in a remote
/// area must be able to verify a citizen's credential with no internet.
///
/// Implements: system-flaws.md#Section-3-Network-Availability
pub struct OfflineVerifier;

#[allow(dead_code)]
impl OfflineVerifier {
    /// Verifies a standard Verifiable Credential — no network required.
    ///
    /// The issuer's public key is resolved directly from their `did:key` string.
    /// This function makes zero I/O calls and is deterministic.
    pub fn verify_credential<T: Serialize + Clone>(vc: &VerifiableCredential<T>) -> Result<()> {
        let proof = vc
            .proof
            .as_ref()
            .ok_or_else(|| anyhow!("[OFFLINE] Credential has no proof"))?;

        // Resolve public key from the DID — this is LOCAL, no network needed
        let public_key = Self::resolve_did_key_local(&vc.issuer)?;

        // Reconstruct the signed payload (proof stripped off)
        let mut vc_clone = vc.clone();
        vc_clone.proof = None;
        let json_data = serde_json::to_vec(&vc_clone)?;

        // Decode the signature
        let (_base, sig_bytes) = multibase::decode(&proof.proof_value)
            .map_err(|_| anyhow!("[OFFLINE] Invalid proof encoding"))?;
        let signature = Signature::from_slice(&sig_bytes)
            .map_err(|_| anyhow!("[OFFLINE] Invalid signature format"))?;

        // Verify — pure math, no I/O
        let verifying_key = VerifyingKey::from_bytes(&public_key)
            .map_err(|_| anyhow!("[OFFLINE] Invalid public key in DID"))?;

        verifying_key
            .verify_strict(&json_data, &signature)
            .map_err(|_| anyhow!("[OFFLINE] Signature invalid — credential may be forged"))?;

        Ok(())
    }

    /// Verifies a single sub-claim from a Selective Disclosure credential.
    ///
    /// Used when a citizen presents only one fact (e.g., "over_18") from
    /// their full credential, such as scanning a QR code at a checkpoint.
    pub fn verify_sub_claim_offline(
        holder_did: &str,
        issuer_did: &str,
        sub_claim: &SubClaim,
    ) -> Result<()> {
        let public_key = Self::resolve_did_key_local(issuer_did)?;

        // Reconstruct the canonical signed data
        let sub_claim_data = serde_json::json!({
            "holder": holder_did,
            "field": sub_claim.field,
            "value": sub_claim.value
        });
        let json_data = serde_json::to_vec(&sub_claim_data)?;

        let (_base, sig_bytes) = multibase::decode(&sub_claim.proof.proof_value)
            .map_err(|_| anyhow!("[OFFLINE] Invalid sub-claim proof encoding"))?;
        let signature = Signature::from_slice(&sig_bytes)
            .map_err(|_| anyhow!("[OFFLINE] Invalid sub-claim signature"))?;

        let verifying_key = VerifyingKey::from_bytes(&public_key)
            .map_err(|_| anyhow!("[OFFLINE] Invalid issuer public key"))?;

        verifying_key
            .verify_strict(&json_data, &signature)
            .map_err(|_| anyhow!("[OFFLINE] Sub-claim signature is invalid"))?;

        Ok(())
    }

    /// Resolves a `did:key` into raw Ed25519 public key bytes — pure local operation.
    ///
    /// This is the key insight: `did:key` encodes the public key IN the DID string.
    /// No DNS, no blockchain query, no HTTP call needed. Just string decoding.
    fn resolve_did_key_local(did: &str) -> Result<[u8; 32]> {
        if !did.starts_with("did:key:z") {
            return Err(anyhow!(
                "[OFFLINE] Unsupported DID method: only did:key is supported offline"
            ));
        }

        let encoded_part = &did[9..];
        let (_base, bytes) = multibase::decode(format!("z{}", encoded_part))
            .map_err(|_| anyhow!("[OFFLINE] Failed to decode DID key material"))?;

        if bytes.len() != 34 || bytes[0] != 0xed || bytes[1] != 0x01 {
            return Err(anyhow!(
                "[OFFLINE] Invalid Ed25519 multicodec prefix in DID"
            ));
        }

        let mut pubkey = [0u8; 32];
        pubkey.copy_from_slice(&bytes[2..]);
        Ok(pubkey)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credential::Issuer;
    use crate::identity::IdentityKeypair;
    use crate::types::credential::NigerianCitizenCredential;
    use serde_json::json;

    /// Simulates a rural checkpoint with NO internet access.
    /// The officer scans a QR code and verifies the credential locally.
    #[test]
    fn test_offline_credential_verification() {
        let nimc_key = IdentityKeypair::generate();
        let nimc = Issuer::new(nimc_key);

        let citizen_did = "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string();
        let claims = NigerianCitizenCredential {
            name: "Emeka Obi".to_string(),
            state_of_origin: "Enugu".to_string(),
        };

        let vc = nimc
            .issue(citizen_did, claims, "NigerianCitizenCredential")
            .unwrap();

        // This call does ZERO network I/O
        let result = OfflineVerifier::verify_credential(&vc);
        assert!(
            result.is_ok(),
            "Offline verification must succeed: {:?}",
            result
        );
    }

    #[test]
    fn test_offline_sub_claim_verification() {
        let nimc_key = IdentityKeypair::generate();
        let nimc = Issuer::new(nimc_key);
        let holder_did = "did:key:z6Mktest".to_string();

        let selective_cred = nimc
            .issue_selective(
                holder_did.clone(),
                vec![("over_18", json!(true)), ("name", json!("Emeka Obi"))],
            )
            .unwrap();

        // Only disclose over_18 sub-claim — offline
        let sub_claim = selective_cred
            .sub_claims
            .iter()
            .find(|c| c.field == "over_18")
            .unwrap();

        let result = OfflineVerifier::verify_sub_claim_offline(&holder_did, &nimc.did, sub_claim);
        assert!(
            result.is_ok(),
            "Offline sub-claim verification must succeed: {:?}",
            result
        );
    }

    #[test]
    fn test_offline_detects_forgery() {
        let nimc = Issuer::new(IdentityKeypair::generate());
        let citizen_did = "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string();

        let claims = NigerianCitizenCredential {
            name: "Emeka Obi".to_string(),
            state_of_origin: "Enugu".to_string(),
        };
        let mut vc = nimc
            .issue(citizen_did, claims, "NigerianCitizenCredential")
            .unwrap();

        // A forger alters the data after signing
        vc.credential_subject.claims.state_of_origin = "Lagos".to_string();

        let result = OfflineVerifier::verify_credential(&vc);
        assert!(result.is_err(), "Offline verifier must detect forgery");
    }
}
