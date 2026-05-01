#![deny(unsafe_code)]
#![deny(warnings)]
#![deny(missing_docs)]

//! # SSI Nigeria
//!
//! National Identity Infrastructure built with Self-Sovereign Identity principles.

mod biometric;
mod credential;
mod governance;
mod identity;
mod tests;
mod types;

use credential::{Holder, Issuer, OfflineVerifier, RevocationManager, Verifier};
use identity::IdentityKeypair;
use serde_json::json;
use types::financial::FinancialSummary;

/// Main entry point — runs Phase 7 (Financial Audit) + Phase 8 (Offline Verification).
fn main() -> anyhow::Result<()> {
    // =====================================================================
    //  PHASE 7 — Financial Inclusion & Tax Audit
    // =====================================================================
    println!("SSI Nigeria — Phase 7: Financial Inclusion & Tax Audit");
    println!("-------------------------------------------");

    let central_bank = Issuer::new(IdentityKeypair::generate());
    let amaka = Holder::new(IdentityKeypair::generate());

    println!("[1] CBN-authorized bank issues Amaka an Anonymized Financial Summary.");

    let financial_data = FinancialSummary {
        annual_volume: 5_000_000,
        tax_cleared: true,
        currency: "NGN".to_string(),
    };

    let financial_vc = central_bank.issue(amaka.did.clone(), financial_data, "FinancialSummary")?;

    println!("[2] Amaka presents to FIRS. FIRS verifies — no personal data revealed.");
    let vp = amaka.create_presentation(vec![financial_vc])?;

    match Verifier::verify_presentation(&vp) {
        Ok(_) => {
            let claims = &vp.verifiable_credential[0].credential_subject.claims;
            println!("    [✓] BANK SIGNATURE VALID.");
            println!(
                "    [✓] TAX STATUS:  {}",
                if claims.tax_cleared {
                    "COMPLIANT"
                } else {
                    "NON-COMPLIANT"
                }
            );
            println!(
                "    [✓] VOLUME:      {} {}",
                claims.annual_volume, claims.currency
            );
            println!("    [✓] IDENTITY:    [ANONYMOUS — DID only]");
        }
        Err(e) => println!("    [✗] FAILED: {}", e),
    }

    // =====================================================================
    //  PHASE 8 — Offline Verification (No Network)
    // =====================================================================
    println!("\nSSI Nigeria — Phase 8: Offline Verification");
    println!("-------------------------------------------");
    println!("Scenario: A border checkpoint in a rural area with NO internet access.");
    println!("An officer scans a QR code from a citizen's phone/card.\n");

    let nimc = Issuer::new(IdentityKeypair::generate());
    let emeka = Holder::new(IdentityKeypair::generate());

    // NIMC issues Emeka a full selective credential (happens ONCE, online)
    println!("[1] NIMC previously issued Emeka a Selective Credential (done online).");
    let fields = vec![
        ("name", json!("Emeka Obi")),
        ("state", json!("Enugu")),
        ("over_18", json!(true)),
        ("voter_registered", json!(true)),
    ];
    let emeka_id = nimc.issue_selective(emeka.did.clone(), fields)?;

    // Emeka is now at a checkpoint — OFFLINE
    println!("[2] Emeka presents his phone at the checkpoint (OFFLINE).");

    // He only reveals the claims the officer needs
    let voter_claim = emeka_id
        .sub_claims
        .iter()
        .find(|c| c.field == "voter_registered")
        .expect("voter_registered sub-claim must exist");

    println!("[3] Officer is verifying 'voter_registered' sub-claim locally...");
    println!("    (No internet. No database. Pure cryptography.)");

    match OfflineVerifier::verify_sub_claim_offline(&emeka.did, &nimc.did, voter_claim) {
        Ok(_) => {
            println!("    [✓] OFFLINE VERIFICATION SUCCESS");
            println!(
                "    [✓] FIELD: {} = {}",
                voter_claim.field, voter_claim.value
            );
            println!("    [✓] ISSUER: NIMC (resolved from DID — no network call)");
        }
        Err(e) => println!("    [✗] VERIFICATION FAILED: {}", e),
    }

    // Test forgery detection offline too
    println!("\n[4] A forger tampers with Emeka's QR code data...");
    let mut tampered = voter_claim.clone();
    tampered.value = json!(false);

    match OfflineVerifier::verify_sub_claim_offline(&emeka.did, &nimc.did, &tampered) {
        Ok(_) => println!("    [!] ERROR: Accepted tampered data!"),
        Err(_) => println!("    [✓] FORGERY REJECTED: Tampered data detected offline."),
    }

    println!("-------------------------------------------");
    println!("Phase 8 complete. The system works in Maiduguri, Lagos, or anywhere in between.");

    // =====================================================================
    //  PHASE 9 — Biometric Binding (Hardware Secure Element Simulation)
    // =====================================================================
    println!("\nSSI Nigeria — Phase 9: Biometric Binding");
    println!("-------------------------------------------");
    println!("Scenario: Amaka's physical identity card has a chip.");
    println!("The chip holds her private key. It will NOT sign unless");
    println!("it detects her real fingerprint.\n");

    use biometric::card::BiometricCard;

    let amaka_key = crate::identity::IdentityKeypair::generate();
    let mut amaka_card = BiometricCard::provision(amaka_key);

    // Simulate NIMC enrollment — Amaka places finger on the card reader
    amaka_card
        .enroll_fingerprint(b"amaka_right_index_fingerprint")
        .unwrap();
    println!("[1] Amaka's fingerprint enrolled on her card at the NIMC office.");

    // Scenario A: Legitimate use — Amaka pays at a POS terminal
    println!("\n[2] Amaka taps her card at a Bank POS terminal.");
    println!("    Terminal challenges card with a nonce to prevent replay attacks.");
    let challenge_nonce = b"bank_pos_challenge_nonce_20250501";
    amaka_card.authenticate(b"amaka_right_index_fingerprint");
    match amaka_card.sign_with_biometric_gate(challenge_nonce) {
        Ok(sig) => {
            println!("    [✓] BIOMETRIC MATCH: Card signed the challenge.");
            println!("    [✓] SIGNATURE (first 8 bytes): {:02x?}", &sig[..8]);
            println!("    [✓] PAYMENT AUTHORIZED.");
        }
        Err(e) => println!("    [✗] FAILED: {}", e),
    }

    // Scenario B: Thief steals the card but not the fingerprint
    println!("\n[3] A thief steals Amaka's card and tries to use it.");
    let authenticated = amaka_card.authenticate(b"thief_different_fingerprint");
    if !authenticated {
        println!("    [✓] CARD REJECTED: Biometric mismatch. Card is still locked.");
        println!("    [✓] The thief CANNOT sign anything with this card.");
    }

    // Scenario C: One-use unlock enforcement
    println!("\n[4] Demonstrating one-use unlock security.");
    amaka_card.authenticate(b"amaka_right_index_fingerprint");
    amaka_card.sign_with_biometric_gate(b"first_op").unwrap();
    match amaka_card.sign_with_biometric_gate(b"second_op_without_reauth") {
        Ok(_) => println!("    [!] ERROR: Card signed twice on one unlock!"),
        Err(_) => println!("    [✓] SECOND SIGN REJECTED: Card auto-locked after first use."),
    }

    println!("-------------------------------------------");
    println!("Phase 9 complete. Private keys are bound to the physical body of the citizen.");

    // =====================================================================
    //  PHASE 10 — Credential Revocation (No Central Database)
    // =====================================================================
    println!("\nSSI Nigeria — Phase 10: Credential Revocation");
    println!("-------------------------------------------");
    println!("Scenario: Amaka's card is stolen. She reports it to NIMC.");
    println!("NIMC must invalidate her old credential WITHOUT a central DB.\n");

    let nimc_revoke = Issuer::new(IdentityKeypair::generate());
    let mut nimc_registry = RevocationManager::new(IdentityKeypair::generate());
    let citizen = Holder::new(IdentityKeypair::generate());

    let claims = types::credential::NigerianCitizenCredential {
        name: "Amaka Okafor".to_string(),
        state_of_origin: "Anambra".to_string(),
    };
    let amaka_vc = nimc_revoke.issue(citizen.did.clone(), claims, "NigerianCitizenCredential")?;

    println!("[1] NIMC issued Amaka a credential.");
    println!("    Checking revocation status before report...");
    let is_revoked = RevocationManager::is_revoked(&nimc_registry.registry, &amaka_vc)?;
    println!(
        "    [✓] Status: {}",
        if is_revoked { "REVOKED" } else { "VALID" }
    );

    println!("\n[2] Amaka reports her card stolen. NIMC revokes her credential.");
    nimc_registry.revoke(&amaka_vc)?;
    println!("    [✓] Credential added to Revocation Registry.");
    println!("    [✓] Registry re-signed by NIMC.");

    println!("\n[3] A Bank tries to verify Amaka's old (stolen) credential.");
    let is_revoked = RevocationManager::is_revoked(&nimc_registry.registry, &amaka_vc)?;
    println!(
        "    [{}] Revocation check: {}",
        if is_revoked { "✓" } else { "✗" },
        if is_revoked {
            "REVOKED — credential rejected"
        } else {
            "VALID"
        }
    );

    println!("\n[4] Verifying the registry itself hasn't been tampered with...");
    match RevocationManager::verify_registry(&nimc_registry.registry, &nimc_registry.did) {
        Ok(_) => println!("    [✓] Registry signature VALID — NIMC signed this list."),
        Err(e) => println!("    [✗] Registry compromised: {}", e),
    }

    println!("\n[5] Amaka re-enrolls with a new keypair at NIMC.");
    let amaka_new = Holder::new(IdentityKeypair::generate());
    println!("    [✓] New DID: {}", amaka_new.did);
    println!("    [✓] Old DID is forever revoked. New DID is clean.");

    println!("-------------------------------------------");
    println!(
        "Phase 10 complete. Revocation is decentralized, privacy-preserving, and tamper-proof."
    );

    Ok(())
}
