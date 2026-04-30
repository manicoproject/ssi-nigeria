# SSI Nigeria — Credential Hierarchy & Data Map
### Mapping the Digital Persona: From Citizenship to Health, Finance, and Taxation

---

> This document defines the "What" of the system. 
> It maps the specific credentials, the authorities that issue them, and the data they contain.
> Every entry here corresponds to a data structure we will eventually implement in Rust.

---

## 1. Foundational Credentials (The Root)

These are the credentials that make a person a "Citizen" in the eyes of the system. Without these, other credentials cannot be attached.

| Credential Name | Issuer | Core Data Fields | Purpose |
|---|---|---|---|
| **Nigerian Citizenship** | NIMC | Full Name, DOB, State of Origin, Biometric Hash | Proves personhood and nationality. |
| **Birth Certificate** | NPC | Place of Birth, Parents' Names, Timestamp | Verification of age and lineage. |
| **National Identity (NIN)**| NIMC | Unique NIN String, Verified Biometric Link | The primary anchor for all other credentials. |

---

## 2. Financial Credentials (Banking & Economy)

Instead of one bank "owning" your data, you hold these credentials and present them when needed.

| Credential Name | Issuer | Core Data Fields | Privacy Level |
|---|---|---|---|
| **Bank Account Proof** | Commercial Banks | Account No, Bank Name, Account Tier | High - Shared with Verifiers during KYC. |
| **Credit Score** | Credit Bureaus | Score Value, Debt-to-Income Ratio | Medium - Shared for loans. |
| **Financial Activity Summary**| CBN / Banks | Annual Transaction Volume, Asset Classes | **Restricted** - Read by Tax Authorities (FIRS). |
| **Tax Clearance (TCC)** | FIRS | Tax ID (TIN), Payment Status, Fiscal Year | Public/Verifier - Needed for government services. |

### The Tax Logic
- **Automated Assessment:** The Tax Authority (FIRS) has a specialized "Auditor Credential."
- **Read Access:** This allows them to query the "Financial Activity Summary" from citizens' wallets to calculate liability.
- **Transparency:** The system logs every time FIRS reads this data. The citizen receives a notification: *"FIRS accessed your 2025 financial summary for tax assessment."*

---

## 3. Health & Biometric Credentials

| Credential Name | Issuer | Core Data Fields | Purpose |
|---|---|---|---|
| **Medical Record** | Hospitals | Blood Group, Allergies, Chronic Conditions | Emergency access by doctors. |
| **Immunization Card** | NPHCDA | Vaccine Type, Date, Batch Number | Travel and school enrollment. |
| **Biometric Master** | NIMC / System | Encrypted Fingerprint/Iris templates | Only exists on the physical Card chip. |

---

## 4. Professional & Educational Credentials

| Credential Name | Issuer | Core Data Fields | Unlocks |
|---|---|---|---|
| **University Degree** | Universities | Course, Grade, Graduation Date | Job applications, higher study. |
| **Professional License** | NBA / MDCN | License No, Standing, Expiry | The power to act as a "Judge" or "Doctor." |
| **Police/Service ID** | NPF / Military | Rank, Division, Service Number | Authorization to request warrants or enforce laws. |

---

## 5. The Governance Hierarchy (Warrants & Access)

How authority is actually exercised through credentials.

### Level 1: The Citizen
- **Holds:** Basic Identity, Health, Finance.
- **Power:** Grants/Revokes access to their own data.

### Level 2: The Professional (Doctor/Banker)
- **Holds:** Professional License.
- **Power:** Can request access to specific slices of citizen data (with consent).

### Level 3: The Judicial Authority (Judge)
- **Holds:** Judicial Appointment + Bar Certificate.
- **Power:** Can sign **Warrants** (Digital Credentials) that override citizen consent for law enforcement.

### Level 4: The System Regulator (CBN/FIRS)
- **Holds:** Regulatory Mandate.
- **Power:** Automated audit of high-level financial patterns for national security and taxation.

---

## 6. Selective Disclosure Rules (The "Privacy Filter")

A citizen doesn't show the whole "book" of their life. They show "pages."

- **The Bar Scenario:** Citizen shows *over_18* flag, not *Date of Birth*.
- **The Loan Scenario:** Citizen shows *income_proof*, not *every individual transaction*.
- **The Hospital Scenario:** Citizen shows *allergies/blood_type*, not *entire psychiatric history*.

---

## 7. Data Lifecycle & Revocation

- **Expiration:** Credentials like "Driver's License" or "Judicial Appointment" have hard expiry dates. The system stops recognizing them automatically at midnight on that date.
- **Revocation:** If a bank account is closed for fraud, the bank revokes the "Bank Account Proof" on the blockchain. Any verifier checking it instantly sees: `Status: REVOKED`.
- **Death:** When a "Death Certificate" credential is issued by the NPC, the system automatically marks the root "Citizenship" credential as inactive. No further signatures can be generated.

---

*This document is the map. Our Rust implementation will follow this structure to build the structs and verification logic.*
