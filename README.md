# SSI Nigeria
### Self-Sovereign Identity: From Concept to Working Rust Implementation

🇳🇬 **SSI Nigeria** is a working proof-of-concept for a national decentralized identity infrastructure. It demonstrates how a Nigerian citizen can hold a cryptographic identity they fully control, receive verifiable credentials from institutions (NIMC, CBN, Universities), and present those credentials for verification—without any central database being queried.

---

## 🚀 The Vision

Traditional identity systems (NIN, BVN) rely on centralized databases that are silos, vulnerable to breaches, and difficult to reconcile. This project replaces the "database lookup" with **mathematical proof**.

- **You are a keypair:** Your private key lives on your device (or a physical card); your public key is your global address.
- **No middlemen:** Verification happens locally via cryptography.
- **Selective Disclosure:** Prove you are over 18 or a citizen without revealing your birth date or home address.

---

## 🛠 Project Status: Phase 1 Complete

We are currently in the **Foundational Identity** stage. 

- [x] **Phase 1: Identity Foundation** — Ed25519 keypair generation and `did:key` derivation.
- [ ] **Phase 2: Credential Issuance** — Defining and signing W3C Verifiable Credentials.
- [ ] **Phase 3: Verification** — Locally verifying issuer signatures without database calls.
- [ ] **Phase 4: Presentation** — Wrapping credentials in Verifiable Presentations.
- [ ] **Phase 5: Selective Disclosure** — Implementing zero-knowledge proofs for privacy.

---

## 📖 Documentation Index

We believe in documenting the "Why" and the "How" before the code.

| Document | Description |
|---|---|
| [Blueprint](ssi-nigeria-blueprint.md) | The architectural master plan and build order. |
| [Identity & Blockchain](identity-blockchain-nigeria.md) | Systems thinking on why Nigeria needs this architecture. |
| [System Flaws](system-flaws.md) | A honest threat model of every weakness (Coercion, Spoofing, etc.). |
| [Trust & Authority](trust-and-authority.md) | Solving the "Root of Trust" problem: Who authorizes the authorizers? |
| [Credential Hierarchy](credential-hierarchy.md) | Map of health, financial, and tax credentials and their issuers. |

---

## 💻 Tech Stack

- **Language:** [Rust](https://www.rust-lang.org/)
- **Cryptography:** `ed25519-dalek`
- **DID Standard:** `did:key`
- **SSI Core:** `ssi` (SpruceID)
- **Serialization:** `serde` / `serde_json`

---

## 🏃 Getting Started

### Prerequisites
- Install [Rust](https://rustup.rs/)

### Running the Demo
To see the identity foundation in action:

```bash
# Clone the repository
# cd ssi-nigeria

# Run the Phase 1 demo
cargo run
```

### Example Output
```text
🇳🇬 SSI Nigeria — Phase 1: Identity Foundation
-------------------------------------------
[✓] Generated Ed25519 Keypair
[✓] Derived DID: did:key:z6MkgW6TKbpheunYGxm995Wob9DL4JVJXTNBaCYtdcCCRjuP

Citizen Identity Summary:
  DID Address: did:key:z6MkgW6TKbpheunYGxm995Wob9DL4JVJXTNBaCYtdcCCRjuP
  Public Key:  0x1e714598c8ed931d0de622a0ef6e37d2768ccaf70e8331bc6a6f4a3d1a03e6d6
-------------------------------------------
Identity creation successful. This citizen is now ready to receive credentials.
```

---

## ⚖️ Governance & Principles

1. **The Citizen Is the Root:** Every actor (Police, Judge, President) is a citizen first.
2. **On-Chip Matching:** Biometrics never leave the hardware card.
3. **Audit Everything:** Every data access is logged on a distributed ledger.
4. **Self-Governing:** The system enforces its own rules through code, not human discretion.

---

*Built with ❤️ for the future of Nigerian digital infrastructure.*
