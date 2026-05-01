# SSI Nigeria
### Self-Sovereign Identity: From Concept to Production-Grade Rust Infrastructure

🇳🇬 **SSI Nigeria** is a working, high-assurance proof-of-concept for a national decentralized identity infrastructure. It demonstrates how a Nigerian citizen can hold a cryptographic identity they fully control, receive verifiable credentials from institutions (NIMC, CBN, Universities), and present those credentials for verification—without any central database being queried.

---

## 🚀 The Vision

Traditional identity systems (NIN, BVN) rely on centralized databases that are silos, vulnerable to breaches, and difficult to reconcile. This project replaces the "database lookup" with **mathematical proof**.

- **You are a keypair:** Your private key lives on your device (or a physical hardware card); your public key is your global address.
- **No middlemen:** Verification happens locally via cryptography, capable of functioning entirely offline.
- **Selective Disclosure:** Prove you are over 18 or a citizen without revealing your birth date or home address.
- **Hardware Bound:** Keys are locked behind physical biometric confirmation.

---

## 🛠 Project Status: Phase 12 Complete

We have successfully completed all core engineering phases, resulting in a robust, mathematically proven system.

- [x] **Phase 1: Identity Foundation** — Ed25519 keypair generation and `did:key` derivation.
- [x] **Phase 2: Credential Issuance** — Defining and signing W3C-style Verifiable Credentials.
- [x] **Phase 3: Verification** — Locally verifying issuer signatures without database calls.
- [x] **Phase 4: Presentation** — Wrapping credentials in Verifiable Presentations to prove ownership.
- [x] **Phase 5: Selective Disclosure** — Implementing privacy-preserving sub-claim extraction.
- [x] **Phase 6: Governance (Warrants)** — Multi-sig judicial logic allowing Police to access data only with a Judge's cryptographic signature.
- [x] **Phase 7: Financial Inclusion** — Privacy-preserving FIRS tax audits and bank integrations.
- [x] **Phase 8: Offline Verification** — Rural checkpoint verification using zero internet architecture.
- [x] **Phase 9: Biometric Binding** — Simulating hardware secure elements with one-use fingerprint gating.
- [x] **Phase 10: Revocation** — Decentralized, privacy-preserving credential invalidation registries.
- [x] **Phase 11: Property-Based Testing** — Proving mathematical invariants against thousands of random inputs.
- [x] **Phase 12: High-Assurance Fuzzing** — `cargo-fuzz` integration proving zero panics against malicious memory corruption.

---

## 📖 Architecture & Design Documentation

We believe in documenting the "Why" and the "How" before the code.

| Document | Description |
|---|---|
| [Blueprint](ssi-nigeria-blueprint.md) | The architectural master plan and build order. |
| [Identity & Blockchain](identity-blockchain-nigeria.md) | Systems thinking on why Nigeria needs this architecture. |
| [System Flaws](system-flaws.md) | A honest threat model of every weakness (Coercion, Spoofing, etc.). |
| [Trust & Authority](trust-and-authority.md) | Solving the "Root of Trust" problem: Who authorizes the authorizers? |
| [Credential Hierarchy](credential-hierarchy.md) | Map of health, financial, and tax credentials and their issuers. |
| [Engineering Standards](engineering-standards.md) | The strict `#![deny(unsafe_code)]` and zero-panic policies enforcing national security. |
| [Custom Ledger Architecture](custom-ledger-architecture.md) | Why we use Iroh and Proof-of-Authority instead of Ethereum/Blockchain. |

---

## 💻 Tech Stack & Engineering Standards

This project enforces strict, aerospace-grade Rust constraints suitable for national infrastructure:
- `#![deny(unsafe_code)]`
- `#![deny(warnings)]`
- Property-based testing via `proptest`
- Continuous Fuzzing via `libFuzzer`

**Core Dependencies:**
- `ed25519-dalek` (Signatures)
- `sha2` (Hashing and Biometric templates)
- `serde` / `serde_json` (Data Serialization)

---

## 🏃 Getting Started

### Prerequisites
- Install [Rust](https://rustup.rs/)

### Running the Full Demo Gauntlet
To see the entire infrastructure pipeline run in sequence (Phases 1-10):

```bash
cargo run
```

### Running the Property-Based Proofs
To execute the mathematical invariants proving the system cannot be forged or tampered with:

```bash
cargo test
```

### Running the Fuzzers (Requires Nightly)
To throw millions of mutated byte arrays at the verifiers to prove they cannot panic:

```bash
rustup toolchain install nightly
cargo +nightly fuzz run fuzz_target_1
cargo +nightly fuzz run fuzz_registry
```

---

## ⚖️ Governance & Principles

1. **The Citizen Is the Root:** Every actor (Police, Judge, President) is a citizen first.
2. **On-Chip Matching:** Biometrics never leave the hardware card.
3. **Decentralized Transport:** We use custom P2P synchronization (Iroh) instead of public blockchains, eliminating gas fees while retaining cryptographic immutability.
4. **Self-Governing:** The system enforces its own rules through code, not human discretion.

---

*Built with ❤️ for the future of Nigerian digital infrastructure.*
