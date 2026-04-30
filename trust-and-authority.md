# SSI Nigeria — Trust, Authority, and the Root of Trust Problem
### Who Authorizes the Authorizer? How a Self-Governing System Answers Its Own Questions.

---

> The hardest question in any identity system is not technical.
> It is philosophical:
> **"Who decides who gets to decide?"**
> This document is our answer.

---

## Table of Contents

1. [The Root of Trust Problem](#1-the-root-of-trust-problem)
2. [The Infinite Regress of Authority](#2-the-infinite-regress-of-authority)
3. [The Elegant Solution — Everyone Is a Citizen First](#3-the-elegant-solution--everyone-is-a-citizen-first)
4. [The Credential Chain — How Authority Is Proven](#4-the-credential-chain--how-authority-is-proven)
5. [Face Recognition and the Citizen Lookup Question](#5-face-recognition-and-the-citizen-lookup-question)
6. [The Foundational Design Principle](#6-the-foundational-design-principle)
7. [What This Means for Recovery](#7-what-this-means-for-recovery)
8. [Open Questions](#8-open-questions)

---

## 1. The Root of Trust Problem

When we designed the access control model — police need a warrant, warrant must be signed by a judge — a critical flaw surfaced immediately:

> **Who authorized the judge to sign the warrant?**

And whoever authorized the judge — who authorized *them*?

This is called the **Root of Trust Problem.** It exists in every security system ever built. It is not unique to SSI. Every government, every military, every bank faces it. The question is not whether the problem exists — it does. The question is: **where does the chain of authority stop, and who controls that stopping point?**

In our system, this is not a flaw to be patched. It is the central design question. Our answer is what makes this system different.

---

## 2. The Infinite Regress of Authority

Traced out, the problem looks like this:

```
A police officer wants to access a citizen's profile
    → They need a warrant
    → The warrant must be signed by a Judge
    → But who made this person a Judge?
    → The National Judicial Council (NJC) appointed them
    → But who authorized the NJC?
    → The Constitution
    → But who signed the Constitution?
    → The founding institutions of the Nigerian state
    → But who authorized them?
    → The people of Nigeria
    → But how does the system verify "the people of Nigeria" agreed?
    → ...
```

In traditional systems, this chain bottoms out at **raw political power** — whoever controls the army, controls the database, controls the answer. That is exactly the problem we are building away from.

In our system, this chain bottoms out at something different: **cryptographic consensus across distributed nodes, anchored in the citizen as the atomic unit of authority.**

The citizen is the root. Everything else is a credential on top of that root.

---

## 3. The Elegant Solution — Everyone Is a Citizen First

This is the architectural insight that resolves the regress:

> **A Judge is just a citizen with extra credentials.**
> **A Police Officer is just a citizen with extra credentials.**
> **A President is just a citizen with extra credentials.**
> **A Node Operator is just a citizen with extra credentials.**

There is no separate "authority class" in this system. Every actor — from the most ordinary citizen to the Chief Justice — starts from the same root: a cryptographic identity, generated from their biometrics, anchored on the blockchain.

What makes them different is not who they are. It is **what credentials they hold** — and those credentials are:
- Verifiable (mathematically provable)
- Auditable (every issuance is on the blockchain)
- Revocable (when a judge retires, resigns, or is removed, their credential is revoked)
- Time-limited (credentials can carry expiry dates — a serving government credential is not permanent)

This is not abstract. It changes everything:

| Old System | This System |
|---|---|
| A minister has power because they hold an office | A minister has power because they hold a verifiable, time-limited credential |
| Remove a corrupt official → hard, political, slow | Revoke a corrupt official's credential → immediate, cryptographic, automatic |
| Power survives the office (informal networks) | Power dies with the credential (no credential = no access) |
| Dead people can remain in the system indefinitely | A dead official's credential is revoked. Their access ends. |

---

## 4. The Credential Chain — How Authority Is Proven

When a judge signs a warrant, they are not just signing with their name. They are signing with their **full credential chain.** The system can verify every link:

```
Judge Fatima signs a warrant
    │
    ├── She is a Nigerian Citizen                        ✓ (NIMC credential)
    ├── She holds a Law Degree                           ✓ (University of Lagos credential)
    ├── She is a called to the bar                       ✓ (NBA credential)
    ├── She holds a Judicial Appointment                 ✓ (NJC credential)
    ├── Her appointment has not been revoked             ✓ (Revocation registry checked)
    ├── Her appointment has not expired                  ✓ (Expiry date checked)
    └── She has jurisdiction over this type of warrant   ✓ (Scope of credential checked)

Result: WARRANT IS VALID. System grants the access.
```

No human needed to confirm any of this. The math did it.

And critically — if Judge Fatima is removed from office tomorrow, her NJC credential is revoked. From that moment, any warrant she signs fails the verification. Automatically. Without a phone call. Without a memo. Without anyone having to update a spreadsheet.

**The system does not trust people. It trusts credentials. And credentials can always be verified.**

---

## 5. Face Recognition and the Citizen Lookup Question

When a crime is committed and captured on CCTV, the natural question is: can the police run the face against the national identity database?

This is where the hardest tension in the system lives — between **public safety** and **civil liberties.** Both are legitimate. Neither cancels the other.

### The Wrong Design

```
Police upload face → System searches ALL citizens → Returns a name
```

This is mass surveillance. Every citizen who ever walked past any camera becomes a suspect. The system treats the entire population as a database to be queried at will. This is unacceptable.

### The Right Design

```
Police upload face → System checks ONLY against:
    - People with active arrest warrants
    - People with existing criminal records
    - People who have been flagged by court order
    → Returns: match or no match (not a name, until authorized)
```

The distinction is fundamental:

| Wrong Approach | Right Approach |
|---|---|
| "Who is this person?" | "Is this person already known to law enforcement?" |
| Searches all citizens | Searches only flagged individuals |
| Everyone is a suspect | Only those already in the criminal system are searchable |
| No audit trail needed | Every search is logged on the blockchain |
| Police self-authorize | Requires a valid warrant to unlock the matched name |

Even when a match is found, the name is not immediately revealed. The system returns: **"Match found. Obtain a warrant to unlock the identity."** The police still need judicial authorization to see who it is. The face recognition narrows the field. The warrant unlocks the name.

### The Audit Trail

Every face recognition query is a permanently logged event on the blockchain:
- Who ran the query (which officer, with which credential)
- When it was run
- What footage was submitted
- What result was returned
- Whether a warrant was subsequently obtained

A citizen, upon request, can see if their face has ever been run through the system. This is not optional — it is a constitutional right encoded into the protocol.

---

## 6. The Foundational Design Principle

> *"If there is a way to design it for any possibilities, those questions will answer themselves as they come up."*

This is the most important architectural decision in this document. And it has a name:

**Design Principles Over Design Rules.**

A system built on rules tries to anticipate every possible scenario and write a rule for it. It will always fail — because the world generates scenarios faster than rules can be written.

A system built on principles defines a small set of foundational truths, and every new question is answered by applying those truths. The principles handle the answer. The system enforces it.

### Our Foundational Principles

**1. The Citizen Is the Root**
Every actor in the system — police, judge, president, node operator — is a citizen first. No one is above the citizen layer. Authority is a credential on top of citizenship, not a replacement for it.

**2. Every Action Requires a Verifiable Credential Chain**
No one can act in this system without a credential chain that can be fully verified. "I am a police officer" is not enough. The system must be able to verify: you are a citizen, you hold a police credential, that credential was legitimately issued, it has not been revoked, and it grants you the specific permission you are trying to exercise.

**3. Every Action Is Logged and Auditable**
Every access, every query, every credential issuance, every revocation is a permanent, timestamped event on the blockchain. Nothing is invisible. Nothing is deniable. The citizen can always see who accessed their data, when, and under what authorization.

**4. The Citizen Always Consents or Is Informed**
For any access to a citizen's data, one of two things must be true: either the citizen has explicitly granted access (for a doctor, a bank, a service provider), or the citizen is notified that a court-authorized access has occurred. The only exception is an active criminal investigation with a valid warrant — and even then, the access is logged and the citizen is informed after the investigation concludes.

**5. No Single Actor Has Unilateral Power**
No government agency, no node operator, no institution can unilaterally alter, revoke, or access a citizen's identity record. All significant actions require consensus across distributed nodes. All sensitive actions require a verifiable credential chain. All actions are audited.

**6. The System Enforces Its Own Rules**
The rules are not enforced by humans — they are enforced by code on the blockchain. A human can be bribed. A human can be intimidated. A human can be replaced by a corrupt actor. Code running on a distributed network cannot. The rules are the same for everyone, every time, without exception.

---

## 7. What This Means for Recovery

The distributed node architecture and the biometric enrollment solve the recovery problem in an elegant way that does not require seed phrases or central authority:

```
A citizen loses their card
    │
    ├── They go to any enrollment center
    ├── Their fingerprints are re-captured
    ├── The biometric proof is checked against ALL nodes:
    │       Amazon node (South Africa)  → matches ✓
    │       Lagos node                  → matches ✓
    │       Port Harcourt node          → matches ✓
    │       CBN node                    → matches ✓
    ├── Supermajority of nodes confirms: same person
    ├── Citizen cryptographically consents to key rotation
    ├── Old key is revoked across all nodes
    └── New card is issued with new key

No seed phrase. No central authority. No single point of trust.
Your finger is your recovery mechanism.
```

The biometric enrolled at the beginning is the same biometric that recovers you at the end. The chain remembers. The nodes agree. The math confirms.

---

## 8. Open Questions

These questions do not yet have answers. They are documented here because they will need to be answered before implementation — not by us alone, but by the governance structure the system is built within.

**Q: Who are the initial node operators?**
The first nodes must be established by someone. That bootstrapping moment is a centralization risk. How is it handled?

**Q: Who can issue institutional credentials?**
Who issues the "Judicial Appointment" credential to a new judge? What if that issuing body is corrupt? What checks exist on the issuers themselves?

**Q: How are constitutional amendments handled?**
If the rules of the system are encoded in the blockchain, and the Constitution changes, how does the system update? Who can propose and ratify protocol changes?

**Q: What is the minimum consensus threshold?**
For a given action, how many nodes must agree? 51%? 67%? 75%? The threshold affects both security and efficiency. Higher thresholds are more secure but slower.

**Q: How are node operators held accountable?**
A node operator who goes offline, becomes corrupt, or is compromised affects the entire network. What are the consequences? Who enforces them?

**Q: When does public safety override individual privacy?**
The face recognition question is one instance of a broader tension. Where exactly is the line? Who draws it? And how does the system enforce it consistently?

---

> These questions are limitless. That is not a weakness — it is the nature of building infrastructure for a nation.
> The right foundation does not answer every question in advance.
> It creates the framework within which every question can be answered honestly, consistently, and without corruption.

---

*Document authored during architectural design phase of ssi-nigeria.
Topic: Trust hierarchies, authority chains, access control philosophy, and foundational design principles.
This is a living document — updated as the design evolves.*
