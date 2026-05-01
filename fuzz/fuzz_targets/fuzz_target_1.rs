#![no_main]

use libfuzzer_sys::fuzz_target;
use ssi_nigeria::types::credential::VerifiableCredential;
use ssi_nigeria::credential::OfflineVerifier;
use serde_json::Value;

fuzz_target!(|data: &[u8]| {
    // Attempt to parse the random bytes as a generic VerifiableCredential
    if let Ok(vc) = serde_json::from_slice::<VerifiableCredential<Value>>(data) {
        // If it parses successfully, pass it to the OfflineVerifier.
        // The goal of this fuzz target is to prove that NO combination of 
        // syntactically valid JSON can EVER cause a panic in the verifier.
        let _ = OfflineVerifier::verify_credential(&vc);
    }
});
