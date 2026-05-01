use anyhow::{Result, anyhow};
use chrono::Utc;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::identity::IdentityKeypair;
use crate::identity::did::DidKey;
use crate::types::credential::VerifiableCredential;
use crate::types::revocation::RevocationRegistry;

/// Manages the issuance, signing, and checking of credential revocations.
pub struct RevocationManager {
    /// The issuer's keypair used to sign the registry.
    keypair: IdentityKeypair,
    /// The issuer's DID.
    pub did: String,
    /// The current state of the registry.
    pub registry: RevocationRegistry,
}

#[allow(dead_code)]
impl RevocationManager {
    /// Creates a new RevocationManager for an issuer.
    pub fn new(keypair: IdentityKeypair) -> Self {
        let did = DidKey::from_keypair(&keypair);
        let registry = RevocationRegistry::new(did.clone(), Utc::now().to_rfc3339());
        Self {
            keypair,
            did,
            registry,
        }
    }

    /// Computes the unique ID for a credential — a hash of its signed content.
    ///
    /// This is privacy-preserving: the hash reveals nothing about the
    /// credential's contents.
    pub fn credential_id<T: Serialize + Clone>(vc: &VerifiableCredential<T>) -> Result<Vec<u8>> {
        // Strip proof so the ID is stable regardless of when it was signed
        let mut vc_clone = vc.clone();
        vc_clone.proof = None;
        let json_bytes = serde_json::to_vec(&vc_clone)?;

        let mut hasher = Sha256::new();
        hasher.update(&json_bytes);
        Ok(hasher.finalize().to_vec())
    }

    /// Revokes a credential by adding its ID to the registry and re-signing it.
    ///
    /// After this call, the registry is updated and re-signed. Any verifier
    /// holding a cached copy of this registry will reject the credential.
    pub fn revoke<T: Serialize + Clone>(&mut self, vc: &VerifiableCredential<T>) -> Result<()> {
        let credential_id = Self::credential_id(vc)?;

        if self.registry.revoked_ids.contains(&credential_id) {
            return Err(anyhow!("Credential is already revoked."));
        }

        self.registry.revoked_ids.push(credential_id);
        self.registry.updated_at = Utc::now().to_rfc3339();

        // Re-sign the updated registry
        let registry_bytes = serde_json::to_vec(&RevocationRegistry {
            signature: None,
            ..self.registry.clone()
        })?;
        let signature = self.keypair.sign(&registry_bytes);

        self.registry.signature = Some(signature.to_vec());
        Ok(())
    }

    /// Checks if a credential has been revoked.
    ///
    /// This works OFFLINE — the verifier has a local copy of the registry.
    /// No network call needed. The registry is periodically synced (like a CRL).
    pub fn is_revoked<T: Serialize + Clone>(
        registry: &RevocationRegistry,
        vc: &VerifiableCredential<T>,
    ) -> Result<bool> {
        let credential_id = Self::credential_id(vc)?;
        Ok(registry.revoked_ids.contains(&credential_id))
    }

    /// Verifies the integrity of a registry — checks the issuer signed it.
    pub fn verify_registry(registry: &RevocationRegistry, issuer_did: &str) -> Result<()> {
        let signature_bytes = registry
            .signature
            .as_ref()
            .ok_or_else(|| anyhow!("Registry has no signature"))?;

        // Resolve issuer public key from DID
        if !issuer_did.starts_with("did:key:z") {
            return Err(anyhow!("Unsupported DID method for revocation"));
        }
        let encoded = &issuer_did[9..];
        let (_base, bytes) = multibase::decode(format!("z{}", encoded))
            .map_err(|_| anyhow!("Failed to decode issuer DID"))?;

        if bytes.len() != 34 || bytes[0] != 0xed || bytes[1] != 0x01 {
            return Err(anyhow!("Invalid Ed25519 DID format in revocation check"));
        }

        let mut pubkey = [0u8; 32];
        pubkey.copy_from_slice(&bytes[2..]);

        // Reconstruct the unsigned registry bytes
        let unsigned = RevocationRegistry {
            signature: None,
            ..registry.clone()
        };
        let registry_bytes = serde_json::to_vec(&unsigned)?;

        let verifying_key = ed25519_dalek::VerifyingKey::from_bytes(&pubkey)
            .map_err(|_| anyhow!("Invalid public key in revocation check"))?;

        let sig_array: [u8; 64] = signature_bytes
            .as_slice()
            .try_into()
            .map_err(|_| anyhow!("Signature has wrong length"))?;

        let signature = ed25519_dalek::Signature::from_bytes(&sig_array);

        verifying_key
            .verify_strict(&registry_bytes, &signature)
            .map_err(|_| anyhow!("Registry signature is invalid — registry may be tampered"))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credential::Issuer;
    use crate::identity::IdentityKeypair;
    use crate::types::credential::NigerianCitizenCredential;

    fn make_vc() -> (
        Issuer,
        RevocationManager,
        VerifiableCredential<NigerianCitizenCredential>,
    ) {
        let key1 = IdentityKeypair::generate();
        let key2 = IdentityKeypair::generate();

        // Issuer and RevocationManager share the same identity
        // In production they would be the same institution (NIMC)
        let revocation_key = IdentityKeypair {
            secret: key1.secret,
            public: key1.public,
        };
        let issuer_key = IdentityKeypair {
            secret: key2.secret,
            public: key2.public,
        };

        let issuer = Issuer::new(issuer_key);
        let manager = RevocationManager::new(revocation_key);

        let claims = NigerianCitizenCredential {
            name: "Amaka Okafor".to_string(),
            state_of_origin: "Anambra".to_string(),
        };
        let vc = issuer
            .issue(
                "did:key:citizen".to_string(),
                claims,
                "NigerianCitizenCredential",
            )
            .unwrap();

        (issuer, manager, vc)
    }

    #[test]
    fn test_credential_not_revoked_initially() {
        let (_, manager, vc) = make_vc();
        let revoked = RevocationManager::is_revoked(&manager.registry, &vc).unwrap();
        assert!(!revoked, "Fresh credential must not be revoked");
    }

    #[test]
    fn test_revocation_marks_credential() {
        let (_, mut manager, vc) = make_vc();
        manager.revoke(&vc).unwrap();

        let revoked = RevocationManager::is_revoked(&manager.registry, &vc).unwrap();
        assert!(revoked, "Credential must be revoked after revoke() call");
    }

    #[test]
    fn test_double_revocation_is_error() {
        let (_, mut manager, vc) = make_vc();
        manager.revoke(&vc).unwrap();
        let result = manager.revoke(&vc);
        assert!(result.is_err(), "Revoking twice must return an error");
    }

    #[test]
    fn test_revocation_registry_signature_is_valid() {
        let (_, mut manager, vc) = make_vc();
        manager.revoke(&vc).unwrap();

        let result = RevocationManager::verify_registry(&manager.registry, &manager.did);
        assert!(
            result.is_ok(),
            "Registry signature must be valid: {:?}",
            result
        );
    }

    #[test]
    fn test_tampered_registry_fails_verification() {
        let (_, mut manager, vc) = make_vc();
        manager.revoke(&vc).unwrap();

        // Tamper: add a fake revocation entry
        let mut tampered_registry = manager.registry.clone();
        tampered_registry.revoked_ids.push(vec![0u8; 32]);

        let result = RevocationManager::verify_registry(&tampered_registry, &manager.did);
        assert!(result.is_err(), "Tampered registry must fail verification");
    }
}
