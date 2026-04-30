# SSI Nigeria — Project Blueprint
### Self-Sovereign Identity: From Concept to Working Rust Implementation

---

## What We Are Building

A working proof-of-concept of Self-Sovereign Identity (SSI) — demonstrating that a Nigerian citizen can hold a cryptographic identity they fully control, receive verifiable credentials from institutions, and present those credentials for verification — without any central database being queried.

This is not a toy. The architecture here is the same architecture that would run at national scale. We are building it small first, correctly, so it can grow.

---

## The Core Mental Model

Forget databases. Forget ID numbers. Think like this:

```
You = a keypair
      public key  → your address, visible to the world
      private key → your proof, never leaves your device
```

When an institution wants to verify you, they do not call a database.
They verify a mathematical signature. The math either checks out or it does not.
No middleman. No single point of failure.

---

## The Three Roles

Every SSI system has exactly three roles. Everything else is detail.

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│   ISSUER              HOLDER               VERIFIER            │
│                                                                 │
│   NIMC, CBN,    →    Nigerian      →    Bank, INEC,           │
│   University         Citizen             Border Control        │
│                                                                 │
│   Signs              Stores              Checks                │
│   credentials        credentials         the signature         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Issuer
- Has authority in a domain (government, bank, university)
- Issues signed credentials: "This person is a citizen", "This person has a degree"
- Signs with their own private key so the credential can be verified later

### Holder
- The citizen
- Generates their own keypair — this is their DID (Decentralized Identifier)
- Receives credentials from issuers and stores them
- Presents credentials to verifiers on demand
- Controls what they share and what they do not

### Verifier
- A third party that needs to confirm something about the holder
- Receives a credential or a proof
- Checks the issuer's signature mathematically
- Does NOT call the issuer. Does NOT query a database. Math is enough.

---

## What Actually Flows

### Step 1 — Identity Creation (Holder)

```
Holder generates keypair
        │
        ├── private key → stored securely on device, never transmitted
        │
        └── public key  → becomes the DID (Decentralized Identifier)
                          format: did:key:z6Mk...
                          this is your public identity address
```

A DID looks like this:
```
did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK
```
It is not a random number assigned by someone else.
It is mathematically derived from your public key.
It belongs to you because only you hold the private key that corresponds to it.

### Step 2 — Credential Issuance (Issuer → Holder)

The issuer creates a Verifiable Credential — a signed JSON document:

```json
{
  "@context": ["https://www.w3.org/2018/credentials/v1"],
  "type": ["VerifiableCredential", "NigerianCitizenCredential"],
  "issuer": "did:key:z6Mk...issuer_did...",
  "issuanceDate": "2025-01-01T00:00:00Z",
  "credentialSubject": {
    "id": "did:key:z6Mk...holder_did...",
    "citizenship": "Nigerian",
    "stateOfOrigin": "Lagos"
  },
  "proof": {
    "type": "Ed25519Signature2020",
    "verificationMethod": "did:key:z6Mk...issuer_did...#key-1",
    "proofValue": "z58DAdFfa..."
  }
}
```

The `proof` field is the issuer's cryptographic signature over the credential content.
The holder stores this credential. It is theirs.

### Step 3 — Verification (Holder → Verifier)

```
Holder presents credential to Verifier
        │
Verifier checks:
        │
        ├── Is the issuer's signature valid?         (cryptography)
        ├── Is the issuer in our trusted list?       (policy)
        ├── Has this credential been revoked?        (ledger check)
        └── Does the credential belong to this holder? (DID match)
        │
        └── Result: VALID or INVALID
```

No call to NIMC. No call to CBN. No database lookup.
The proof is self-contained.

---

## Selective Disclosure

One of the most powerful properties of SSI:

> You can prove a claim without revealing the data behind it.

Examples:
- Prove you are above 18 without revealing your date of birth
- Prove you are a Nigerian citizen without revealing your home address
- Prove you have a university degree without revealing your grades

This is called a **Zero-Knowledge Proof** at the advanced level.
At the basic level, it is simply presenting only the fields of a credential that are needed.

---

## The Technology Stack

### Language
**Rust** — exclusively.

### Core Crates

| Crate | Purpose | What it handles |
|---|---|---|
| `did-key` | DID generation | Creates DIDs from keypairs, supports Ed25519 |
| `ssi` | SSI core (by Spruce) | Verifiable Credentials issuance and verification |
| `ed25519-dalek` | Cryptographic signing | Fast, secure Ed25519 key operations |
| `serde` / `serde_json` | Serialization | JSON handling for credentials |
| `rand` | Randomness | Secure random number generation for keys |

### Cryptographic Primitives in Use

| Primitive | What it is | Why it matters |
|---|---|---|
| Ed25519 | Elliptic curve signature scheme | Fast, small keys, battle-tested |
| Public/Private Keypair | Asymmetric cryptography | Holder controls private, world sees public |
| Digital Signature | Signed hash of credential | Proves issuer signed without revealing private key |
| DID | Decentralized Identifier | Self-generated identity address |
| VC | Verifiable Credential | Signed claim about a subject |
| VP | Verifiable Presentation | Holder-wrapped proof sent to verifier |

---

## Project Structure

```
ssi-nigeria/
├── Cargo.toml
├── README.md
│
├── src/
│   ├── main.rs               # Entry point, demo flow
│   │
│   ├── identity/
│   │   ├── mod.rs
│   │   ├── keypair.rs        # Generate and manage keypairs
│   │   └── did.rs            # DID generation and resolution
│   │
│   ├── credential/
│   │   ├── mod.rs
│   │   ├── issuer.rs         # Issue and sign credentials
│   │   ├── holder.rs         # Store and present credentials
│   │   └── verifier.rs       # Verify credential signatures
│   │
│   ├── types/
│   │   ├── mod.rs
│   │   ├── credential.rs     # VerifiableCredential struct
│   │   └── presentation.rs   # VerifiablePresentation struct
│   │
│   └── errors.rs             # Error types
│
└── examples/
    ├── citizen_credential.rs  # Full Issuer → Holder → Verifier demo
    └── selective_disclosure.rs # Prove citizenship without revealing address
```

---

## The Build Order

### Phase 1 — Identity Foundation
- [ ] Generate Ed25519 keypair
- [ ] Derive DID from public key (`did:key` method)
- [ ] Serialize/deserialize DID documents
- [ ] Store keypair securely (encrypted file for now, secure enclave later)

**Milestone:** `cargo run` prints a valid DID generated from a fresh keypair.

### Phase 2 — Credential Issuance
- [ ] Define `VerifiableCredential` struct
- [ ] Implement issuer signing (Ed25519 signature over credential JSON)
- [ ] Build credential JSON in W3C VC format
- [ ] Holder receives and stores credential

**Milestone:** Issuer signs a "NigerianCitizenCredential" and holder stores it.

### Phase 3 — Verification
- [ ] Implement signature verification
- [ ] Resolve issuer DID from credential
- [ ] Check credential has not expired
- [ ] Return structured verification result

**Milestone:** Verifier receives credential and returns `Valid` or `Invalid` with reason.

### Phase 4 — Presentation Layer
- [ ] Holder wraps credential in a `VerifiablePresentation`
- [ ] Presentation includes holder's own signature (proves holder controls the DID)
- [ ] Verifier checks both issuer signature and holder signature

**Milestone:** Full three-party demo runs end to end from CLI.

### Phase 5 — Selective Disclosure
- [ ] Implement field-level disclosure (share only what is needed)
- [ ] Verifier requests specific claims
- [ ] Holder generates presentation with only those claims

**Milestone:** Verifier confirms citizenship without ever seeing the holder's address.

---

## What v0 Does NOT Include (yet)

These are real problems that will need real solutions later.
We are not ignoring them. We are sequencing correctly.

| Deferred | Why deferred | Future approach |
|---|---|---|
| Biometric binding | Requires hardware/device APIs | Secure enclave integration in v2 |
| Revocation registry | Requires a shared ledger | Blockchain or distributed log in v2 |
| DID resolution network | Requires P2P infrastructure | Iroh-based resolver in v3 |
| Institutional key management | Requires policy + HSMs | Separate issuer service in v2 |
| Recovery mechanisms | Requires social recovery design | Multi-sig guardians in v3 |

---

## The Nigerian Context — Why This Architecture Fits

| Nigerian Problem | SSI Solution |
|---|---|
| NIN, BVN, Voter Card are disconnected silos | One DID, multiple credentials attached to it |
| Different name spellings across databases | The DID is the identity, not the name |
| Ghost voters who cannot be deregistered | Dead people cannot cryptographically sign |
| No link between CCTV evidence and identity | Biometric → private key → DID → verifiable identity |
| Millions unbanked with no legacy documentation | Biometric enrollment creates a DID from scratch |
| Database breaches exposing millions of records | A breach of one credential does not compromise others |

---

## The Gap This Project Addresses

The gap in Nigeria is not technical capability.
The technology exists. The cryptography is proven. The standards are written.

The gap is a working demonstration, built by Nigerians, for the Nigerian context,
that shows what the architecture looks like when it runs.

This project is that demonstration.

---

## Standards and Specifications Referenced

- [W3C Decentralized Identifiers (DIDs) v1.0](https://www.w3.org/TR/did-core/)
- [W3C Verifiable Credentials Data Model v1.1](https://www.w3.org/TR/vc-data-model/)
- [DID Key Method](https://w3c-ccg.github.io/did-method-key/)
- [Ed25519Signature2020](https://w3c-ccg.github.io/lds-ed25519-2020/)

---

## First Command

```bash
cargo new ssi-nigeria
cd ssi-nigeria
```

That is where it starts.

---

*Blueprint authored for the ssi-nigeria project. Architecture: Self-Sovereign Identity on Ed25519/did:key/W3C VC stack. Language: Rust. Context: Nigerian identity infrastructure problem.*
