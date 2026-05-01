use crate::identity::keypair::IdentityKeypair;
use multibase::{Base, encode};

/// Utility for generating and managing Decentralized Identifiers (DIDs).
pub struct DidKey;

impl DidKey {
    /// Generates a did:key string from an IdentityKeypair
    /// Format: did:key:z[Base58Btc(0xed01 + public_key)]
    ///
    /// Implements: identity-blockchain-nigeria.md#Step-1-Identity-Creation
    pub fn from_keypair(keypair: &IdentityKeypair) -> String {
        let public_key = keypair.public_key_bytes();

        // Multicodec prefix for Ed25519 public key is 0xed 0x01
        let mut bytes = Vec::with_capacity(34);
        bytes.push(0xed);
        bytes.push(0x01);
        bytes.extend_from_slice(&public_key);

        let encoded = encode(Base::Base58Btc, &bytes);
        format!("did:key:{}", encoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_did_generation_format() {
        let kp = IdentityKeypair::generate();
        let did = DidKey::from_keypair(&kp);

        assert!(
            did.starts_with("did:key:z"),
            "DID must follow did:key:z format"
        );
        // Ed25519 did:key is typically 50+ chars
        assert!(did.len() > 40, "DID string seems too short");
    }

    #[test]
    fn test_did_determinism() {
        let kp = IdentityKeypair::generate();
        let did1 = DidKey::from_keypair(&kp);
        let did2 = DidKey::from_keypair(&kp);

        assert_eq!(did1, did2, "Same keypair must produce same DID");
    }
}
