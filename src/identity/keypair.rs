use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A container for the citizen's cryptographic keys.
///
/// Implements: engineering-standards.md#Section-3-Cryptographic-Discipline
/// - Secure Zeroization: Keys are wiped from memory on drop.
#[derive(Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct IdentityKeypair {
    /// The 32-byte Ed25519 secret key.
    #[serde(with = "serde_bytes")]
    pub secret: [u8; 32],
    /// The 32-byte Ed25519 public key.
    #[serde(with = "serde_bytes")]
    pub public: [u8; 32],
}

impl std::fmt::Debug for IdentityKeypair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdentityKeypair")
            .field("public", &hex::encode(self.public))
            .field("secret", &"[REDACTED]")
            .finish()
    }
}

mod serde_bytes {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(bytes)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = Vec::deserialize(deserializer)?;
        let mut array = [0u8; 32];
        if bytes.len() == 32 {
            array.copy_from_slice(&bytes);
            Ok(array)
        } else {
            Err(serde::de::Error::custom("invalid byte length for key"))
        }
    }
}

impl IdentityKeypair {
    /// Generates a new random Ed25519 keypair.
    ///
    /// This uses OsRng which is cryptographically secure.
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = VerifyingKey::from(&signing_key);

        Self {
            secret: signing_key.to_bytes(),
            public: verifying_key.to_bytes(),
        }
    }

    /// Returns the public key as bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.public
    }

    /// Sign a message using the secret key
    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        let signing_key = SigningKey::from_bytes(&self.secret);
        signing_key.sign(message).to_bytes()
    }
}

mod hex {
    pub fn encode(bytes: [u8; 32]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let kp1 = IdentityKeypair::generate();
        let kp2 = IdentityKeypair::generate();

        assert_ne!(kp1.secret, kp2.secret, "Keys must be unique");
        assert_ne!(kp1.public, kp2.public, "Public keys must be unique");
    }

    #[test]
    fn test_signing_verification() {
        let kp = IdentityKeypair::generate();
        let message = b"Nigerian Citizen Auth 2025";
        let signature = kp.sign(message);

        let verifying_key = VerifyingKey::from_bytes(&kp.public).unwrap();
        let sig = ed25519_dalek::Signature::from_bytes(&signature);

        assert!(
            verifying_key.verify_strict(message, &sig).is_ok(),
            "Signature must be valid"
        );
    }

    #[test]
    fn test_debug_redaction() {
        let kp = IdentityKeypair::generate();
        let debug_str = format!("{:?}", kp);
        assert!(
            debug_str.contains("[REDACTED]"),
            "Secret must not be printed in debug"
        );
        assert!(
            !debug_str.contains(&hex::encode(kp.secret)),
            "Secret must not be printed in debug"
        );
    }
}
