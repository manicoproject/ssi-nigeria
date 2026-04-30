use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityKeypair {
    #[serde(with = "serde_bytes")]
    pub secret: [u8; 32],
    #[serde(with = "serde_bytes")]
    pub public: [u8; 32],
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
    /// Generates a new random Ed25519 keypair
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
