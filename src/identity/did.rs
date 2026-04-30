use crate::identity::keypair::IdentityKeypair;
use multibase::{encode, Base};

pub struct DidKey;

impl DidKey {
    /// Generates a did:key string from an IdentityKeypair
    /// Format: did:key:z[Base58Btc(0xed01 + public_key)]
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
