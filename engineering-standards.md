# SSI Nigeria — Engineering Standards
### Ultra-Strict Requirements for National Infrastructure Development

---

> Building for a nation is different from building for a company. 
> A bug here is a violation of human rights. 
> These rules are not suggestions; they are the gatekeepers of the codebase.

---

## 1. The Zero-Tolerance Policy

- **ZERO `unsafe` code:** The use of the `unsafe` keyword is strictly prohibited. If a dependency uses `unsafe`, it must be audited or replaced with a pure-Rust, memory-safe alternative.
- **ZERO `panic!`, `unwrap()`, or `expect()`:** The application must never crash. All possible error paths must be represented by `Result<T, E>` and handled by the caller.
- **ZERO Warnings:** The build will fail if any linting warning exists. This includes `dead_code`, `unused_variables`, and style suggestions.

---

## 2. Mandatory Verification Suite

Every PR must pass the "Gauntlet" before being considered for review:

### A. Static Analysis (The Basics)
- `cargo fmt --all --check`
- `cargo check --workspace`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo audit` (Checks for known security vulnerabilities in dependencies)

### B. Dynamic Analysis (The "More")
- **Fuzz Testing:** Every parser and cryptographic input must be fuzzed using `cargo-fuzz`. If a fuzzer can crash the code in 24 hours, the code is rejected.
- **Miri Check:** The test suite must run under `cargo miri` to detect any undefined behavior or memory leaks.
- **Property-Based Testing:** Use `proptest` for all logic. Instead of testing one case, we test 10,000 random valid and invalid cases to ensure the math holds.
- **Code Coverage:** Minimum **95% branch coverage** required. No "happy path only" testing.

---

## 3. Cryptographic Discipline

- **Constant-Time Operations:** Any code that handles private keys or biometric hashes must be constant-time to prevent **Side-Channel Attacks** (timing attacks).
- **Secure Zeroization:** Private keys must be wiped from memory (`zeroize` crate) as soon as they are no longer needed. They must never be allowed to "linger" in RAM.
- **No Custom Crypto:** We use industry-standard crates (`ed25519-dalek`, `ssi`). We do not "roll our own" cryptographic primitives.

---

## 4. Documentation & Traceability

- **Requirement-to-Code Mapping:** Every non-trivial function must include a comment linking it to a requirement in the `.md` documentation. 
  - *Example: `// Implements: trust-and-authority.md#Section-4-Credential-Chain`*
- **Architecture Decision Records (ADR):** Any change to the system's core architecture must be documented in an ADR before code is written.

---

## 5. Dependency Management

- **The "Small Surface" Rule:** We minimize dependencies. Every new crate added to `Cargo.toml` must be justified by a security and maintenance audit.
- **Pinning:** All dependencies must be pinned to exact versions in `Cargo.toml` to prevent "dependency drift."

---

## 6. Failure Modes (Design for Disaster)

- **The "Offline-First" Rule:** The system must function correctly when nodes are disconnected. Verification must be possible without a live network check.
- **Graceful Degradation:** If the blockchain node in South Africa is down, the system must fail over to Lagos or Port Harcourt without the user noticing.

---

## 7. The "Judge" Rule

- **Auditability:** Every action that affects a citizen's state must be logged in a way that a non-technical "Judge" can understand during a legal audit. The code's output must be as clear as a legal document.

---

*Any developer contributing to this project agrees to these standards. Failure to adhere to them is a failure of the mission.*
