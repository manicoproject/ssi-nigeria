//! Property-based tests for SSI Nigeria.
//!
//! These tests prove mathematical invariants hold for ALL inputs,
//! not just hand-picked examples. Using proptest, each property
//! is verified against thousands of randomly generated cases.
//!
//! Implements: engineering-standards.md#Section-2B-Property-Based-Testing

use proptest::prelude::*;

use crate::biometric::card::BiometricCard;
use crate::credential::revocation::RevocationManager;
use crate::credential::{Issuer, OfflineVerifier};
use crate::identity::IdentityKeypair;
use crate::identity::did::DidKey;
use crate::types::credential::NigerianCitizenCredential;

// =========================================================================
//  PROPERTY 1: Signing Soundness
//  "Any message signed by a valid keypair can always be verified."
// =========================================================================
proptest! {
    #[test]
    fn prop_valid_signature_always_verifies(
        message in proptest::collection::vec(any::<u8>(), 1..1024)
    ) {
        let kp = IdentityKeypair::generate();
        let sig = kp.sign(&message);

        let verifying_key = ed25519_dalek::VerifyingKey::from_bytes(&kp.public).unwrap();
        let signature = ed25519_dalek::Signature::from_bytes(&sig);
        prop_assert!(
            verifying_key.verify_strict(&message, &signature).is_ok(),
            "A valid signature must always verify"
        );
    }
}

// =========================================================================
//  PROPERTY 2: Tamper Detection
//  "Any single-byte mutation of a signed credential is always detected."
// =========================================================================
proptest! {
    #[test]
    fn prop_tampered_credential_always_fails(
        name in "[A-Za-z]{4,30}",
        state in "[A-Za-z]{2,20}",
        // Use a byte guaranteed to differ from ASCII letters (0x00-0x1F are control chars)
        tamper_byte in 0u8..=31u8,
        tamper_position in 0usize..4usize,
    ) {
        let nimc = Issuer::new(IdentityKeypair::generate());
        let citizen_did = "did:key:z6Mktest".to_string();
        let original_name = name.clone();

        let claims = NigerianCitizenCredential {
            name: original_name.clone(),
            state_of_origin: state,
        };
        let mut vc = nimc.issue(citizen_did, claims, "NigerianCitizenCredential").unwrap();

        // Mutate the name with a non-ASCII byte — guaranteed to change the JSON payload
        let mut name_bytes = original_name.into_bytes();
        let pos = tamper_position % name_bytes.len();
        name_bytes[pos] = tamper_byte; // 0x00–0x1F can't appear in original ASCII name

        // The name bytes are now invalid UTF-8 or a different string
        // Use the raw bytes to directly mutate the JSON-serialized form
        // by mutating the credential subject's name string
        let tampered_name = String::from_utf8_lossy(&name_bytes).to_string();

        // Only assert if the string genuinely changed
        if tampered_name != vc.credential_subject.claims.name {
            vc.credential_subject.claims.name = tampered_name;
            let result = OfflineVerifier::verify_credential(&vc);
            prop_assert!(
                result.is_err(),
                "Tampered credential must always fail verification"
            );
        }
    }
}

// =========================================================================
//  PROPERTY 3: DID Determinism
//  "The same keypair always produces the same DID."
// =========================================================================
proptest! {
    #[test]
    fn prop_did_is_deterministic(_seed in any::<u64>()) {
        let kp = IdentityKeypair::generate();
        let did1 = DidKey::from_keypair(&kp);
        let did2 = DidKey::from_keypair(&kp);
        prop_assert_eq!(did1, did2, "Same keypair must always produce the same DID");
    }
}

// =========================================================================
//  PROPERTY 4: DID Uniqueness
//  "Two independently generated keypairs never produce the same DID."
//  (Collision resistance — relies on Ed25519 security)
// =========================================================================
proptest! {
    #[test]
    fn prop_did_uniqueness(_seed in any::<u64>()) {
        let kp1 = IdentityKeypair::generate();
        let kp2 = IdentityKeypair::generate();
        let did1 = DidKey::from_keypair(&kp1);
        let did2 = DidKey::from_keypair(&kp2);
        prop_assert_ne!(did1, did2, "Different keypairs must never produce the same DID");
    }
}

// =========================================================================
//  PROPERTY 5: Biometric Gate Soundness
//  "A fingerprint that was NOT enrolled never unlocks a card."
// =========================================================================
proptest! {
    #[test]
    fn prop_wrong_fingerprint_never_authenticates(
        enrolled in proptest::collection::vec(any::<u8>(), 10..64),
        attacker in proptest::collection::vec(any::<u8>(), 10..64),
    ) {
        // Only run when the two fingerprints are different
        prop_assume!(enrolled != attacker);

        let kp = IdentityKeypair::generate();
        let mut card = BiometricCard::provision(kp);
        card.enroll_fingerprint(&enrolled).unwrap();

        let authenticated = card.authenticate(&attacker);
        prop_assert!(
            !authenticated,
            "A different fingerprint must never authenticate"
        );
    }
}

// =========================================================================
//  PROPERTY 6: Biometric Gate Completeness
//  "The enrolled fingerprint always authenticates its own card."
// =========================================================================
proptest! {
    #[test]
    fn prop_enrolled_fingerprint_always_authenticates(
        fingerprint in proptest::collection::vec(any::<u8>(), 1..64),
    ) {
        let kp = IdentityKeypair::generate();
        let mut card = BiometricCard::provision(kp);
        card.enroll_fingerprint(&fingerprint).unwrap();

        let authenticated = card.authenticate(&fingerprint);
        prop_assert!(authenticated, "Enrolled fingerprint must always authenticate");
    }
}

// =========================================================================
//  PROPERTY 7: Revocation Completeness
//  "A revoked credential is ALWAYS detected by is_revoked()."
// =========================================================================
proptest! {
    #[test]
    fn prop_revoked_credential_always_detected(
        name in "[A-Za-z]{2,30}",
        state in "[A-Za-z]{2,20}",
    ) {
        let issuer = Issuer::new(IdentityKeypair::generate());
        let mut manager = RevocationManager::new(IdentityKeypair::generate());

        let claims = NigerianCitizenCredential {
            name,
            state_of_origin: state,
        };

        let vc = issuer
            .issue("did:key:citizen".to_string(), claims, "NigerianCitizenCredential")
            .unwrap();

        manager.revoke(&vc).unwrap();

        let is_revoked = RevocationManager::is_revoked(&manager.registry, &vc).unwrap();
        prop_assert!(is_revoked, "Revoked credential must always be detected");
    }
}

// =========================================================================
//  PROPERTY 8: Non-Revocation Soundness
//  "A credential that was NOT revoked is NEVER reported as revoked."
// =========================================================================
proptest! {
    #[test]
    fn prop_non_revoked_credential_never_detected(
        name in "[A-Za-z]{2,30}",
        state in "[A-Za-z]{2,20}",
    ) {
        let issuer = Issuer::new(IdentityKeypair::generate());
        let manager = RevocationManager::new(IdentityKeypair::generate());

        let claims = NigerianCitizenCredential {
            name,
            state_of_origin: state,
        };

        let vc = issuer
            .issue("did:key:citizen".to_string(), claims, "NigerianCitizenCredential")
            .unwrap();

        // NOTE: We do NOT revoke the credential
        let is_revoked = RevocationManager::is_revoked(&manager.registry, &vc).unwrap();
        prop_assert!(!is_revoked, "Non-revoked credential must never appear in registry");
    }
}
