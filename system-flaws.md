# SSI Nigeria — System Flaws & Threat Model
### A Honest Analysis of Every Weakness in the Card + Reader + Blockchain Architecture

---

> This document exists because we believe in building things honestly.
> Every system has flaws. The ones you document are the ones you can design around.
> The ones you ignore are the ones that will break you in production — or worse, in a national crisis.

---

## Table of Contents

1. [Physical Threats](#1-physical-threats)
2. [Biometric Threats](#2-biometric-threats)
3. [Hardware Threats](#3-hardware-threats)
4. [Blockchain & Protocol Threats](#4-blockchain--protocol-threats)
5. [Recovery & Loss Threats](#5-recovery--loss-threats)
6. [Privacy & Surveillance Threats](#6-privacy--surveillance-threats)
7. [Institutional & Human Threats](#7-institutional--human-threats)
8. [Nigerian-Specific Context Threats](#8-nigerian-specific-context-threats)
9. [Lessons From ATM & BVN](#9-lessons-from-atm--bvn)
10. [Proposed Mitigations](#10-proposed-mitigations)

---

## 1. Physical Threats

### 1.1 Coercion — "The Gun to the Head"
**Severity: CRITICAL**

This is the most fundamental flaw. No cryptographic system can stop physical violence.

- An attacker forces the legitimate cardholder to place their finger on the reader.
- The system sees a valid fingerprint. The system sees a valid card. It approves everything.
- The blockchain records it as a legitimate, authorized transaction.
- There is no technical signal that distinguishes a "willing" fingerprint from a "forced" one.

**Why ATM has the same problem:**
ATMs solve this with withdrawal limits, velocity checks, and in some countries, a "duress PIN" — a different PIN that authorizes a small withdrawal while silently alerting authorities. The damage is *capped and flagged*, not prevented.

**Our equivalent:**
The **duress fingerprint** — a specific finger (e.g., right thumb = normal, left thumb = duress mode). When the duress finger is used, the system:
- Appears to work normally to the attacker
- Silently sends an alert to law enforcement with GPS coordinates
- May authorize a limited, capped action (e.g., show a fake "low balance" credential)

**Status: Unresolved. Must be designed into the hardware and protocol from the start.**

---

### 1.2 Card Theft Without Biometric Bypass
**Severity: MEDIUM**

If someone steals your card but cannot spoof your fingerprint, they cannot use your identity. The biometric binding solves most of the "stolen card" problem.

**Residual risk:** If the fingerprint sensor is low quality, a lifted fingerprint (from a glass, for example) could potentially fool it.

---

### 1.3 Physical Card Destruction
**Severity: LOW-MEDIUM**

A card can be cut, burned, or deliberately destroyed by the holder's abuser (e.g., domestic violence, human trafficking scenarios — common in Nigeria).

- The victim now has no way to prove identity.
- If the private key is *only* on the destroyed card, the identity may be unrecoverable.

**Mitigation:** Seed phrase backup. But this introduces its own risks (see Section 5).

---

## 2. Biometric Threats

### 2.1 Fingerprint Spoofing
**Severity: HIGH**

Cheap fingerprint sensors can be fooled by:
- High-resolution photographs of a fingerprint
- "Gummy fingers" — molds made from silicone, gelatin, or even wood glue
- Lifted prints from surfaces the victim touched

**The Nigerian context makes this worse:** Fingerprints are enrolled in BVN, NIN, SIM registration, and multiple other databases. If any of those systems are breached, an attacker may already have a copy of your fingerprint template.

**Mitigation:** Liveness detection (the sensor must detect a living, warm, pulsing finger — not a mold). Multi-factor biometrics (fingerprint + iris + facial geometry). Hardware-level secure elements that do biometric matching *inside* the chip.

---

### 2.2 Fingerprint Changes Over Time
**Severity: LOW-MEDIUM**

Fingerprints can change due to:
- Manual labor (worn ridges — very common in Nigeria's working population)
- Burns, injuries, skin conditions
- Age (ridges flatten over decades)

A citizen enrolled at age 20 may have a failing match rate by age 60.

**Mitigation:** Re-enrollment pathways. Multi-biometric backup (iris or facial geometry as fallback).

---

### 2.3 Biometric Data Storage Risk
**Severity: CRITICAL**

If raw biometric templates are stored on a central server (even the blockchain), and that server is breached:
- You cannot change your fingerprint. Ever.
- Unlike a password, a compromised biometric is permanently compromised.

**The correct architecture:** Raw biometrics must **never** leave the card chip. Biometric matching must happen *on the card*, not on the reader or a server. The card only ever outputs "match" or "no match" — never the biometric data itself.

**Status: This must be a non-negotiable hardware requirement.**

---

## 3. Hardware Threats

### 3.1 Supply Chain Attack — The Backdoor Chip
**Severity: CRITICAL**

Who manufactures the chips? Who manufactures the readers?

If any component in the supply chain is compromised — by a foreign government, a corrupt procurement process, or a malicious manufacturer — the entire system is compromised before a single card is issued.

- A backdoored chip could extract private keys on demand.
- A backdoored reader could log all fingerprint scans.
- A backdoored firmware update could silently hollow out the security of every card in the country.

**The Nigerian sovereignty problem:** If the hardware is manufactured outside Nigeria, the country's identity infrastructure is dependent on the trustworthiness of foreign manufacturers and foreign governments. This is not hypothetical — similar attacks have happened to national infrastructure globally.

**Mitigation:** Open hardware specifications. Publicly auditable firmware. Multiple competing manufacturers with independent security audits. No single point of trust in the supply chain.

---

### 3.2 Malicious Reader — The Skimmer
**Severity: HIGH**

When you slide your card into a reader, how do you know the reader is honest?

A malicious reader could:
- **Replay attack:** Capture the signed presentation and replay it elsewhere before it expires.
- **Biometric harvest:** Log your fingerprint scan for later use in spoofing.
- **Man-in-the-middle:** Alter the data being signed before passing it to the card, so you sign something different from what you intended.

**Mitigation:** 
- Every reader must be cryptographically authenticated to the cardholder's card before the card responds.
- Short-lived, single-use signed challenges (nonces) that expire in seconds.
- Visual confirmation on the card itself (a tiny display) showing exactly what is being signed.

---

### 3.3 Reader Tampering
**Severity: MEDIUM**

Physical readers in public locations (banks, border posts, polling units) can be tampered with by insiders or attackers who have brief physical access.

**Mitigation:** Tamper-evident seals. Remote attestation — the reader must prove to the central system that its firmware has not been modified before any transaction is processed.

---

## 4. Blockchain & Protocol Threats

### 4.1 Who Controls the Blockchain?
**Severity: CRITICAL**

A government-controlled blockchain recreates the exact centralization problem SSI was designed to solve. If the government can rewrite the ledger, they can:
- Erase someone's identity
- Alter credential records
- Selectively disenfranchise voters by invalidating their keys

**The tension:** Total decentralization (no government control) creates governance problems. Who resolves disputes? Who handles legitimate revocation of a criminal's credentials?

**There is no clean answer here. This is a political and governance question as much as a technical one.**

---

### 4.2 Key Compromise — "I Am Now You"
**Severity: CRITICAL**

If an attacker ever extracts your private key from the card chip (through a hardware attack, a side-channel attack, or a supply chain backdoor), they become you — completely and permanently — until a revocation is registered on the blockchain.

- They can sign credentials as you.
- They can vote as you.
- They can open bank accounts as you.
- The blockchain cannot distinguish your signature from theirs.

**Mitigation:** Hardware secure elements that physically destroy the key under tampering. Revocation mechanisms that allow a legitimate holder to invalidate a compromised key. Fast revocation propagation across all verifiers.

---

### 4.3 Blockchain Network Availability
**Severity: MEDIUM**

If the blockchain network goes down — due to infrastructure failure, cyberattack, or power outages (extremely common in Nigeria) — and a verifier requires a live ledger check:
- Transactions fail.
- Citizens cannot prove identity.
- Hospitals, borders, polling units grind to a halt.

**Mitigation:** Design the core verification to work *offline* using cached credential proofs. Only revocation checks and new credential issuance require live network access. The system must be designed for Nigeria's actual infrastructure reality, not an assumed always-on internet.

---

### 4.4 Quantum Computing (Long-Term Threat)
**Severity: LOW (now) → HIGH (future)**

Ed25519, the cryptographic algorithm at the core of this system, is currently secure. However, a sufficiently powerful quantum computer could theoretically break it.

- This is not an immediate threat.
- But credentials issued today may still be in use in 20–30 years, by which time quantum computing may be mature.

**Mitigation:** Plan for cryptographic agility — the ability to upgrade the signing algorithm without re-enrolling every citizen. The card hardware must support algorithm upgrades via secure firmware.

---

## 5. Recovery & Loss Threats

### 5.1 Lost Card — Identity Recovery
**Severity: CRITICAL**

In a pure SSI system with no central database, if the card is lost and the seed phrase is also lost, the identity may be unrecoverable. The citizen is cryptographically dead.

This is unacceptable for a national identity system serving a country of 220+ million people, many of whom are not digitally literate.

**Mitigation options:**
- **Seed phrase backup** — given to the citizen at enrollment, like a crypto wallet. Problem: most people will lose it.
- **Institutional recovery** — NIMC can vouch for re-enrollment after biometric re-verification. Problem: this recreates central authority.
- **Social recovery** — a set of trusted "guardians" (family members, community leaders) can collectively authorize a key reset. Each guardian holds a "shard" of a recovery key. Problem: requires careful design to prevent collusion.

**There is no simple answer. Recovery is the hardest problem in SSI.**

---

### 5.2 Seed Phrase Insecurity
**Severity: HIGH**

The seed phrase (master backup) is typically 12–24 words written on a piece of paper. For most Nigerian citizens:
- They will not understand what it is or why it matters.
- They may photograph it (exposing it digitally).
- They may store it next to the card (defeating its purpose as a backup).
- They may share it with a family member who misuses it.

**Mitigation:** Institutional custody of encrypted seed phrase shards. Social recovery model. Education at enrollment.

---

## 6. Privacy & Surveillance Threats

### 6.1 Identity Tracking
**Severity: HIGH**

If every use of the card generates a traceable on-chain event, the blockchain becomes a **complete surveillance log** of every citizen's:
- Movements (border crossings, location of readers used)
- Transactions
- Interactions with institutions
- Voting behavior (if not properly anonymized)

This is not a hypothetical concern — it is an explicit design risk in blockchain-based identity systems that have been deployed globally.

**Mitigation:** Zero-knowledge proofs for verification (prove you are a citizen without revealing which citizen). Unlinkable presentations — each verification event must look different on the ledger, making it impossible to correlate two events to the same person without the cardholder's consent.

---

### 6.2 Authoritarian Weaponization
**Severity: CRITICAL**

A cryptographic identity system linked to all government services is also, by definition, a tool for total state control. A government that can:
- Revoke a credential → can exclude a citizen from banking, voting, travel
- Read the audit trail → can reconstruct the full movement history of a political opponent
- Control the reader network → can make certain identities "invisible" to systems

SSI was designed to give power to the individual. But the same infrastructure, without the right governance model, gives more power to the state than any ID system has ever given before.

**This is not a technical problem. It is a constitutional, legal, and political one. It must be addressed before technical implementation, not after.**

---

## 7. Institutional & Human Threats

### 7.1 Corrupt Issuance at Enrollment
**Severity: HIGH**

If a corrupt NIMC official issues a credential to someone under a false identity, that credential is cryptographically valid. The blockchain will confirm it as authentic — because it *is* authentic, signed by a legitimate issuer key.

The blockchain guarantees *authenticity of issuance*, not *accuracy of claims*.

**Mitigation:** Audit trails for every issuance event. Multi-party issuance for high-value credentials (requires two independent officials to sign). Separation of enrollment (biometric capture) from issuance (credential signing) to prevent a single corrupt actor from doing both.

---

### 7.2 Insider Threat — Compromised Issuer Key
**Severity: CRITICAL**

Every institution that can issue credentials (NIMC, CBN, INEC) has an institutional private key. If that key is stolen or misused by an insider:
- They can issue unlimited fraudulent credentials.
- Every credential issued under that key is now suspect.
- The entire chain of trust for that institution must be revoked and rebuilt.

**Mitigation:** Hardware Security Modules (HSMs) for institutional keys. Multi-signature issuance requirements. Key rotation schedules. Real-time issuance auditing.

---

## 8. Nigerian-Specific Context Threats

### 8.1 Digital Literacy Gap
**Severity: HIGH**

A significant portion of Nigeria's population — particularly in rural and northern states — has limited digital literacy. A system that requires understanding of:
- Seed phrases
- Cryptographic keys
- Credential presentation flows

...will fail for this population, or worse, create a class of citizens who are technically enrolled but practically unable to use the system without intermediaries — who may exploit them.

**Mitigation:** Enrollment agents (community-level trusted actors). Assisted access pathways. The card must work through physical action alone (slide + fingerprint) with zero digital literacy required for basic use.

---

### 8.2 Power Infrastructure
**Severity: MEDIUM**

Nigeria's electricity grid is unreliable. Card readers and blockchain nodes that depend on continuous power will fail regularly.

**Mitigation:** Offline-capable verification. Battery-powered readers. Local node caching with sync-on-connect.

---

### 8.3 Network Infrastructure
**Severity: MEDIUM**

Rural Nigeria has limited internet connectivity. A system that requires network access for every verification will exclude the populations it most needs to serve.

**Mitigation:** As above — design for offline-first. Network only required for new issuance and revocation checks.

---

### 8.4 Existing Fragmented Databases
**Severity: HIGH**

Nigeria already has BVN, NIN, Voter Card, Driver's License, and Passport databases with conflicting data across systems. A new SSI system must either:
- Integrate with all of them (enormously complex, politically fraught)
- Run parallel to them (creates another silo — the exact problem we are solving)
- Replace them (requires political will that may not exist)

**There is no clean technical path here. This is an institutional coordination problem.**

---

## 9. Lessons From ATM & BVN

### What ATM Got Right (and How We Can Learn)
| ATM Defense | SSI Equivalent |
|---|---|
| Daily withdrawal limits | Credential-level transaction caps |
| Velocity checks (unusual behavior flags) | On-chain anomaly detection |
| Card + PIN (two factors) | Card + Fingerprint (two factors, stronger) |
| Duress PIN → silent alert | Duress fingerprint → silent alert |
| Reversal windows | Revocation registry for immediate invalidation |

### What BVN Got Right (and Where It Stops)
| BVN Strength | BVN Weakness | Our Improvement |
|---|---|---|
| Biometric binding | Centralized database | Decentralized blockchain |
| Cross-bank visibility | Only covers banking | Cross-sector: health, voting, borders |
| Single identity per person | Breach exposes everything | Selective disclosure hides what isn't needed |
| Fraud correlation across banks | No other government systems connected | One DID, all credentials attached |

**BVN proved the core concept works in Nigeria at scale. SSI is BVN, evolved: decentralized, cross-sector, and citizen-controlled.**

---

## 10. Proposed Mitigations (Summary)

| Flaw | Proposed Mitigation | Status |
|---|---|---|
| Coercion | Duress fingerprint + silent alert protocol | To be designed |
| Fingerprint spoofing | Liveness detection, multi-biometric | Hardware requirement |
| Biometric data exposure | On-chip matching only, never export raw biometrics | Non-negotiable architecture rule |
| Supply chain backdoor | Open hardware spec, independent audits, multiple vendors | Governance requirement |
| Malicious reader | Mutual authentication, nonce-based challenges, card-side display | Protocol requirement |
| Lost card | Social recovery (guardian shards) + institutional re-enrollment | To be designed |
| Surveillance/tracking | Zero-knowledge proofs, unlinkable presentations | Protocol requirement |
| Authoritarian weaponization | Constitutional protections, independent blockchain governance | Political/legal requirement |
| Corrupt issuance | Multi-party signing, audit trails, separated enrollment/issuance | Institutional requirement |
| Digital literacy gap | Physical-only UX (slide + finger), enrollment agents | UX requirement |
| Power/network gaps | Offline-first design, battery readers, local caching | Infrastructure requirement |
| Quantum threat | Cryptographic agility, algorithm upgrade path | Long-term requirement |

---

## Closing Note

> A system that acknowledges its flaws is a system that can be improved.
> A system that pretends to have none is a system waiting to fail at the worst possible moment.
>
> Every flaw in this document is a design requirement in disguise.

---

*Document authored during architectural design phase of ssi-nigeria. Updated as new threat vectors are identified. This is a living document.*
