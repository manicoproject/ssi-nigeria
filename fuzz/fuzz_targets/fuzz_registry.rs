#![no_main]

use libfuzzer_sys::fuzz_target;
use ssi_nigeria::types::revocation::RevocationRegistry;
use ssi_nigeria::credential::RevocationManager;

fuzz_target!(|data: &[u8]| {
    // Fuzz the revocation registry verification logic
    if let Ok(registry) = serde_json::from_slice::<RevocationRegistry>(data) {
        // We need a DID string to pass. We can hardcode one for fuzzing the signature 
        // validation, or try to parse one from the data. We'll use the issuer_did from the registry itself.
        let did = registry.issuer_did.clone();
        let _ = RevocationManager::verify_registry(&registry, &did);
    }
});
