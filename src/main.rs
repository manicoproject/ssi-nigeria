mod identity;

use identity::IdentityKeypair;
use identity::did::DidKey;

fn main() {
    println!("🇳🇬 SSI Nigeria — Phase 1: Identity Foundation");
    println!("-------------------------------------------");

    // 1. Generate a new keypair (The citizen's secret)
    let citizen_key = IdentityKeypair::generate();
    println!("[✓] Generated Ed25519 Keypair");

    // 2. Derive the DID (The citizen's public identity)
    let did = DidKey::from_keypair(&citizen_key);
    println!("[✓] Derived DID: {}", did);

    println!("\nCitizen Identity Summary:");
    println!("  DID Address: {}", did);
    println!("  Public Key:  0x{}", hex::encode(citizen_key.public_key_bytes()));
    println!("-------------------------------------------");
    println!("Identity creation successful. This citizen is now ready to receive credentials.");
}

mod hex {
    pub fn encode(bytes: [u8; 32]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}
