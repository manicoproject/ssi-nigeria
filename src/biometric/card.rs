use anyhow::{Result, anyhow};
use sha2::{Digest, Sha256};

use crate::identity::IdentityKeypair;

/// The maximum number of fingerprint enrollment templates stored per citizen.
const MAX_TEMPLATES: usize = 3;

/// Simulates a Hardware Secure Element (HSE) chip on a physical identity card.
///
/// In a real deployment, this logic runs INSIDE the card's chip — the private
/// key NEVER leaves the hardware. Biometric matching happens ON the card.
///
/// Implements: system-flaws.md#Section-5-Biometric-Spoofing
/// Implements: ssi-nigeria-blueprint.md#Section-Biometric-Binding
pub struct BiometricCard {
    /// The citizen's keypair — locked inside the chip.
    keypair: IdentityKeypair,
    /// Enrolled fingerprint hashes (templates stored on card, not a server).
    enrolled_templates: Vec<[u8; 32]>,
    /// Whether the card is currently unlocked for this session.
    is_unlocked: bool,
}

#[allow(dead_code)]
impl BiometricCard {
    /// Provisions a new card with a freshly generated keypair.
    ///
    /// This simulates the one-time enrollment at an NIMC office.
    /// The private key is generated INSIDE the chip and never transmitted.
    pub fn provision(keypair: IdentityKeypair) -> Self {
        Self {
            keypair,
            enrolled_templates: Vec::new(),
            is_unlocked: false,
        }
    }

    /// Enrolls a citizen's fingerprint onto the card.
    ///
    /// In hardware, the raw biometric never leaves the card.
    /// We store a one-way hash of the template here for simulation.
    ///
    /// Returns an error if the card is already at max capacity.
    pub fn enroll_fingerprint(&mut self, raw_biometric: &[u8]) -> Result<()> {
        if self.enrolled_templates.len() >= MAX_TEMPLATES {
            return Err(anyhow!(
                "Card is full. Maximum {} fingerprints enrolled.",
                MAX_TEMPLATES
            ));
        }
        let template_hash = Self::hash_biometric(raw_biometric);
        self.enrolled_templates.push(template_hash);
        Ok(())
    }

    /// Attempts to unlock the card by matching a live fingerprint scan.
    ///
    /// The card compares the incoming scan against stored templates.
    /// If a match is found, the card enters an "unlocked" state for one
    /// signing operation.
    ///
    /// Returns `true` if biometric matched, `false` if rejected.
    pub fn authenticate(&mut self, live_scan: &[u8]) -> bool {
        let scan_hash = Self::hash_biometric(live_scan);
        let matched = self.enrolled_templates.contains(&scan_hash);
        self.is_unlocked = matched;
        matched
    }

    /// Signs a payload — ONLY if the card has been unlocked by a valid biometric.
    ///
    /// After signing, the card immediately locks again (one-use unlock).
    /// This prevents replay attacks where a compromised unlock is reused.
    pub fn sign_with_biometric_gate(&mut self, payload: &[u8]) -> Result<[u8; 64]> {
        if !self.is_unlocked {
            return Err(anyhow!(
                "Card is locked. Biometric authentication required before signing."
            ));
        }

        // One-use unlock — lock immediately after signing
        self.is_unlocked = false;

        Ok(self.keypair.sign(payload))
    }

    /// Returns the card's public DID string.
    pub fn did(&self) -> String {
        crate::identity::did::DidKey::from_keypair(&self.keypair)
    }

    /// One-way hash of a biometric template.
    ///
    /// In hardware this would be a perceptual hash tuned for fingerprints
    /// (e.g., using minutiae extraction). We use SHA-256 as a simulation.
    fn hash_biometric(raw: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(raw);
        hasher.finalize().into()
    }
}

/// A session that proves a citizen has passed biometric authentication.
///
/// Carries the signed payload and DID for downstream verification.
#[derive(Debug)]
#[allow(dead_code)]
pub struct AuthenticatedSession {
    /// The citizen's DID (public, safe to share).
    pub citizen_did: String,
    /// The cryptographic signature produced by the card.
    pub signature: [u8; 64],
    /// The payload that was signed (e.g., a challenge nonce).
    pub signed_payload: Vec<u8>,
}

#[allow(dead_code)]
impl AuthenticatedSession {
    /// Creates a new authenticated session — wraps the card signing process.
    pub fn create(card: &mut BiometricCard, payload: Vec<u8>) -> Result<Self> {
        let signature = card.sign_with_biometric_gate(&payload)?;
        Ok(Self {
            citizen_did: card.did(),
            signature,
            signed_payload: payload,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::IdentityKeypair;

    // Simulated raw fingerprint bytes (in reality this comes from a sensor)
    const AMAKA_FINGERPRINT: &[u8] = b"amaka_right_index_minutiae_template_v1";
    const FAKE_FINGERPRINT: &[u8] = b"attacker_fingerprint_different_pattern";

    #[test]
    fn test_biometric_enrollment_and_authentication() {
        let kp = IdentityKeypair::generate();
        let mut card = BiometricCard::provision(kp);

        card.enroll_fingerprint(AMAKA_FINGERPRINT).unwrap();

        // Amaka places her finger — should unlock
        assert!(
            card.authenticate(AMAKA_FINGERPRINT),
            "Real fingerprint must authenticate"
        );
    }

    #[test]
    fn test_biometric_rejection_of_imposter() {
        let kp = IdentityKeypair::generate();
        let mut card = BiometricCard::provision(kp);

        card.enroll_fingerprint(AMAKA_FINGERPRINT).unwrap();

        // An imposter tries their fingerprint
        assert!(
            !card.authenticate(FAKE_FINGERPRINT),
            "Fake fingerprint must be rejected"
        );
    }

    #[test]
    fn test_sign_requires_biometric_unlock() {
        let kp = IdentityKeypair::generate();
        let mut card = BiometricCard::provision(kp);
        card.enroll_fingerprint(AMAKA_FINGERPRINT).unwrap();

        // Attempt to sign WITHOUT authenticating first
        let result = card.sign_with_biometric_gate(b"payload");
        assert!(
            result.is_err(),
            "Signing must fail without biometric unlock"
        );

        // Now authenticate and sign
        card.authenticate(AMAKA_FINGERPRINT);
        let result = card.sign_with_biometric_gate(b"payload");
        assert!(
            result.is_ok(),
            "Signing must succeed after biometric unlock"
        );
    }

    #[test]
    fn test_one_use_unlock() {
        let kp = IdentityKeypair::generate();
        let mut card = BiometricCard::provision(kp);
        card.enroll_fingerprint(AMAKA_FINGERPRINT).unwrap();

        // Authenticate once
        card.authenticate(AMAKA_FINGERPRINT);

        // Sign once — uses up the unlock
        card.sign_with_biometric_gate(b"first_payload").unwrap();

        // Try to sign again without re-authenticating — MUST FAIL
        let result = card.sign_with_biometric_gate(b"second_payload");
        assert!(
            result.is_err(),
            "Card must lock after one signing operation"
        );
    }

    #[test]
    fn test_enrollment_cap() {
        let kp = IdentityKeypair::generate();
        let mut card = BiometricCard::provision(kp);

        for i in 0..MAX_TEMPLATES {
            card.enroll_fingerprint(format!("finger_{}", i).as_bytes())
                .unwrap();
        }

        // Trying to enroll a 4th template must fail
        let result = card.enroll_fingerprint(b"one_too_many");
        assert!(result.is_err(), "Card must reject over-enrollment");
    }
}
